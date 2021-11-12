use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use actix::prelude::*;
use derivative::Derivative;
use thiserror::Error;

use crate::actor_system::ActorSystemThread;
use augmented::audio::gc::Shared;
use augmented::gui::iced::{Command, Element, Subscription};
use plugin_host_lib::{
    audio_io::audio_io_service,
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

mod audio_chart;
mod audio_file_chart;
pub mod plugin_content;
pub mod plugin_editor_window;
pub mod status_bar;
pub mod transport_controls;
mod view;
pub mod volume_meter;

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
    host_state: HostState,
    /// Volume processor handle
    /// This should not be optional & it might break if the host restarts processors for some reason
    volume_handle: Option<Shared<VolumeMeterProcessorHandle>>,
    /// RMS processor handle
    /// This should not be optional & it might break if the host restarts processors for some reason
    rms_processor_handle: Option<Shared<RunningRMSProcessorHandle>>,
    /// Amplitude over time chart
    audio_chart: Option<audio_chart::AudioChart>,
    /// Unused
    audio_file_model: audio_file_chart::AudioFileModel,

    /// Holds the audio settings view and its sync with the test host
    audio_io_settings: audio_io_settings::Controller,
    /// Reloads options from disk
    host_options_service: HostOptionsService,
    /// Center content view
    plugin_content: plugin_content::View,
    /// Playback buttons
    transport_controls: TransportControlsView,
    /// Holds the plugin GUI window
    editor_controller: plugin_editor_window::EditorController<Arc<Mutex<TestPluginHost>>>,
    volume_meter_state: volume_meter::VolumeMeter,
    status_message: StatusBar,
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
    SetAudioFilePathResponse(String),
    Exit,
    None,
}

