use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use derivative::Derivative;
use iced::{Command, Element, Rectangle, Subscription};
use thiserror::Error;
use vst::{host::PluginInstance, plugin::Plugin};

use audio_garbage_collector::Shared;
use plugin_host_lib::{
    audio_io::audio_io_service::storage::StorageConfig,
    audio_io::{AudioHostPluginLoadError, AudioIOService},
    processors::running_rms_processor::RunningRMSProcessorHandle,
    processors::volume_meter_processor::VolumeMeterProcessorHandle,
    TestPluginHost,
};

use crate::services::host_options_service::{HostOptionsService, HostState};
use crate::services::plugin_file_watch::FileWatcher;
use crate::ui::audio_io_settings;
use crate::ui::main_content_view::audio_file_chart::AudioFileModel;
use crate::ui::main_content_view::status_bar::StatusBar;
use crate::ui::main_content_view::transport_controls::TransportControlsView;
use crate::ui::plugin_editor_window::{close_window, open_plugin_window, PluginWindowHandle};

mod audio_chart;
mod audio_file_chart;
pub mod plugin_content;
pub mod status_bar;
pub mod transport_controls;
mod view;
pub mod volume_meter;

enum ClosePluginWindowResult {
    NoWindow,
    ClosedPlugin { window_frame: Rectangle },
}

