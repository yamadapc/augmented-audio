use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use iced::{Column, Command, Container, Element, Length, Row, Rule, Text};
use vst::host::PluginInstance;
use vst::plugin::Plugin;

use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style::{Container0, Container1};
use plugin_host_lib::audio_io::audio_io_service::storage::StorageConfig;
use plugin_host_lib::audio_io::{
    AudioHost, AudioHostPluginLoadError, AudioIOService, AudioIOServiceResult,
};
use plugin_host_lib::processors::running_rms_processor::RunningRMSProcessorHandle;
use plugin_host_lib::processors::volume_meter_processor::VolumeMeterProcessorHandle;
use plugin_host_lib::TestPluginHost;

use crate::services::host_options_service::{HostOptionsService, HostState};
use crate::ui::audio_io_settings;
use crate::ui::audio_io_settings::{AudioIOSettingsView, DropdownState};
use crate::ui::main_content_view::audio_chart::AudioChart;
use crate::ui::main_content_view::plugin_content::PluginContentView;
use crate::ui::main_content_view::status_bar::StatusBar;
use crate::ui::main_content_view::transport_controls::TransportControlsView;
use crate::ui::main_content_view::volume_meter::VolumeMeter;
use crate::ui::plugin_editor_window::{close_window, open_plugin_window, PluginWindowHandle};

mod audio_chart;
pub mod plugin_content;
pub mod status_bar;
pub mod transport_controls;
mod volume_meter;

// TODO - Break-up this god struct
pub struct MainContentView {
    // TODO - This should not be under a lock
    plugin_host: Arc<Mutex<TestPluginHost>>,
    // TODO - This should not be under a lock
    audio_io_service: Arc<Mutex<AudioIOService>>,
    audio_io_settings: AudioIOSettingsView,
    host_options_service: HostOptionsService,
    plugin_content: PluginContentView,
    transport_controls: TransportControlsView,
    error: Option<Box<dyn std::error::Error>>,
    plugin_window_handle: Option<PluginWindowHandle>,
    host_state: HostState,
    status_message: StatusBar,
    // This should not be optional & it might break if the host restarts processors for some reason
    volume_handle: Option<Shared<VolumeMeterProcessorHandle>>,
    rms_processor_handle: Option<Shared<RunningRMSProcessorHandle>>,
    audio_chart: Option<AudioChart>,
}

#[derive(Clone, Debug)]
pub enum Message {
    AudioIOSettings(audio_io_settings::Message),
    PluginContent(plugin_content::Message),
    TransportControls(transport_controls::Message),
    SetStatus(StatusBar),
    None,
}

impl MainContentView {
    pub fn new(plugin_host: Arc<Mutex<TestPluginHost>>) -> (Self, Command<Message>) {
        let (
            HostContext {
                audio_io_service,
                host_options_service,
                host_state,
            },
            command,
        ) = reload_plugin_host_state(plugin_host.clone());

        let audio_driver_state = MainContentView::build_audio_driver_dropdown_state();
        let input_device_state = MainContentView::build_input_device_dropdown_state(Some(
            AudioIOService::default_host(),
        ))
        .unwrap_or_else(|_| DropdownState::default());
        let output_device_state = MainContentView::build_output_device_dropdown_state(Some(
            AudioIOService::default_host(),
        ))
        .unwrap_or_else(|_| DropdownState::default());
        let audio_io_settings = AudioIOSettingsView::new(audio_io_settings::ViewModel {
            audio_driver_state,
            input_device_state,
            output_device_state,
        });
        let plugin_content = PluginContentView::new(
            host_state.audio_input_file_path.clone(),
            host_state.plugin_path.clone(),
        );

        (
            MainContentView {
                plugin_host,
                audio_io_service,
                audio_io_settings,
                host_options_service,
                host_state,
                plugin_content,
                transport_controls: TransportControlsView::new(),
                error: None,
                plugin_window_handle: None,
                status_message: StatusBar::new("Starting audio thread", status_bar::State::Warning),
                volume_handle: None,
                rms_processor_handle: None,
                audio_chart: None,
            },
            command,
        )
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        self.poll_for_host_handles();
        if let Some(chart) = &mut self.audio_chart {
            chart.update();
        }
        match message {
            Message::AudioIOSettings(msg) => self.update_audio_io_settings(msg),
            Message::PluginContent(msg) => self.update_plugin_content(msg),
            Message::TransportControls(message) => self.update_transport_controls(message),
            Message::SetStatus(message) => self.update_status_message(message),
            _ => Command::none(),
        }
    }

