use std::sync::{Arc, Mutex};

use iced::{Command, Element};

use plugin_host_lib::audio_io::{AudioHost, AudioIOService, AudioIOServiceResult};
pub use view::Message;

pub mod dropdown_with_label;
pub mod view;

pub struct Controller {
    audio_io_service: Arc<Mutex<AudioIOService>>,
    view: view::View,
}

impl Controller {
    // TODO - This should be reading the IO state from disk on startup.
    pub fn new(audio_io_service: Arc<Mutex<AudioIOService>>) -> Self {
        let audio_driver_state = Self::build_audio_driver_dropdown_state();
        let input_device_state =
            Self::build_input_device_dropdown_state(Some(AudioIOService::default_host()))
                .unwrap_or_else(|_| view::DropdownModel::default());
        let output_device_state =
            Self::build_output_device_dropdown_state(Some(AudioIOService::default_host()))
                .unwrap_or_else(|_| view::DropdownModel::default());
        let view = view::View::new(view::Model {
            audio_driver_state,
            input_device_state,
            output_device_state,
        });

        Self {
            audio_io_service,
            view,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        let audio_io_service = self.audio_io_service.clone();
        let command = match message.clone() {
            Message::AudioDriverChange(driver) => Command::perform(
                tokio::task::spawn_blocking(move || {
                    audio_io_service.lock().unwrap().set_host_id(driver)
                }),
                |_| Message::None,
            ),
            Message::InputDeviceChange(device_id) => Command::perform(
                tokio::task::spawn_blocking(move || {
                    audio_io_service
                        .lock()
                        .unwrap()
                        .set_input_device_id(device_id)
                }),
                |_| Message::None,
            ),
            Message::OutputDeviceChange(device_id) => Command::perform(
                tokio::task::spawn_blocking(move || {
                    audio_io_service
                        .lock()
                        .unwrap()
                        .set_output_device_id(device_id)
                }),
                |_| Message::None,
            ),
            _ => Command::none(),
        };
        let children = self.view.update(message);
        Command::batch(vec![command, children])
    }

    pub fn view(&mut self) -> Element<Message> {
        self.view.view()
    }

    fn build_audio_driver_dropdown_state() -> view::DropdownModel {
        let default_host = AudioIOService::default_host();
        let hosts = AudioIOService::hosts();
        view::DropdownModel {
            selected_option: Some(default_host),
            options: hosts,
        }
    }

    fn build_input_device_dropdown_state(
        host: Option<AudioHost>,
    ) -> AudioIOServiceResult<view::DropdownModel> {
        let default_input_device = AudioIOService::default_input_device().map(|device| device.name);
        let input_devices = AudioIOService::input_devices(host)?
            .into_iter()
            .map(|device| device.name)
            .collect();
        Ok(view::DropdownModel {
            selected_option: default_input_device,
            options: input_devices,
        })
    }

    fn build_output_device_dropdown_state(
        host: Option<AudioHost>,
    ) -> AudioIOServiceResult<view::DropdownModel> {
        let default_output_device =
            AudioIOService::default_output_device().map(|device| device.name);
        let output_devices = AudioIOService::output_devices(host)?
            .into_iter()
            .map(|device| device.name)
            .collect();
        Ok(view::DropdownModel {
            selected_option: default_output_device,
            options: output_devices,
        })
    }
}
