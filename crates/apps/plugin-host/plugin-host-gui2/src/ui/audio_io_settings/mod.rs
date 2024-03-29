// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
use actix::{Addr, MailboxError};
use iced::{Command, Element};

use plugin_host_lib::audio_io::{
    AudioHost, AudioIOService, AudioIOServiceResult, AudioIOState, GetStateMessage, ReloadMessage,
    SetStateMessage,
};

use crate::ui::audio_io_settings::view::DropdownModel;

pub mod dropdown_with_label;
pub mod view;

pub struct Controller {
    audio_io_service: Addr<AudioIOService>,
    view: view::View,
}

#[derive(Debug, Clone)]
pub enum Message {
    InitialStateLoaded(AudioIOState),
    View(view::Message),
    None,
}

impl Controller {
    // TODO - This should be reading the IO state from disk on startup.
    pub fn new(audio_io_service: Addr<AudioIOService>) -> (Self, Command<Message>) {
        let audio_driver_state = Self::build_audio_driver_dropdown_state();
        let input_device_state =
            Self::build_input_device_dropdown_state(Some(AudioIOService::default_host()))
                .unwrap_or_default();
        let output_device_state =
            Self::build_output_device_dropdown_state(Some(AudioIOService::default_host()))
                .unwrap_or_default();
        let sample_rate_state =
            Self::build_sample_rate_dropdown_state(AudioIOService::default_host())
                .unwrap_or_default();

        let view = view::View::new(view::Model {
            audio_driver_state,
            input_device_state,
            output_device_state,
            sample_rate_state,
            buffer_size_state: DropdownModel {
                selected_option: None,
                options: vec![],
            },
        });

        let command = Controller::on_init(audio_io_service.clone());

        (
            Self {
                audio_io_service,
                view,
            },
            command,
        )
    }

    fn on_init(audio_io_service: Addr<AudioIOService>) -> Command<Message> {
        Command::perform(
            async move {
                let _ = audio_io_service.send(ReloadMessage).await;
                let result: Result<AudioIOState, MailboxError> =
                    audio_io_service.send(GetStateMessage).await;
                result
            },
            move |result| {
                match result {
                    Ok(state) => Message::InitialStateLoaded(state),
                    // TODO - Get a better error handling strategy up
                    Err(_err) => Message::None,
                }
            },
        )
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        let audio_io_service = self.audio_io_service.clone();
        let command = match message.clone() {
            Message::InitialStateLoaded(state) => {
                let mut commands = vec![];
                commands.push(
                    self.view
                        .update(view::Message::AudioDriverChange(state.host.clone()))
                        .map(Message::View),
                );
                if let Some(device) = state.input_device {
                    commands.push(
                        self.view
                            .update(view::Message::InputDeviceChange(device.name))
                            .map(Message::View),
                    );
                }
                if let Some(device) = state.output_device {
                    commands.push(
                        self.view
                            .update(view::Message::OutputDeviceChange(device.name))
                            .map(Message::View),
                    );
                }
                Command::batch(commands)
            }
            Message::View(view::Message::AudioDriverChange(host_id)) => Command::perform(
                audio_io_service.send(SetStateMessage::SetHostId { host_id }),
                |_| Message::None,
            ),
            Message::View(view::Message::InputDeviceChange(input_device_id)) => Command::perform(
                audio_io_service.send(SetStateMessage::SetInputDeviceId { input_device_id }),
                |_| Message::None,
            ),
            Message::View(view::Message::OutputDeviceChange(output_device_id)) => Command::perform(
                audio_io_service.send(SetStateMessage::SetOutputDeviceId { output_device_id }),
                |_| Message::None,
            ),
            _ => Command::none(),
        };

        let mut commands = vec![command];
        if let Message::View(message) = message {
            let children = self.view.update(message);
            commands.push(children.map(Message::View));
        }
        Command::batch(commands)
    }

    pub fn view(&self) -> Element<Message> {
        self.view.view().map(Message::View)
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

    fn build_sample_rate_dropdown_state(_host: AudioHost) -> Option<view::DropdownModel> {
        let input_device = AudioIOService::default_input_device()?;

        Some(view::DropdownModel {
            selected_option: Some("44100Hz".into()),
            options: input_device
                .sample_rates()
                .iter()
                .map(|rate| format!("{}Hz", rate.0))
                .collect(),
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