    fn poll_for_host_handles(&mut self) {
        if self.volume_handle.is_none() {
            if let Some(volume_handle) = self
                .plugin_host
                .try_lock()
                .ok()
                .map(|host| host.volume_handle())
                .flatten()
            {
                self.volume_handle = Some(volume_handle);
            }
        }
        if self.rms_processor_handle.is_none() {
            if let Some(buffer) = self
                .plugin_host
                .try_lock()
                .ok()
                .map(|host| host.rms_processor_handle())
                .flatten()
            {
                self.rms_processor_handle = Some(buffer.clone());
                self.audio_chart = Some(AudioChart::new(buffer));
            }
        }
    }

    fn update_status_message(&mut self, message: StatusBar) -> Command<Message> {
        self.status_message = message;
        Command::none()
    }

    fn update_audio_io_settings(&mut self, msg: audio_io_settings::Message) -> Command<Message> {
        let audio_io_service = self.audio_io_service.clone();
        let command = match msg.clone() {
            audio_io_settings::Message::AudioDriverChange(driver) => Command::perform(
                tokio::task::spawn_blocking(move || {
                    audio_io_service.lock().unwrap().set_host_id(driver)
                }),
                |_| Message::None,
            ),
            audio_io_settings::Message::InputDeviceChange(device_id) => Command::perform(
                tokio::task::spawn_blocking(move || {
                    audio_io_service
                        .lock()
                        .unwrap()
                        .set_input_device_id(device_id)
                }),
                |_| Message::None,
            ),
            audio_io_settings::Message::OutputDeviceChange(device_id) => Command::perform(
                tokio::task::spawn_blocking(move || {
                    audio_io_service
                        .lock()
                        .unwrap()
                        .set_output_device_id(device_id)
                }),
                |_| Message::None,
            ),
        };
        let children = self
            .audio_io_settings
            .update(msg)
            .map(Message::AudioIOSettings);
        Command::batch(vec![command, children])
    }

