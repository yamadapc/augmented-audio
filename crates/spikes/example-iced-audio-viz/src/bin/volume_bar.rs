use std::time::Duration;

use basedrop::Shared;
use iced::canvas::{Cursor, Fill, Frame, Geometry, Program};
use iced::{
    Application, Canvas, Column, Command, Element, Length, Point, Rectangle, Settings, Size,
    Subscription,
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