#[derive(Debug, Error)]
enum ReloadPluginError {
    #[error("Failed to join tokio blocking thread")]
    Join(#[from] tokio::task::JoinError),
    #[error(transparent)]
    PluginLoad(#[from] AudioHostPluginLoadError),
    #[error("No plugin loaded, configure the plugin path")]
    MissingHost,
}

pub struct MainContentView {
    plugin_host: Arc<Mutex<TestPluginHost>>,
    audio_io_settings: audio_io_settings::Controller,
    host_options_service: HostOptionsService,
    plugin_content: plugin_content::View,
    transport_controls: TransportControlsView,
    volume_meter_state: volume_meter::VolumeMeter,
    error: Option<Box<dyn std::error::Error>>,
    editor_is_floating: bool,
    plugin_window_handle: Option<PluginWindowHandle>,
    host_state: HostState,
    status_message: StatusBar,
    // This should not be optional & it might break if the host restarts processors for some reason
    volume_handle: Option<Shared<VolumeMeterProcessorHandle>>,
    rms_processor_handle: Option<Shared<RunningRMSProcessorHandle>>,
    audio_chart: Option<audio_chart::AudioChart>,
    /// Cached window frame from a previous editor open
    previous_plugin_window_frame: Option<Rectangle>,
    audio_file_model: audio_file_chart::AudioFileModel,
    start_stop_button_state: view::StartStopViewModel,
}

#[derive(Derivative)]
#[derivative(Debug, Clone)]
pub enum Message {
    AudioIOSettings(audio_io_settings::Message),
    PluginContent(plugin_content::Message),
    TransportControls(transport_controls::Message),
    SetStatus(StatusBar),
    ReadyForPlayback,
    ReloadedPlugin(bool, StatusBar),
    VolumeMeter(volume_meter::Message),
    StartStopButtonClicked,
    Exit,
    None,
}

impl MainContentView {
    pub fn new(plugin_host: TestPluginHost) -> (Self, Command<Message>) {
        let plugin_host = Arc::new(Mutex::new(plugin_host));
        let (
            HostContext {
                audio_io_service,
                host_options_service,
                host_state,
            },
            command,
        ) = reload_plugin_host_state(plugin_host.clone());

        let plugin_content = plugin_content::View::new(
            host_state.audio_input_file_path.clone(),
            host_state.plugin_path.clone(),
        );
        let audio_io_settings = audio_io_settings::Controller::new(audio_io_service);

        (
            MainContentView {
                plugin_host,
                audio_io_settings,
                host_options_service,
                host_state,
                plugin_content,
                transport_controls: TransportControlsView::new(),
                error: None,
                editor_is_floating: false,
                plugin_window_handle: None,
                status_message: StatusBar::new("Starting audio thread", status_bar::State::Warning),
                volume_handle: None,
                rms_processor_handle: None,
                audio_chart: None,
                previous_plugin_window_frame: None,
                audio_file_model: AudioFileModel::empty(),
                volume_meter_state: volume_meter::VolumeMeter::new(),
                start_stop_button_state: view::StartStopViewModel {
                    is_started: true,
                    button_state: Default::default(),
                },
            },
            command,
        )
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        self.poll_for_host_handles();
        if let Some(chart) = &mut self.audio_chart {
            chart.update();
        }
        self.volume_meter_state
            .set_volume_info((&self.volume_handle).into());
        match message {
            Message::AudioIOSettings(msg) => self
                .audio_io_settings
                .update(msg)
                .map(Message::AudioIOSettings),
            Message::PluginContent(msg) => self.update_plugin_content(msg),
            Message::TransportControls(message) => self.update_transport_controls(message),
            Message::SetStatus(message) => self.update_status_message(message),
            Message::ReadyForPlayback => {
                Command::perform(iced_futures::futures::future::ready(()), |_| {
                    Message::SetStatus(StatusBar::new(
                        "Ready for playback",
                        status_bar::State::Idle,
                    ))
                })
            }
            Message::None => Command::none(),
            Message::ReloadedPlugin(did_close, status_bar) => {
                self.reset_handles();
                self.status_message = status_bar;
                if did_close {
                    self.open_plugin_window()
                } else {
                    Command::none()
                }
            }
            Message::StartStopButtonClicked => {
                self.start_stop_button_state.is_started = !self.start_stop_button_state.is_started;
                let plugin_host = self.plugin_host.clone();
                let should_start = self.start_stop_button_state.is_started;
                Command::perform(
                    async move {
                        let mut plugin_host = plugin_host.lock().unwrap();
                        if should_start {
                            if let Err(err) = plugin_host.start() {
                                log::error!("Error starting host: {}", err);
                            }
                        } else {
                            plugin_host.stop();
                        }
                    },
                    |_| Message::None,
                )
            }
            Message::VolumeMeter(message) => {
                volume_meter::update(message, self.plugin_host.clone()).map(Message::VolumeMeter)
            }
            Message::Exit => {
                let _ = self.close_plugin_window();
                Command::none()
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if let Some(path) = &self.host_state.plugin_path {
            Subscription::from_recipe(FileWatcher::new(path.as_ref()))
                .map(|_| Message::PluginContent(plugin_content::Message::ReloadPlugin))
        } else {
            Subscription::none()
        }
    }

    fn poll_for_host_handles(&mut self) {
        if self.volume_handle.is_none() {
            if let Ok(Some(volume_handle)) = self.plugin_host.try_lock().map(|h| h.volume_handle())
            {
                self.volume_handle = Some(volume_handle);
            }
        }
        if self.rms_processor_handle.is_none() {
            if let Ok(Some(buffer)) = self
                .plugin_host
                .try_lock()
                .map(|h| h.rms_processor_handle())
            {
                self.rms_processor_handle = Some(buffer.clone());
                self.audio_chart = Some(audio_chart::AudioChart::new(buffer));
            }
        }
    }

    fn update_status_message(&mut self, message: StatusBar) -> Command<Message> {
        self.status_message = message;
        Command::none()
    }

    fn update_plugin_content(&mut self, msg: plugin_content::Message) -> Command<Message> {
        let command = match &msg {
            plugin_content::Message::SetInputFile(input_file) => self.set_input_file(input_file),
            plugin_content::Message::OpenPluginWindow => self.open_plugin_window(),
            plugin_content::Message::FloatPluginWindow => self.float_plugin_window(),
            plugin_content::Message::SetAudioPlugin(path) => self.set_audio_plugin_path(path),
            plugin_content::Message::ReloadPlugin => self.reload_plugin(),
            _ => Command::none(),
        };
        let children = self.plugin_content.update(msg).map(Message::PluginContent);
        Command::batch(vec![command, children])
    }

    fn reload_plugin(&mut self) -> Command<Message> {
        let did_close = if let ClosePluginWindowResult::ClosedPlugin { window_frame } =
            self.close_plugin_window()
        {
            self.previous_plugin_window_frame = Some(window_frame);
            true
        } else {
            false
        };

        let host = self.plugin_host.clone();
        let load_future = async move {
            let result = tokio::task::spawn_blocking(move || {
                let mut host = host.lock().unwrap();
                let plugin_file_path = host
                    .plugin_file_path()
                    .clone()
                    .ok_or(ReloadPluginError::MissingHost)?;
                host.load_plugin(&plugin_file_path)?;
                Ok(())
            })
            .await;
            match result {
                Ok(result) => result,
                Err(err) => Err(ReloadPluginError::Join(err)),
            }
        };

        Command::perform(load_future, move |result| match result {
            Err(err) => Message::ReloadedPlugin(
                did_close,
                StatusBar::new(format!("{}", err), status_bar::State::Error),
            ),
            Ok(_) => Message::ReloadedPlugin(
                did_close,
                StatusBar::new("Loaded plugin", status_bar::State::Idle),
            ),
        })
    }

    // TODO - this is decoding on the main thread, but it should be on a background thread
    fn set_input_file(&mut self, input_file: &str) -> Command<Message> {
        let result = self
            .plugin_host
            .lock()
            .unwrap()
            .set_audio_file_path(PathBuf::from(input_file));

        self.reset_handles();
        result.unwrap_or_else(|err| self.error = Some(Box::new(err)));
        self.host_state.audio_input_file_path = Some(input_file.to_string());
        self.host_options_service
            .store(&self.host_state)
            .unwrap_or_else(|err| {
                log::error!("Failed to store {:?}", err);
            });
        Command::none()
    }

    fn set_audio_plugin_path(&mut self, path: &str) -> Command<Message> {
        let path = path.to_string();
        self.reset_handles();
        if let ClosePluginWindowResult::ClosedPlugin { window_frame } = self.close_plugin_window() {
            self.previous_plugin_window_frame = Some(window_frame);
        }

        self.status_message =
            StatusBar::new("Updating persisted state", status_bar::State::Warning);
        self.host_state.plugin_path = Some(path.clone());
        self.host_options_service
            .store(&self.host_state)
            .unwrap_or_else(|err| {
                log::error!("Failed to store {:?}", err);
            });

        self.status_message = StatusBar::new("Reloading plugin", status_bar::State::Warning);

        let host = self.plugin_host.clone();
        Command::perform(
            tokio::task::spawn_blocking(move || {
                let path = Path::new(&path);
                host.lock().unwrap().load_plugin(path)
            }),
            |result| match result {
                Err(err) => Message::SetStatus(StatusBar::new(
                    format!("Error loading plugin: {}", err),
                    status_bar::State::Error,
                )),
                Ok(_) => {
                    Message::SetStatus(StatusBar::new("Loaded plugin", status_bar::State::Idle))
                }
            },
        )
    }

    fn reset_handles(&mut self) {
        self.audio_chart = None;
        self.rms_processor_handle = None;
        self.volume_handle = None;
    }

    fn update_transport_controls(
        &mut self,
        message: transport_controls::Message,
    ) -> Command<Message> {
        let host = self.plugin_host.lock().unwrap();
        match message {
            transport_controls::Message::Play => {
                host.play();
            }
            transport_controls::Message::Pause => {
                host.pause();
            }
            transport_controls::Message::Stop => {
                host.stop();
            }
            _ => (),
        }
        let children = self
            .transport_controls
            .update(message)
            .map(Message::TransportControls);
        Command::batch(vec![children])
    }

    fn close_plugin_window(&mut self) -> ClosePluginWindowResult {
        if let Some(mut plugin_window_handle) = self.plugin_window_handle.take() {
            log::info!("Closing plugin editor");
            plugin_window_handle.editor.close();
            let frame = close_window(plugin_window_handle.raw_window_handle);
            frame
                .map(|window_frame| ClosePluginWindowResult::ClosedPlugin { window_frame })
                .unwrap_or(ClosePluginWindowResult::NoWindow)
        } else {
            log::warn!("Close requested, but there's no plugin window handle");
            ClosePluginWindowResult::NoWindow
        }
    }

    fn open_plugin_window(&mut self) -> Command<Message> {
        if self.plugin_window_handle.is_some() {
            log::warn!("Refusing to open 2 plugin editors");
        } else {
            log::info!("Opening plugin editor");
            if let Some(instance) = self.plugin_host.lock().unwrap().plugin_instance() {
                log::info!("Found plugin instance");
                let instance_ptr = instance.deref() as *const PluginInstance as *mut PluginInstance;
                if let Some(editor) = unsafe { instance_ptr.as_mut() }.unwrap().get_editor() {
                    log::info!("Found plugin editor");
                    let size = editor.size();
                    let window = open_plugin_window(
                        editor,
                        size,
                        self.previous_plugin_window_frame
                            .map(|frame| frame.position()),
                    );
                    log::info!("Opened editor window");
                    self.plugin_window_handle = Some(window);
                }
            }

            if self.editor_is_floating {
                let _ = self.float_plugin_window();
            }
        }
        Command::none()
    }

    fn float_plugin_window(&mut self) -> Command<Message> {
        self.editor_is_floating = true;
        if let Some(handle) = &mut self.plugin_window_handle {
            handle.float();
        }
        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        let audio_io_settings = &mut self.audio_io_settings;
        let plugin_content = &mut self.plugin_content;
        let audio_chart = &self.audio_chart;
        let transport_controls = &mut self.transport_controls;
        let status_message = &self.status_message;
        let volume_meter_state = &mut self.volume_meter_state;
        let audio_file_model = &self.audio_file_model;
        let start_stop_button_state = &mut self.start_stop_button_state;

        view::main_content_view(view::MainContentViewModel {
            audio_io_settings,
            plugin_content,
            audio_chart,
            volume_meter_state,
            transport_controls,
            status_message,
            start_stop_button_state,
            audio_file_model,
        })
    }
}

impl Drop for MainContentView {
    fn drop(&mut self) {
        let _ = self.close_plugin_window();
    }
}

struct HostContext {
    audio_io_service: Arc<Mutex<AudioIOService>>,
    host_options_service: HostOptionsService,
    host_state: HostState,
}

/// Load plugin-host state from JSON files when it starts. Do file decoding on a background thread.
fn reload_plugin_host_state(
    plugin_host: Arc<Mutex<TestPluginHost>>,
) -> (HostContext, Command<Message>) {
    log::info!("Reloading plugin-host settings from disk");
    let home_dir =
        dirs::home_dir().expect("Failed to get user HOME directory. App will fail to work.");
    let home_config_dir = home_dir.join(".plugin-host-gui");
    std::fs::create_dir_all(&home_config_dir).expect("Failed to create configuration directory.");
    let audio_io_state_storage_path = home_config_dir
        .join("audio-io-state.json")
        .to_str()
        .unwrap()
        .to_string();
    let storage_config = StorageConfig {
        audio_io_state_storage_path,
    };
    let audio_io_service = Arc::new(Mutex::new(AudioIOService::new(
        plugin_host.clone(),
        storage_config,
    )));
    let host_options_storage_path = home_config_dir
        .join("audio-thread-config.json")
        .to_str()
        .unwrap()
        .to_string();
    let host_options_service = HostOptionsService::new(host_options_storage_path);
    let host_state = host_options_service.fetch().unwrap_or_default();

    let command = {
        let host_state = host_state.clone();
        Command::perform(
            tokio::task::spawn_blocking(move || -> Result<(), AudioHostPluginLoadError> {
                log::info!("Reloading audio plugin & file in background thread");
                if let Some(path) = &host_state.audio_input_file_path {
                    plugin_host
                        .lock()
                        .unwrap()
                        .set_audio_file_path(path.into())
                        .map_err(|err| {
                            log::error!("Failed to set audio input {:?}", err);
                            err
                        })?;
                }

                if let Some(path) = &host_state.plugin_path {
                    let mut host = plugin_host.lock().unwrap();
                    host.load_plugin(Path::new(path)).map_err(|err| {
                        log::error!("Failed to set audio input {:?}", err);
                        err
                    })?;
                    host.pause();
                }
                Ok(())
            }),
            |result| match result {
                Err(err) => Message::SetStatus(StatusBar::new(
                    format!("Failed to load plugin: {}", err),
                    status_bar::State::Error,
                )),
                Ok(Err(err)) => Message::SetStatus(StatusBar::new(
                    format!("Failed to load plugin: {}", err),
                    status_bar::State::Error,
                )),
                Ok(Ok(_)) => Message::ReadyForPlayback,
            },
        )
    };

    (
        HostContext {
            audio_io_service,
            host_options_service,
            host_state,
        },
        command,
    )
}
