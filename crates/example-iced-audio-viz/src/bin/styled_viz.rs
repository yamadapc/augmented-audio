use std::time::Duration;

use basedrop::Shared;
use cpal::Stream;
use iced::canvas::{Cursor, Fill, Frame, Geometry, Program, Stroke};
use iced::{
    Application, Canvas, Clipboard, Column, Command, Element, Length, Point, Rectangle, Settings,
    Size, Subscription,
};

use atomic_queue::Queue;
use audio_garbage_collector::GarbageCollector;
use audio_processor_standalone::audio_processor_start;
use circular_data_structures::CircularVec;

use example_iced_audio_viz::buffer_analyser;
use example_iced_audio_viz::buffer_analyser::BufferAnalyserProcessor;

fn main() -> iced::Result {
    log::info!("Initializing app");
    AudioVisualization::run(Settings::default())
}

struct AudioProcessingHandles {
    #[allow(dead_code)]
    garbage_collector: GarbageCollector,
    #[allow(dead_code)]
    audio_streams: (Stream, Stream),
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
        let audio_streams = audio_processor_start(processor);
        let buffer = CircularVec::with_size(5 * 4410, 0.0);

        (
            AudioVisualization {
                audio_processing_handles: AudioProcessingHandles {
                    garbage_collector,
                    audio_streams,
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

    fn update(
        &mut self,
        _message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        while let Some(sample) = self.queue_handle.pop() {
            self.buffer[self.position] = sample;
            self.position += 1;
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::time::every(Duration::from_millis(100)).map(|_| Message::Tick)
    }

    fn view(&mut self) -> Element<Self::Message> {
        let canvas = Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();
        Column::with_children(vec![canvas]).into()
    }
}

impl Program<Message> for AudioVisualization {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
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