    fn update_plugin_content(&mut self, msg: plugin_content::Message) -> Command<Message> {
        let command = match &msg {
            plugin_content::Message::SetInputFile(input_file) => {
                self.reset_handles();
                let result = {
                    let mut host = self.plugin_host.lock().unwrap();
                    host.set_audio_file_path(PathBuf::from(input_file))
                };
                result.unwrap_or_else(|err| self.error = Some(Box::new(err)));
                self.host_state.audio_input_file_path = Some(input_file.clone());
                self.host_options_service
                    .store(&self.host_state)
                    .unwrap_or_else(|err| {
                        log::error!("Failed to store {:?}", err);
                    });
                Command::none()
            }
            plugin_content::Message::OpenPluginWindow => self.open_plugin_window(),
            plugin_content::Message::SetAudioPlugin(path) => {
                self.reset_handles();
                self.close_plugin_window();
                let path = path.clone();

                self.status_message =
                    StatusBar::new("Updating persisted state", status_bar::State::Warning);
                self.host_state.plugin_path = Some(path.clone());
                self.host_options_service
                    .store(&self.host_state)
                    .unwrap_or_else(|err| {
                        log::error!("Failed to store {:?}", err);
                    });

                self.status_message =
                    StatusBar::new("Reloading plugin", status_bar::State::Warning);

                let host_ref = self.plugin_host.clone();
                Command::perform(
                    tokio::task::spawn_blocking(move || {
                        let mut host = host_ref.lock().unwrap();
                        let path = Path::new(&path);
                        host.load_plugin(path)
                    }),
                    |result| match result {
                        Err(err) => Message::SetStatus(StatusBar::new(
                            format!("Error loading plugin: {}", err),
                            status_bar::State::Error,
                        )),
                        Ok(_) => Message::SetStatus(StatusBar::new(
                            "Loaded plugin",
                            status_bar::State::Idle,
                        )),
                    },
                )
            }
            plugin_content::Message::ReloadPlugin => {
                self.close_plugin_window();
                let host_ref = self.plugin_host.clone();
                self.reset_handles();
                Command::perform(
                    tokio::task::spawn_blocking(move || {
                        let mut host = host_ref.lock().unwrap();
                        host.plugin_file_path()
                            .clone()
                            .map(|plugin_file_path| host.load_plugin(&plugin_file_path))
                    }),
                    |result| match result {
                        Err(err) => Message::SetStatus(StatusBar::new(
                            format!("Failure loading plugin in a background thread: {}", err),
                            status_bar::State::Error,
                        )),
                        Ok(None) => Message::SetStatus(StatusBar::new(
                            "There's no plugin loaded, configure the plugin path",
                            status_bar::State::Warning,
                        )),
                        Ok(Some(Err(err))) => Message::SetStatus(StatusBar::new(
                            format!("Error loading plugin: {}", err),
                            status_bar::State::Error,
                        )),
                        Ok(Some(Ok(_))) => Message::SetStatus(StatusBar::new(
                            "Loaded plugin",
                            status_bar::State::Idle,
                        )),
                    },
                )
            }
            _ => Command::none(),
        };
        let children = self.plugin_content.update(msg).map(Message::PluginContent);
        Command::batch(vec![command, children])
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
        let host = self.plugin_host.clone();
        match message {
            transport_controls::Message::Play => {
                let host = host.lock().unwrap();
                host.play();
            }
            transport_controls::Message::Pause => {
                let host = host.lock().unwrap();
                host.pause();
            }
            transport_controls::Message::Stop => {
                let host = host.lock().unwrap();
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

    fn close_plugin_window(&mut self) {
        if let Some(mut plugin_window_handle) = self.plugin_window_handle.take() {
            log::info!("Closing plugin editor");
            plugin_window_handle.editor.close();
            close_window(plugin_window_handle.raw_window_handle);
        } else {
            log::warn!("Close requested, but there's no plugin window handle");
        }
    }

    fn open_plugin_window(&mut self) -> Command<Message> {
        if self.plugin_window_handle.is_some() {
            log::warn!("Refusing to open 2 plugin editors");
        } else {
            log::info!("Opening plugin editor");
            let mut host = self.plugin_host.lock().unwrap();
            if let Some(instance) = host.plugin_instance() {
                log::info!("Found plugin instance");
                let instance_ptr = instance.deref() as *const PluginInstance as *mut PluginInstance;
                if let Some(editor) = unsafe { instance_ptr.as_mut() }.unwrap().get_editor() {
                    log::info!("Found plugin editor");
                    let size = editor.size();
                    let window = open_plugin_window(editor, size);
                    log::info!("Opened editor window");
                    self.plugin_window_handle = Some(window);
                }
            }
        }
        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::with_children(vec![
            self.audio_io_settings.view().map(Message::AudioIOSettings),
            Rule::horizontal(1)
                .style(audio_processor_iced_design_system::style::Rule)
                .into(),
            Container::new(self.plugin_content.view().map(Message::PluginContent))
                .style(Container0)
                .height(Length::Fill)
                .width(Length::Fill)
                .into(),
            Rule::horizontal(1)
                .style(audio_processor_iced_design_system::style::Rule)
                .into(),
            Container::new(
                Row::with_children(vec![
                    Container::new(
                        Container::new::<Element<Message>>(match &self.audio_chart {
                            Some(chart) => chart.view().map(|_| Message::None),
                            None => Text::new("").into(),
                        })
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .style(Container0),
                    )
                    .padding(Spacing::base_spacing())
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .into(),
                    Container::new(
                        VolumeMeter::new((&self.volume_handle).into())
                            .view()
                            .map(|_| Message::None),
                    )
                    .style(Container1::default().border())
                    .width(Length::Units(Spacing::base_control_size()))
                    .height(Length::Fill)
                    .into(),
                ])
                .width(Length::Fill),
            )
            .style(Container1::default())
            .height(Length::Units(150))
            .width(Length::Fill)
            .into(),
            Rule::horizontal(1)
                .style(audio_processor_iced_design_system::style::Rule)
                .into(),
            Container::new(
                self.transport_controls
                    .view()
                    .map(Message::TransportControls),
            )
            .style(Container1::default())
            .height(Length::Units(80))
            .width(Length::Fill)
            .into(),
            Rule::horizontal(1)
                .style(audio_processor_iced_design_system::style::Rule)
                .into(),
            Container::new(self.status_message.clone().view().map(|_| Message::None))
                .center_y()
                .style(Container0)
                .height(Length::Units(20))
                .width(Length::Fill)
                .into(),
        ])
        .into()
    }

    fn build_audio_driver_dropdown_state() -> DropdownState {
        let default_host = AudioIOService::default_host();
        let hosts = AudioIOService::hosts();
        DropdownState {
            selected_option: Some(default_host),
            options: hosts,
        }
    }

    fn build_input_device_dropdown_state(
        host: Option<AudioHost>,
    ) -> AudioIOServiceResult<DropdownState> {
        let default_input_device = AudioIOService::default_input_device().map(|device| device.name);
        let input_devices = AudioIOService::input_devices(host)?
            .into_iter()
            .map(|device| device.name)
            .collect();
        Ok(DropdownState {
            selected_option: default_input_device,
            options: input_devices,
        })
    }

    fn build_output_device_dropdown_state(
        host: Option<AudioHost>,
    ) -> AudioIOServiceResult<DropdownState> {
        let default_output_device =
            AudioIOService::default_output_device().map(|device| device.name);
        let output_devices = AudioIOService::output_devices(host)?
            .into_iter()
            .map(|device| device.name)
            .collect();
        Ok(DropdownState {
            selected_option: default_output_device,
            options: output_devices,
        })
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
                Ok(_) => Message::SetStatus(StatusBar::new(
                    "Ready for playback",
                    status_bar::State::Idle,
                )),
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
