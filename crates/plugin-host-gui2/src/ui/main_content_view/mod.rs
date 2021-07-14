use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use iced::{Column, Command, Container, Element, Length, Rule, Text};
use raw_window_handle::RawWindowHandle;
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

mod macos;
mod pause;
mod plugin_content;
mod stop;
mod transport_controls;
mod triangle;

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
    editor: Option<PluginWindowHandle>,
    host_state: HostState,
}

#[derive(Clone, Debug)]
pub enum Message {
    AudioIOSettings(audio_io_settings::Message),
    PluginContent(plugin_content::Message),
    TransportControls(transport_controls::Message),
    None,
}

impl MainContentView {
    pub fn new(plugin_host: Arc<Mutex<TestPluginHost>>) -> Self {
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
        let home_dir =
            dirs::home_dir().expect("Failed to get user HOME directory. App will fail to work.");
        let home_config_dir = home_dir.join(".plugin-host-gui");
        std::fs::create_dir_all(&home_config_dir)
            .expect("Failed to create configuration directory.");
        let audio_io_service = Arc::new(Mutex::new(AudioIOService::new(
            plugin_host.clone(),
            StorageConfig {
                audio_io_state_storage_path: home_config_dir
                    .join("audio-io-state.json")
                    .to_str()
                    .unwrap()
                    .to_string(),
            },
        )));
        let host_options_service = HostOptionsService::new(
            home_config_dir
                .join("audio-thread-config.json")
                .to_str()
                .unwrap()
                .to_string(),
        );
        let host_state = host_options_service.fetch().unwrap_or_default();
        let plugin_content = PluginContentView::new(
            host_state.audio_input_file_path.clone(),
            host_state.plugin_path.clone(),
        );

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
            plugin_host
                .lock()
                .unwrap()
                .load_plugin(Path::new(path))
                .unwrap_or_else(|err| {
                    log::error!("Failed to set audio input {:?}", err);
                });
        }

        MainContentView {
            plugin_host,
            audio_io_service,
            audio_io_settings,
            host_options_service,
            host_state,
            plugin_content,
            transport_controls: TransportControlsView::new(),
            error: None,
            editor: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AudioIOSettings(msg) => {
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
            Message::PluginContent(msg) => {
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
                    plugin_content::Message::OpenPluginWindow => {
                        log::info!("Opening plugin editor");
                        let mut host = self.plugin_host.lock().unwrap();
                        if let Some(instance) = host.plugin_instance() {
                            log::info!("Found plugin instance");
                            let instance_ptr =
                                instance.deref() as *const PluginInstance as *mut PluginInstance;
                            if let Some(editor) =
                                unsafe { instance_ptr.as_mut() }.unwrap().get_editor()
                            {
                                log::info!("Found plugin editor");
                                let size = editor.size();
                                let window = open_plugin_window(editor, size);
                                log::info!("Opened editor window");
                                self.editor = Some(window);
                            }
                        }

                        Command::none()
                    }
                    plugin_content::Message::SetAudioPlugin(path) => {
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
            Message::TransportControls(message) => {
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
            _ => Command::none(),
        }
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
            Container::new(
                Text::new("Status messages will come here").size(Spacing::small_font_size()),
            )
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
