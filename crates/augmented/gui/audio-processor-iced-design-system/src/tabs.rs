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
use std::fmt::Debug;

use iced::{
    widget::button, widget::Button, widget::Column, widget::Container, widget::Row, widget::Text,
    Background, Color, Element, Length,
};

use crate::spacing::Spacing;
use crate::style;

#[derive(Clone, Debug)]
pub enum Message<InnerMessage> {
    SetTab(usize),
    Inner(InnerMessage),
}

#[derive(Default)]
pub struct State {
    selected_tab: usize,
}

impl State {
    pub fn new() -> Self {
        State { selected_tab: 0 }
    }

    pub fn update<InnerMessage: Clone + Debug>(&mut self, message: Message<InnerMessage>) {
        if let Message::SetTab(index) = message {
            self.selected_tab = index;
        }
    }

    pub fn view<'a, InnerMessage: 'static + Clone + Debug>(
        &'a self,
        mut items: Vec<Tab<'a, InnerMessage>>,
    ) -> Element<Message<InnerMessage>> {
        let selected_tab = self.selected_tab;
        let heading = Row::with_children(
            items
                .iter()
                .enumerate()
                .map(|(index, tab)| {
                    Button::new(Text::new(tab.title.clone()).size(Spacing::small_font_size()))
                        .style(
                            style::button::Button::default()
                                .set_active(button::Appearance {
                                    background: if index == selected_tab {
                                        Some(Background::Color(Color::BLACK))
                                    } else {
                                        None
                                    },
                                    border_width: 0.0,
                                    ..style::button::button_base_style()
                                })
                                .into(),
                        )
                        .on_press(Message::SetTab(index))
                        .into()
                })
                .collect(),
        );

        let tab = items.swap_remove(self.selected_tab);
        let element = tab.content.map(Message::Inner);

        Container::new(Column::with_children(vec![
            heading.into(),
            iced::widget::rule::Rule::horizontal(1)
                .style(style::Rule)
                .into(),
            element,
        ]))
        .style(style::Container0::default())
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

pub struct Tab<'a, InnerMessage> {
    title: String,
    content: Element<'a, InnerMessage>,
}

impl<'a, InnerMessage> Tab<'a, InnerMessage> {
    pub fn new(title: impl Into<String>, content: impl Into<Element<'a, InnerMessage>>) -> Self {
        Tab {
            title: title.into(),
            content: content.into(),
        }
    }
}
