use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::colors::Colors;
use iced::canvas::{Frame, Stroke};
use iced::Point;
use iced_baseview::canvas::{Cursor, Geometry, Program};
use iced_baseview::{Canvas, Element, Length, Rectangle};
use looper_processor::LooperProcessorHandle;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum Message {}

pub struct LooperVisualizationView {
    processor_handle: Shared<LooperProcessorHandle<f32>>,
    audio_buffer: Vec<f32>,
    cursor: usize,
}

impl LooperVisualizationView {
    pub fn new(processor_handle: Shared<LooperProcessorHandle<f32>>) -> Self {
        let mut audio_buffer = Vec::new();
        audio_buffer.resize(
            (Duration::from_secs(1).as_secs_f32() * 44100.) as usize,
            0.0,
        );
        Self {
            processor_handle,
            audio_buffer,
            cursor: 0,
        }
    }

    pub fn tick_visualization(&mut self) {
        if self.processor_handle.is_recording() {
            while let Some(sample) = self.processor_handle.queue.pop() {
                self.audio_buffer[self.cursor] = sample;
                self.cursor += 1;
                if self.cursor >= self.audio_buffer.len() {
                    self.cursor = 0;
                }
            }
        }
    }

    pub fn view(&mut self) -> Element<()> {
        Canvas::new(self)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}

impl Program<()> for LooperVisualizationView {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());

        let data = &self.audio_buffer;
        let mut index = 0;
        let mut path = iced::canvas::path::Builder::new();

        while index < data.len() {
            let item = data[index];
            let f_index = index as f32;
            let x = ((f_index + 1.0) / data.len() as f32) * frame.width();
            let y = (item as f32) * frame.height() / 2.0 + frame.height() / 2.0;

            path.line_to(Point::new(x, y));

            index += 10;
        }

        let color = if self.processor_handle.is_recording() {
            Colors::error()
        } else {
            Colors::active_border_color()
        };
        frame.stroke(&path.build(), Stroke::default().with_color(color));

        vec![frame.into_geometry()]
    }
}