impl MainContentView {
    pub fn new(
        plugin_host: TestPluginHost,
        actor_system_thread: &ActorSystemThread,
    ) -> (Self, Command<Message>) {
        let plugin_host = Arc::new(Mutex::new(plugin_host));
        let (
            HostContext {
                audio_io_service,
                host_options_service,
                host_state,
            },
            command,
        ) = reload_plugin_host_state(plugin_host.clone(), actor_system_thread);

        let plugin_content = plugin_content::View::new(
            host_state.audio_input_file_path.clone(),
            host_state.plugin_path.clone(),
        );
        let (audio_io_settings, audio_io_settings_command) =
            audio_io_settings::Controller::new(audio_io_service);
        let editor_controller = plugin_editor_window::EditorController::new(plugin_host.clone());

        let command = Command::batch(vec![
            command,
            audio_io_settings_command.map(Message::AudioIOSettings),
        ]);

        (
            MainContentView {
                plugin_host,
                audio_io_settings,
                host_options_service,
                host_state,
                plugin_content,
                editor_controller,
                transport_controls: TransportControlsView::default(),
                status_message: StatusBar::new("Starting audio thread", status_bar::State::Warning),
                volume_handle: None,
                rms_processor_handle: None,
                audio_chart: None,
                audio_file_model: AudioFileModel::empty(),
                volume_meter_state: volume_meter::VolumeMeter::default(),
                start_stop_button_state: view::StartStopViewModel::default(),
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
            Message::AudioIOSettings(msg) => self.update_audio_io_settings(msg),
            Message::PluginContent(msg) => self.update_plugin_content(msg),
            Message::TransportControls(message) => self.update_transport_controls(message),
            Message::SetStatus(message) => self.update_status_message(message),
            Message::ReadyForPlayback => Self::update_ready_for_playback(),
            Message::ReloadedPlugin(did_close, status_bar) => {
                self.update_reloaded_plugin(did_close, status_bar)
            }
            Message::StartStopButtonClicked => self.update_start_stop_button_clicked(),
            Message::VolumeMeter(message) => self.update_volume_meter(message),
            Message::Exit => self.update_exit(),
            Message::SetAudioFilePathResponse(input_file) => {
                self.update_set_audio_file_path_response(input_file)
            }
            Message::None => Command::none(),
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

    pub fn view(&mut self) -> Element<Message> {
        let audio_io_settings = &mut self.audio_io_settings;
        let plugin_content = &mut self.plugin_content;
        let audio_chart = &mut self.audio_chart;
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

// Child update event dispatch
impl MainContentView {
    fn update_status_message(&mut self, message: StatusBar) -> Command<Message> {
        self.status_message = message;
        Command::none()
    }

    fn update_plugin_content(&mut self, msg: plugin_content::Message) -> Command<Message> {
        let command = match &msg {
            plugin_content::Message::SetInputFile(input_file) => self.set_input_file(input_file),
            plugin_content::Message::OpenPluginWindow => {
                self.editor_controller.open_window();
                Command::none()
            }
            plugin_content::Message::ClosePluginWindow => {
                let _ = self.editor_controller.close_window();
                Command::none()
            }
            plugin_content::Message::FloatPluginWindow => {
                self.editor_controller.float_window();
                Command::none()
            }
            plugin_content::Message::SetAudioPlugin(path) => self.set_audio_plugin_path(path),
            plugin_content::Message::ReloadPlugin => self.reload_plugin(),
            _ => Command::none(),
        };
        let children = self.plugin_content.update(msg).map(Message::PluginContent);
        Command::batch(vec![command, children])
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

    fn update_set_audio_file_path_response(&mut self, input_file: String) -> Command<Message> {
        self.on_set_input_file_response(input_file);
        Command::none()
    }

    fn update_exit(&mut self) -> Command<Message> {
        let _ = self.editor_controller.close_window();
        Command::none()
    }

    fn update_volume_meter(&mut self, message: volume_meter::Message) -> Command<Message> {
        volume_meter::update(message, self.plugin_host.clone()).map(Message::VolumeMeter)
    }

    fn update_start_stop_button_clicked(&mut self) -> Command<Message> {
        self.start_stop_button_state.is_started = !self.start_stop_button_state.is_started;
        let plugin_host = self.plugin_host.clone();
        let should_start = self.start_stop_button_state.is_started;
        Command::perform(
            async move { Self::run_on_toggle_playback_command(plugin_host, should_start) },
            |_| Message::None,
        )
    }

    fn update_reloaded_plugin(
        &mut self,
        did_close: bool,
        status_bar: StatusBar,
    ) -> Command<Message> {
        self.reset_handles();
        self.status_message = status_bar;
        if did_close {
            self.editor_controller.open_window();
        }
        Command::none()
    }

    // TODO Util function?
    fn update_ready_for_playback() -> Command<Message> {
        Command::perform(iced_futures::futures::future::ready(()), |_| {
            Message::SetStatus(StatusBar::new(
                "Ready for playback",
                status_bar::State::Idle,
            ))
        })
    }

    fn update_audio_io_settings(&mut self, msg: audio_io_settings::Message) -> Command<Message> {
        self.audio_io_settings
            .update(msg)
            .map(Message::AudioIOSettings)
    }

    fn run_on_toggle_playback_command(plugin_host: Arc<Mutex<TestPluginHost>>, should_start: bool) {
        let mut plugin_host = plugin_host.lock().unwrap();
        if should_start {
            if let Err(err) = plugin_host.start_audio() {
                log::error!("Error starting host: {}", err);
            }
        } else {
            plugin_host.stop();
        }
    }
}

// Current mechanism for getting handles to audio processors
impl MainContentView {
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

    fn reset_handles(&mut self) {
        self.audio_chart = None;
        self.rms_processor_handle = None;
        self.volume_handle = None;
    }
}

// Input file event controller
impl MainContentView {
    fn set_input_file(&mut self, input_file: &str) -> Command<Message> {
        let plugin_host = self.plugin_host.clone();
        Command::perform(
            MainContentView::handle_set_input_file_path(plugin_host, input_file.to_string()),
            move |result| match result {
                Ok(input_file) => Message::SetAudioFilePathResponse(input_file),
                Err(err) => {
                    Message::SetStatus(StatusBar::new(format!("{}", err), status_bar::State::Error))
                }
            },
        )
    }

    async fn handle_set_input_file_path(
        plugin_host: Arc<Mutex<TestPluginHost>>,
        input_file: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let path = PathBuf::from(&input_file);
        tokio::task::spawn_blocking(move || plugin_host.lock().unwrap().set_audio_file_path(path))
            .await??;
        Ok(input_file)
    }

    fn on_set_input_file_response(&mut self, input_file: String) {
        self.reset_handles();
        self.host_state.audio_input_file_path = Some(input_file);
        self.host_options_service
            .store(&self.host_state)
            .unwrap_or_else(|err| {
                log::error!("Failed to store {:?}", err);
            });
    }
}

// Audio plugin file event controller
impl MainContentView {
    fn reload_plugin(&mut self) -> Command<Message> {
        let did_close = self.editor_controller.on_reload();

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

    fn set_audio_plugin_path(&mut self, path: &str) -> Command<Message> {
        let path = path.to_string();
        self.reset_handles();
        let _ = self.editor_controller.close_window();

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
}

impl Drop for MainContentView {
    fn drop(&mut self) {
        let _ = self.editor_controller.close_window();
    }
}

struct HostContext {
    audio_io_service: Addr<AudioIOService>,
    host_options_service: HostOptionsService,
    host_state: HostState,
}

/// Load plugin-host state from JSON files when it starts. Do file decoding on a background thread.
fn reload_plugin_host_state(
    plugin_host: Arc<Mutex<TestPluginHost>>,
    actor_system_thread: &ActorSystemThread,
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
    let audio_io_service = {
        let plugin_host = plugin_host.clone();
        actor_system_thread
            .spawn_result(async move { AudioIOService::new(plugin_host, storage_config).start() })
    };
    let host_options_storage_path = home_config_dir
        .join("audio-thread-config.json")
        .to_str()
        .unwrap()
        .to_string();
    let host_options_service = HostOptionsService::new(host_options_storage_path);
    let host_state = host_options_service.fetch().unwrap_or_default();

    let command = {
        let host_state = host_state.clone();
        let audio_io_service = audio_io_service.clone();

        Command::perform(
            async move {
                if let Err(err) = audio_io_service.send(audio_io_service::ReloadMessage).await {
                    log::error!("Failed to reload audio options: {}", err);
                } else {
                    log::info!("Loaded audio options");
                }

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
            },
            |result: Result<(), AudioHostPluginLoadError>| match result {
                Err(err) => Message::SetStatus(StatusBar::new(
                    format!("Failed to load plugin: {}", err),
                    status_bar::State::Error,
                )),
                Ok(_) => Message::ReadyForPlayback,
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
