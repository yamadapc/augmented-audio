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

use atomic_queue::Queue;
use audio_garbage_collector::GarbageCollector;
use audio_processor_standalone::{audio_processor_start, StandaloneHandles};
use circular_data_structures::CircularVec;
use example_iced_audio_viz::buffer_analyser::BufferAnalyserProcessor;

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
    queue_handle: Shared<Queue<f32>>,
    buffer: CircularVec<f32>,
    position: usize,
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
        let processor = BufferAnalyserProcessor::new(garbage_collector.handle());
        let queue_handle = processor.queue();
        let standalone_handles = audio_processor_start(processor);
        let buffer = CircularVec::with_size(5 * 4410, 0.0);

        (
            AudioVisualization {
                audio_processing_handles: AudioProcessingHandles {
                    garbage_collector,
                    standalone_handles,
                },
                queue_handle,
                buffer,
                position: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("ICED Audio Viz")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        while let Some(sample) = self.queue_handle.pop() {
            self.buffer[self.position] = sample;
            self.position += 1;
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::time::every(Duration::from_millis(100)).map(|_| Message::Tick)
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

        let data = self.buffer.inner();
        let mut index = 0;

        while index < data.len() {
            let item = data[index];
            let magnitude = item.abs();
            let f_index = index as f32;
            let x_coord = (f_index / data.len() as f32) * frame.width();
            let magnitude = (magnitude as f32) * frame.height() / 2.0 * 5.0;
            let y_coord = frame.height() / 2.0 - magnitude / 2.0;

            frame.fill_rectangle(
                Point::new(x_coord, y_coord),
                Size::new(3.0, magnitude),
                Fill::default(),
            );

            index += 100;
        }

        vec![frame.into_geometry()]
    }
}
