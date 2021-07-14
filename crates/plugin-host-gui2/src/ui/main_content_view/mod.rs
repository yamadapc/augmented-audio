use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use iced::{Column, Command, Container, Element, Length, Rule, Text};

use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style::{Container0, Container1};
use plugin_host_lib::audio_io::audio_io_service::storage::StorageConfig;
use plugin_host_lib::audio_io::{AudioHost, AudioIOService, AudioIOServiceResult};
use plugin_host_lib::TestPluginHost;

use crate::ui::audio_io_settings;
use crate::ui::audio_io_settings::{AudioIOSettingsView, DropdownState};
use crate::ui::main_content_view::plugin_content::PluginContentView;

mod plugin_content;

pub struct MainContentView {
    #[allow(dead_code)]
    plugin_host: Arc<Mutex<TestPluginHost>>,
    audio_io_service: AudioIOService,
    audio_io_settings: AudioIOSettingsView,
    plugin_content: PluginContentView,
    error: Option<Box<dyn std::error::Error>>,
}

#[derive(Clone, Debug)]
pub enum Message {
    AudioIOSettings(audio_io_settings::Message),
    PluginContent(plugin_content::Message),
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
        let audio_io_service = AudioIOService::new(
            plugin_host.clone(),
            StorageConfig {
                audio_io_state_storage_path: home_config_dir
                    .join("audio-io-state.json")
                    .to_str()
                    .unwrap()
                    .to_string(),
            },
        );
        MainContentView {
            plugin_host,
            audio_io_service,
            audio_io_settings,
            plugin_content: PluginContentView::new(),
            error: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AudioIOSettings(msg) => {
                match &msg {
                    audio_io_settings::Message::AudioDriverChange(driver) => {
                        self.audio_io_service
                            .set_host_id(driver.clone())
                            .unwrap_or_else(|err| {
                                self.error = Some(Box::new(err));
                            });
                    }
                    audio_io_settings::Message::InputDeviceChange(device_id) => {
                        self.audio_io_service
                            .set_input_device_id(device_id.clone())
                            .unwrap_or_else(|err| {
                                self.error = Some(Box::new(err));
                            });
                    }
                    audio_io_settings::Message::OutputDeviceChange(device_id) => {
                        self.audio_io_service
                            .set_output_device_id(device_id.clone())
                            .unwrap_or_else(|err| {
                                self.error = Some(Box::new(err));
                            });
                    }
                }
                self.audio_io_settings
                    .update(msg)
                    .map(|msg| Message::AudioIOSettings(msg))
            }
            Message::PluginContent(msg) => {
                let command = match &msg {
                    plugin_content::Message::SetInputFile(input_file) => {
                        let result = {
                            let mut host = self.plugin_host.lock().unwrap();
                            host.set_audio_file_path(PathBuf::from(input_file))
                        };
                        result.unwrap_or_else(|err| self.error = Some(Box::new(err)));
                        Command::none()
                    }
                    plugin_content::Message::SetAudioPlugin(path) => {
                        let path = path.clone();
                        let host_ref = self.plugin_host.clone();
                        Command::perform(
                            tokio::task::spawn_blocking(move || {
                                let mut host = host_ref.lock().unwrap();
                                let path = Path::new(&path);
                                host.load_plugin(path)
                            }),
                            // TODO - Send back the error
                            |result| Message::None,
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
            Container::new(Text::new("Transport controls will come here"))
                .style(Container1)
                .height(Length::Units(200))
                .width(Length::Fill)
                .into(),
            Rule::horizontal(1)
                .style(audio_processor_iced_design_system::style::Rule)
                .into(),
            Container::new(
                Text::new("Status messages will come here").size(Spacing::small_font_size()),
            )
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
