use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use iced::{Column, Command, Container, Element, Length, Rule, Text};
use vst::host::PluginInstance;
use vst::plugin::Plugin;

use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style::{Container0, Container1};
use plugin_host_lib::audio_io::audio_io_service::storage::StorageConfig;
use plugin_host_lib::audio_io::{AudioHost, AudioIOService, AudioIOServiceResult};
use plugin_host_lib::TestPluginHost;

use crate::services::host_options_service::{HostOptionsService, HostState};
use crate::ui::audio_io_settings;
use crate::ui::audio_io_settings::{AudioIOSettingsView, DropdownState};
use crate::ui::main_content_view::macos::{open_plugin_window, PluginWindowHandle};
use crate::ui::main_content_view::plugin_content::PluginContentView;
use crate::ui::main_content_view::transport_controls::TransportControlsView;

pub mod macos;
pub mod plugin_content;
pub mod transport_controls;

// TODO - Break-up this god struct
pub struct MainContentView {
    #[allow(dead_code)]
    plugin_host: Arc<Mutex<TestPluginHost>>,
    audio_io_service: Arc<Mutex<AudioIOService>>,
    audio_io_settings: AudioIOSettingsView,
    host_options_service: HostOptionsService,
    plugin_content: PluginContentView,
    transport_controls: TransportControlsView,
    error: Option<Box<dyn std::error::Error>>,
    plugin_window_handle: Option<PluginWindowHandle>,
    host_state: HostState,
    status_message: String,
}

#[derive(Clone, Debug)]
pub enum Message {
    AudioIOSettings(audio_io_settings::Message),
    PluginContent(plugin_content::Message),
    TransportControls(transport_controls::Message),
    SetStatus(String),
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
                status_message: String::from("Loading..."),
            },
            command,
        )
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AudioIOSettings(msg) => self.update_audio_io_settings(msg),
            Message::PluginContent(msg) => self.update_plugin_content(msg),
            Message::TransportControls(message) => self.update_transport_controls(message),
            Message::SetStatus(message) => {
                self.status_message = message;
                Command::none()
            }
            _ => Command::none(),
        }
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
            .map(|msg| Message::AudioIOSettings(msg));
        Command::batch(vec![command, children])
    }

    fn update_plugin_content(&mut self, msg: plugin_content::Message) -> Command<Message> {
        let command = match &msg {
            plugin_content::Message::SetInputFile(input_file) => {
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
                self.close_plugin_window();
                let path = path.clone();
                let host_ref = self.plugin_host.clone();
                self.host_state.plugin_path = Some(path.clone());
                self.host_options_service
                    .store(&self.host_state)
                    .unwrap_or_else(|err| {
                        log::error!("Failed to store {:?}", err);
                    });
                Command::perform(
                    tokio::task::spawn_blocking(move || {
                        let mut host = host_ref.lock().unwrap();
                        let path = Path::new(&path);
                        host.load_plugin(path)
                    }),
                    // TODO - Send back the error
                    |_result| Message::None,
                )
            }
            _ => Command::none(),
        };
        let children = self
            .plugin_content
            .update(msg)
            .map(|msg| Message::PluginContent(msg));
        Command::batch(vec![command, children])
    }

    fn update_transport_controls(
        &mut self,
        message: transport_controls::Message,
    ) -> Command<Message> {
        let host = self.plugin_host.clone();
        match message.clone() {
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
            .map(|msg| Message::TransportControls(msg));
        Command::batch(vec![children])
    }

    fn close_plugin_window(&mut self) {
        if let Some(mut plugin_window_handle) = self.plugin_window_handle.take() {
            plugin_window_handle.editor.close();
        }
    }

    fn open_plugin_window(&mut self) -> Command<Message> {
        if let Some(mut plugin_window_handle) = self.plugin_window_handle.take() {
            plugin_window_handle.editor.close();
            let size = plugin_window_handle.editor.size();
            self.plugin_window_handle = Some(open_plugin_window(plugin_window_handle.editor, size));
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
            self.audio_io_settings
                .view()
                .map(|msg| Message::AudioIOSettings(msg))
                .into(),
            Rule::horizontal(1)
                .style(audio_processor_iced_design_system::style::Rule)
                .into(),
            Container::new(
                self.plugin_content
                    .view()
                    .map(|msg| Message::PluginContent(msg)),
            )
            .style(Container0)
            .height(Length::Fill)
            .width(Length::Fill)
            .into(),
            Rule::horizontal(1)
                .style(audio_processor_iced_design_system::style::Rule)
                .into(),
            Container::new(
                self.transport_controls
                    .view()
                    .map(|msg| Message::TransportControls(msg)),
            )
            .style(Container1)
            .height(Length::Units(80))
            .width(Length::Fill)
            .into(),
            Rule::horizontal(1)
                .style(audio_processor_iced_design_system::style::Rule)
                .into(),
            Container::new(Text::new(&self.status_message).size(Spacing::small_font_size()))
                .center_y()
                .padding([0, Spacing::base_spacing()])
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
            tokio::task::spawn_blocking(move || {
                log::info!("Reloading audio plugin & file in background thread");
                if let Some(path) = &host_state.audio_input_file_path {
                    plugin_host
                        .lock()
                        .unwrap()
                        .set_audio_file_path(path.into())
                        .unwrap_or_else(|err| {
                            log::error!("Failed to set audio input {:?}", err);
                        });
                }

                if let Some(path) = &host_state.plugin_path {
                    let mut host = plugin_host.lock().unwrap();
                    host.load_plugin(Path::new(path)).unwrap_or_else(|err| {
                        log::error!("Failed to set audio input {:?}", err);
                    });
                    host.pause();
                }
            }),
            |_| Message::SetStatus(String::from("Ready for playback")),
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
