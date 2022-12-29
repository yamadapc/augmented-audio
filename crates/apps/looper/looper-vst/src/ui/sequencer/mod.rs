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
use iced::Element;
use iced_baseview::widget::{Button, Row, Text};

use audio_processor_iced_design_system::style;

#[derive(Debug, Clone)]
pub enum Message {}

pub struct SequencerView {}

impl Default for SequencerView {
    fn default() -> Self {
        SequencerView {}
    }
}

impl SequencerView {
    pub fn view(&self) -> Element<Message> {
        use iced::*;

        let buttons = (0..8)
            .map(|i| {
                Button::new(Text::new(format!("{}", i + 1)))
                    .style(style::Button::default().into())
                    .width(Length::Fill)
                    .into()
            })
            .collect();

        Row::with_children(buttons).width(Length::Fill).into()
    }
}
