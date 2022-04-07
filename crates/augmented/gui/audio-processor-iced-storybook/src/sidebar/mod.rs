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
use crate::Options;
use audio_processor_iced_design_system::menu_list;
use audio_processor_iced_design_system::style;
use iced::{Command, Element, Row, Rule, Text};

#[derive(Debug, Clone)]
pub enum Message {
    MenuList(menu_list::Message<SelectedStory>),
}

#[derive(Debug, Clone)]
pub struct SelectedStory {
    pub id: String,
}

pub struct SidebarView {
    menu_list: menu_list::State<String, SelectedStory>,
}

impl SidebarView {
    pub fn new<Inner>(options: &Options<Inner>) -> Self {
        let items = options
            .stories
            .iter()
            .map(|story| {
                (
                    story.title.clone(),
                    SelectedStory {
                        id: story.id.clone(),
                    },
                )
            })
            .collect();
        Self {
            menu_list: menu_list::State::new(items, None),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::MenuList(message) => {
                self.menu_list.update(message);
            }
        }
        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        let container = self
            .menu_list
            .view(|text| Text::new(&*text).into())
            .map(Message::MenuList);

        let rule = Rule::vertical(1).style(style::Rule).into();
        return Row::with_children(vec![container, rule]).into();
    }
}
