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
    alignment,
    widget::{Button, Column, Container, Row, Text},
    Alignment, Command, Element, Length,
};

use audio_processor_iced_design_system::spacing::Spacing;

use crate::utils::command_message;

pub struct View {
    plugin_is_open: bool,
    input_file: Option<String>,
    audio_plugin_path: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Message {
    OpenInputFilePathPicker,
    OpenAudioPluginFilePathPicker,
    ReloadPlugin,
    OpenPluginWindow,
    ClosePluginWindow,
    FloatPluginWindow,
    SetInputFile(String),
    SetAudioPlugin(String),
}

impl View {
    pub fn new(input_file: Option<String>, audio_plugin_path: Option<String>) -> Self {
        View {
            plugin_is_open: false,
            input_file,
            audio_plugin_path,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::OpenInputFilePathPicker => {
                let result = tinyfiledialogs::open_file_dialog("Input file", "", None);
                log::info!("Got response {:?}", result);
                if let Some(path) = result {
                    return command_message(Message::SetInputFile(path));
                }
            }
            Message::OpenAudioPluginFilePathPicker => {
                let result = tinyfiledialogs::open_file_dialog("Audio plugin", "", None);
                log::info!("Got response {:?}", result);
                if let Some(path) = result {
                    return command_message(Message::SetAudioPlugin(path));
                }
            }
            Message::SetInputFile(path) => {
                self.input_file = Some(path);
            }
            Message::SetAudioPlugin(path) => {
                self.audio_plugin_path = Some(path);
            }
            Message::OpenPluginWindow => self.plugin_is_open = true,
            Message::ClosePluginWindow => self.plugin_is_open = false,
            _ => {}
        }
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        let mut children = vec![
            Self::file_picker_with_label(
                "Input file",
                &self.input_file,
                "Select input file",
                Message::OpenInputFilePathPicker,
            ),
            Self::file_picker_with_label(
                "Audio plugin",
                &self.audio_plugin_path,
                "Select audio plugin",
                Message::OpenAudioPluginFilePathPicker,
            ),
        ];

        let mut buttons_row = vec![];
        #[cfg(target_os = "macos")]
        {
            if !self.plugin_is_open {
                buttons_row.push(
                    Button::new(Text::new("Open editor"))
                        .style(audio_processor_iced_design_system::style::Button::default().into())
                        .on_press(Message::OpenPluginWindow)
                        .into(),
                );
            } else {
                buttons_row.push(
                    Button::new(Text::new("Close editor"))
                        .style(audio_processor_iced_design_system::style::Button::default().into())
                        .on_press(Message::ClosePluginWindow)
                        .into(),
                );
            }
            buttons_row.push(
                Button::new(Text::new("Float editor"))
                    .style(audio_processor_iced_design_system::style::Button::default().into())
                    .on_press(Message::FloatPluginWindow)
                    .into(),
            );
        }
        #[cfg(not(target_os = "macos"))]
        {
            children.push(
                Container::new(Text::new("Opening the editor is not supported in this OS"))
                    .center_x()
                    .width(Length::Fill)
                    .into(),
            );
        }

        buttons_row.push(
            Button::new(Text::new("Reload plugin"))
                .on_press(Message::ReloadPlugin)
                .style(audio_processor_iced_design_system::style::Button::default().into())
                .into(),
        );
        children.push(
            Container::new(Row::with_children(buttons_row).spacing(Spacing::base_spacing()))
                .center_x()
                .width(Length::Fill)
                .into(),
        );

        Column::with_children(children)
            .spacing(Spacing::base_spacing())
            .padding(Spacing::base_spacing())
            .into()
    }

    fn file_picker_with_label<'a>(
        label: impl Into<String>,
        option: &'a Option<String>,
        button_text: impl Into<String>,
        message: Message,
    ) -> Element<'a, Message> {
        Row::with_children(vec![
            Container::new(Text::new(label.into()))
                .width(Length::FillPortion(2))
                .align_x(alignment::Horizontal::Right)
                .center_y()
                .padding([0, Spacing::base_spacing()])
                .into(),
            Container::new(
                Row::with_children(vec![Button::new(Text::new(match option {
                    Some(file) => file.into(),
                    None => button_text.into(),
                }))
                .on_press(message)
                .style(audio_processor_iced_design_system::style::Button::default().into())
                .into()])
                .align_items(Alignment::Center)
                .spacing(Spacing::base_spacing()),
            )
            .center_y()
            .width(Length::FillPortion(8))
            .into(),
        ])
        .align_items(Alignment::Center)
        .into()
    }
}

#[cfg(feature = "story")]
pub mod story {
    use audio_processor_iced_storybook::StoryView;

    use super::*;

    pub fn default() -> Story {
        Story::default()
    }

    pub struct Story {
        view: View,
    }

    impl Default for Story {
        fn default() -> Self {
            Self {
                view: View::new(None, None),
            }
        }
    }

    impl StoryView<Message> for Story {
        fn update(&mut self, message: Message) -> Command<Message> {
            self.view.update(message)
        }

        fn view(&self) -> Element<Message> {
            self.view.view()
        }
    }
}
