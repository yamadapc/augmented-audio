use basedrop::Shared;
use cpal::Stream;
use iced::{
    Application, Canvas, Clipboard, Command, Element, Length, Point, Rectangle, Settings,
    Subscription,
};

use atomic_queue::Queue;
use audio_garbage_collector::GarbageCollector;
use audio_processor_standalone::audio_processor_start;

use crate::buffer_analyser::BufferAnalyserProcessor;
use circular_data_structures::CircularVec;
use iced::canvas::{Cursor, Frame, Geometry, Program, Stroke};
use std::time::Duration;

mod buffer_analyser;

fn main() -> iced::Result {
    log::info!("Initializing app");
    AudioVisualization::run(Settings::default())
}

struct AudioVisualization {
    #[allow(dead_code)]
    garbage_collector: GarbageCollector,
    #[allow(dead_code)]
    queue_handle: Shared<Queue<f32>>,
    #[allow(dead_code)]
    audio_streams: (Stream, Stream),
    #[allow(dead_code)]
    buffer: CircularVec<f32>,
    #[allow(dead_code)]
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
                garbage_collector,
                queue_handle,
                audio_streams,
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

    fn view<'a>(&'a mut self) -> Element<'a, Self::Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl Program<Message> for AudioVisualization {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());
        let mut path = iced::canvas::path::Builder::new();

        let data = self.buffer.inner();
        let mut prev = data[0];
        let mut index = 0;

        while index < data.len() {
            let item = data[index];
            let f_index = index as f32;
            let x_coord = (f_index / data.len() as f32) * frame.width();
            let x2_coord = ((f_index + 1.0) / data.len() as f32) * frame.width();
            let y_coord = (prev as f32) * frame.height() / 2.0 + frame.height() / 2.0;
            let y2_coord = (item as f32) * frame.height() / 2.0 + frame.height() / 2.0;

            path.move_to(Point::new(x_coord, y_coord));
            path.line_to(Point::new(x2_coord, y2_coord));

            prev = item;
            index += 10;
        }

        frame.stroke(&path.build(), Stroke::default());

        vec![frame.into_geometry()]
    }
}
