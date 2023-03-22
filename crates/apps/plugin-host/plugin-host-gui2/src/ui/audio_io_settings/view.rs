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
use iced::{
    widget::{Column, Container, Rule, Text},
    Command, Element, Length,
};

use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style::{Container0, Container1};

use super::dropdown_with_label::DropdownWithLabel;

pub struct View {
    audio_driver_dropdown: DropdownWithLabel,
    input_device_dropdown: DropdownWithLabel,
    output_device_dropdown: DropdownWithLabel,
    sample_rate_dropdown: DropdownWithLabel,
    buffer_size_dropdown: DropdownWithLabel,
}

#[derive(Debug, Clone)]
pub enum Message {
    AudioDriverChange(String),
    InputDeviceChange(String),
    OutputDeviceChange(String),
    SampleRateChange(String),
    BufferSizeChange(String),
    None,
}

impl View {
    pub fn new(model: Model) -> Self {
        View {
            audio_driver_dropdown: DropdownWithLabel::new(
                "Audio driver",
                model.audio_driver_state.options,
                model.audio_driver_state.selected_option,
            ),
            input_device_dropdown: DropdownWithLabel::new(
                "Input device",
                model.input_device_state.options,
                model.input_device_state.selected_option,
            ),
            output_device_dropdown: DropdownWithLabel::new(
                "Output device",
                model.output_device_state.options,
                model.output_device_state.selected_option,
            ),
            sample_rate_dropdown: DropdownWithLabel::new(
                "Sample rate",
                model.sample_rate_state.options,
                model.sample_rate_state.selected_option,
            ),
            buffer_size_dropdown: DropdownWithLabel::new(
                "Buffer size",
                model.buffer_size_state.options,
                model.buffer_size_state.selected_option,
            ),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AudioDriverChange(selected) => self
                .audio_driver_dropdown
                .update(selected)
                .map(Message::AudioDriverChange),
            Message::InputDeviceChange(selected) => self
                .input_device_dropdown
                .update(selected)
                .map(Message::InputDeviceChange),
            Message::OutputDeviceChange(selected) => self
                .output_device_dropdown
                .update(selected)
                .map(Message::OutputDeviceChange),
            Message::SampleRateChange(selected) => self
                .sample_rate_dropdown
                .update(selected)
                .map(Message::SampleRateChange),
            Message::BufferSizeChange(selected) => self
                .buffer_size_dropdown
                .update(selected)
                .map(Message::BufferSizeChange),
            Message::None => Command::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let header = section_heading("Audio IO Settings");
        let content = self.content_view();
        Column::with_children(vec![header.into(), content.into()])
            .width(Length::Fill)
            .into()
    }

    pub fn content_view(&self) -> impl Into<Element<Message>> {
        Container::new(
            Column::with_children(vec![
                self.audio_driver_dropdown
                    .view()
                    .map(Message::AudioDriverChange),
                self.input_device_dropdown
                    .view()
                    .map(Message::InputDeviceChange),
                self.output_device_dropdown
                    .view()
                    .map(Message::OutputDeviceChange),
                self.sample_rate_dropdown
                    .view()
                    .map(Message::SampleRateChange),
                self.buffer_size_dropdown
                    .view()
                    .map(Message::BufferSizeChange),
            ])
            .spacing(Spacing::base_spacing()),
        )
        .padding(Spacing::base_spacing())
        .width(Length::Fill)
        .style(Container1::default())
    }
}

pub struct Model {
    pub audio_driver_state: DropdownModel,
    pub input_device_state: DropdownModel,
    pub output_device_state: DropdownModel,
    pub sample_rate_state: DropdownModel,
    pub buffer_size_state: DropdownModel,
}

#[derive(Default)]
pub struct DropdownModel {
    pub selected_option: Option<String>,
    pub options: Vec<String>,
}

fn section_heading<'a, T: Into<String>>(label: T) -> impl Into<Element<'a, Message>> {
    let text = Text::new(label.into());
    Column::with_children(vec![
        Container::new(text)
            .style(Container0::default())
            .padding(Spacing::base_spacing())
            .into(),
        horizontal_rule().into(),
    ])
}

fn horizontal_rule() -> Rule<iced::Renderer> {
    Rule::horizontal(1).style(audio_processor_iced_design_system::style::Rule)
}

#[cfg(feature = "story")]
pub mod story {
    use audio_processor_iced_storybook::StoryView;

    use crate::string_vec;

    use super::*;

    pub fn default() -> Story {
        Story::default()
    }

    pub struct Story {
        audio_io_settings: View,
    }

    impl Default for Story {
        fn default() -> Self {
            let model = Model {
                audio_driver_state: DropdownModel {
                    selected_option: None,
                    options: string_vec!["Driver 1", "Driver 2"],
                },
                input_device_state: DropdownModel {
                    selected_option: None,
                    options: string_vec!["Input device 1", "Input device 2"],
                },
                output_device_state: DropdownModel {
                    selected_option: None,
                    options: string_vec!["Output device 1", "Output device 2"],
                },
                sample_rate_state: DropdownModel {
                    selected_option: None,
                    options: string_vec!["44.1kHz", "96.0kHz"],
                },
                buffer_size_state: DropdownModel {
                    selected_option: None,
                    options: string_vec!["256", "512", "1024", "2048"],
                },
            };
            Self {
                audio_io_settings: View::new(model),
            }
        }
    }

    impl StoryView<Message> for Story {
        fn update(&mut self, message: Message) -> Command<Message> {
            self.audio_io_settings.update(message)
        }

        fn view(&self) -> Element<Message> {
            self.audio_io_settings.view()
        }
    }
}
