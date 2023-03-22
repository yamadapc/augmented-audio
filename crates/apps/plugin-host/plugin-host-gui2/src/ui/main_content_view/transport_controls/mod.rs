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
use iced::{widget::Button, widget::Container, widget::Row, Command, Element, Length};

use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::{colors, style};
use pause::Pause;
use stop::Stop;
use triangle::Triangle;

pub mod pause;
pub mod stop;
pub mod triangle;

pub struct TransportControlsView {
    pause: Pause,
    triangle: Triangle,
    stop: Stop,
}

#[derive(Clone, Debug)]
pub enum Message {
    Play,
    Pause,
    Stop,
    None,
}

impl Default for TransportControlsView {
    fn default() -> Self {
        Self::new()
    }
}

impl TransportControlsView {
    pub fn new() -> Self {
        TransportControlsView {
            pause: Pause::new()
                .color(colors::Colors::text())
                .hover(colors::yellow()),
            triangle: Triangle::new()
                .color(colors::Colors::text())
                .hover(colors::green()),
            stop: Stop::new()
                .color(colors::Colors::text())
                .hover(colors::red()),
        }
    }

    pub fn update(&self, _message: Message) -> Command<Message> {
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        let pause = Button::new(self.pause.view().map(|_| Message::None))
            .on_press(Message::Pause)
            .style(style::ChromelessButton.into())
            .padding(0)
            .width(Length::Fixed(Spacing::small_control_size() as f32))
            .height(Length::Fixed(Spacing::small_control_size() as f32))
            .into();
        let triangle = Button::new(self.triangle.view().map(|_| Message::None))
            .style(style::ChromelessButton.into())
            .on_press(Message::Play)
            .padding(0)
            .width(Length::Fixed(Spacing::small_control_size() as f32))
            .height(Length::Fixed(Spacing::small_control_size() as f32))
            .into();
        let square = Button::new(self.stop.view().map(|_| Message::None))
            .on_press(Message::Stop)
            .style(style::ChromelessButton.into())
            .padding(0)
            .width(Length::Fixed(Spacing::small_control_size() as f32))
            .height(Length::Fixed(Spacing::small_control_size() as f32))
            .into();

        Container::new(
            Row::with_children(vec![pause, triangle, square]).spacing(Spacing::base_spacing()),
        )
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

#[cfg(feature = "story")]
pub mod story {
    use audio_processor_iced_storybook::StoryView;

    use super::*;

    pub fn default() -> Story {
        Story {
            transport: TransportControlsView::new(),
        }
    }

    pub struct Story {
        transport: TransportControlsView,
    }

    impl StoryView<Message> for Story {
        fn update(&mut self, message: Message) -> Command<Message> {
            self.transport.update(message)
        }

        fn view(&self) -> Element<Message> {
            self.transport.view()
        }
    }
}
