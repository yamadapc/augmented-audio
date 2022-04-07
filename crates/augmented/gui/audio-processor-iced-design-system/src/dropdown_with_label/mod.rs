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
use iced::Command;
use iced::{alignment, pick_list, Alignment, Container, Element, Length, Row, Text};

use crate::spacing::Spacing;

type Message = String;

pub struct DropdownWithLabel {
    pick_list_state: pick_list::State<String>,
    label: String,
    options: Vec<String>,
    selected_option: Option<String>,
}

impl DropdownWithLabel {
    pub fn new(
        label: impl Into<String>,
        options: Vec<String>,
        selected_option: Option<impl Into<String>>,
    ) -> Self {
        DropdownWithLabel {
            pick_list_state: pick_list::State::default(),
            label: label.into(),
            options,
            selected_option: selected_option.map(|s| s.into()),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        self.selected_option = Some(message);
        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        Row::with_children(vec![
            Container::new(Text::new(&self.label))
                .width(Length::FillPortion(2))
                .align_x(alignment::Horizontal::Right)
                .center_y()
                .padding([0, Spacing::base_spacing()])
                .into(),
            Container::new(
                pick_list::PickList::new(
                    &mut self.pick_list_state,
                    self.options.clone(),
                    self.selected_option.clone(),
                    |option| option,
                )
                .style(crate::style::PickList)
                .padding(Spacing::base_spacing())
                .width(Length::Fill),
            )
            .width(Length::FillPortion(8))
            .into(),
        ])
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .into()
    }
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
        dropdown: DropdownWithLabel,
    }

    impl Default for Story {
        fn default() -> Self {
            let dropdown = DropdownWithLabel::new(
                "Dropdown label",
                string_vec!["Option 1", "Option 2", "Option 3"],
                Some("Option 2"),
            );
            Self { dropdown }
        }
    }

    impl StoryView<Message> for Story {
        fn update(&mut self, message: Message) -> Command<Message> {
            self.dropdown.update(message)
        }

        fn view(&mut self) -> Element<Message> {
            self.dropdown.view()
        }
    }
}
