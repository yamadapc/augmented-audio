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
use iced::{Background, Container, Element, Length, Text};

use audio_processor_iced_design_system::colors::Colors;
use audio_processor_iced_design_system::spacing::Spacing;

#[derive(Clone, Debug)]
pub enum State {
    Success,
    Warning,
    Error,
    Idle,
}

#[derive(Clone, Debug)]
pub struct StatusBar {
    message: String,
    state: State,
}

impl StatusBar {
    pub fn new(message: impl Into<String>, state: State) -> Self {
        StatusBar {
            message: message.into(),
            state,
        }
    }

    pub fn view(self) -> Element<'static, ()> {
        Container::new(Text::new(&self.message).size(Spacing::small_font_size()))
            .center_y()
            .padding([0, Spacing::base_spacing()])
            .style(StatusContainer { state: self.state })
            .height(Length::Units(20))
            .width(Length::Fill)
            .into()
    }
}

struct StatusContainer {
    state: State,
}

impl iced::container::StyleSheet for StatusContainer {
    fn style(&self) -> iced::container::Style {
        iced::container::Style {
            text_color: Some(Colors::text()),
            background: match self.state {
                State::Success => Some(Background::Color(Colors::success())),
                State::Error => Some(Background::Color(Colors::error())),
                State::Warning => Some(Background::Color(Colors::warning())),
                State::Idle => Some(Background::Color(Colors::idle())),
            },
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Colors::border_color(),
        }
    }
}

#[cfg(feature = "story")]
pub mod story {
    use iced::Column;

    use audio_processor_iced_storybook::StoryView;

    use super::*;

    pub fn default() -> impl StoryView<()> {
        || {
            Column::with_children(vec![
                StatusBar::new("Success message!", State::Success).view(),
                StatusBar::new("Warning message!", State::Warning).view(),
                StatusBar::new("Error message!", State::Error).view(),
                StatusBar::new("Idle message!", State::Idle).view(),
            ])
            .into()
        }
    }
}
