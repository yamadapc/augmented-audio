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
use std::time::Duration;

use basedrop::Shared;
use iced::widget::canvas::{Cursor, Fill, Frame, Geometry, Program};
use iced::{
    widget::Canvas, widget::Column, Application, Command, Element, Length, Point, Rectangle,
    Settings, Size, Subscription,
};

use audio_garbage_collector::GarbageCollector;
use audio_processor_standalone::{audio_processor_start, StandaloneHandles};
use example_iced_audio_viz::volume_analyser::{VolumeAnalyser, VolumeAnalyserHandle};

fn main() -> iced::Result {
    log::info!("Initializing app");
    AudioVisualization::run(Settings::default())
}

struct AudioProcessingHandles {
    #[allow(dead_code)]
    garbage_collector: GarbageCollector,
    #[allow(dead_code)]
    standalone_handles: StandaloneHandles,
}

struct AudioVisualization {
    #[allow(dead_code)]
    audio_processing_handles: AudioProcessingHandles,
    handle: Shared<VolumeAnalyserHandle>,
    volume_left: f32,
    volume_right: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
}

impl Application for AudioVisualization {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let garbage_collector = GarbageCollector::default();
        let processor = VolumeAnalyser::new(garbage_collector.handle(), Duration::from_millis(8));
        let handle = processor.handle().clone();
        let standalone_handles = audio_processor_start(processor);

        (
            AudioVisualization {
                audio_processing_handles: AudioProcessingHandles {
                    garbage_collector,
                    standalone_handles,
                },
                handle,
                volume_left: 0.0,
                volume_right: 0.0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("ICED Audio Viz")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        let (left, right) = self.handle.volume();
        self.volume_left = left;
        self.volume_right = right;
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::time::every(Duration::from_millis(16)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<Self::Message> {
        let canvas = Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();
        Column::with_children(vec![canvas]).into()
    }

    type Theme = iced::Theme;
}

impl Program<Message> for AudioVisualization {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());

        {
            let volume = self.volume_left;
            let magnitude = volume * frame.height() * 10.0;
            let y_coord = frame.height();
            frame.fill_rectangle(
                Point::new(30.0, y_coord),
                Size::new(30.0, magnitude),
                Fill::default(),
            );
        }

        {
            let volume = self.volume_right;
            let magnitude = volume * frame.height() * 10.0;
            let y_coord = frame.height();
            frame.fill_rectangle(
                Point::new(70.0, y_coord),
                Size::new(30.0, magnitude),
                Fill::default(),
            );
        }

        vec![frame.into_geometry()]
    }
}
