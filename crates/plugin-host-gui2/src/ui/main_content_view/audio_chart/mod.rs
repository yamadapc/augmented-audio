use std::sync::atomic::{AtomicUsize, Ordering};

use iced::canvas::{Cursor, Fill, Frame, Geometry, Program};
use iced::{canvas, Canvas, Element, Length, Point, Rectangle};

use audio_processor_iced_design_system::colors::Colors;
use audio_processor_traits::AudioBuffer;

pub struct AudioChart<'a, Buffer: AudioBuffer> {
    audio_buffer: &'a Buffer,
    position: &'a AtomicUsize,
}

pub type Message = ();

impl<'a, Buffer: AudioBuffer<SampleType = f32>> AudioChart<'a, Buffer> {
    pub fn new(audio_buffer: &'a Buffer, position: &'a AtomicUsize) -> Self {
        Self {
            audio_buffer,
            position,
        }
    }

    pub fn view(self) -> Element<'a, Message> {
        Canvas::new(self)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}

impl<'a, Buffer: AudioBuffer<SampleType = f32>> Program<Message> for AudioChart<'a, Buffer> {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());
        let mut path = canvas::path::Builder::new();

        let num_samples = self.audio_buffer.num_samples();

        path.line_to(Point::new(frame.width(), frame.height()));
        let position = self.position.load(Ordering::Relaxed);
        for sample_index in 0..num_samples {
            let read_index = ((num_samples - sample_index) + position) % num_samples;
            let sample = *self.audio_buffer.get(0, read_index); // TODO - This is "mono" just ignoring channels

            let item = sample;
            let f_index = sample_index as f32;
            let x2_coord = frame.width() - ((f_index + 1.0) / num_samples as f32) * frame.width();
            let y2_coord = frame.height() - (item as f32) * frame.height();

            path.line_to(Point::new(x2_coord, y2_coord));
        }
        path.line_to(Point::new(0.0, frame.height()));
        path.line_to(Point::new(frame.width(), frame.height()));

        frame.fill(&path.build(), Fill::from(Colors::border_color()));

        vec![frame.into_geometry()]
    }
}
