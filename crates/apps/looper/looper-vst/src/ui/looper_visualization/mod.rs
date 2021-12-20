use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::colors::Colors;
use iced::canvas::{Frame, Stroke};
use iced::Point;
use iced_baseview::canvas::{Cursor, Geometry, Program};
use iced_baseview::{Canvas, Element, Length, Rectangle};
use looper_processor::LooperProcessorHandle;

#[derive(Debug, Clone)]
pub enum Message {}

pub struct LooperVisualizationView {
    processor_handle: Shared<LooperProcessorHandle<f32>>,
    audio_buffer: Vec<f32>,
}

impl LooperVisualizationView {
    pub fn new(processor_handle: Shared<LooperProcessorHandle<f32>>) -> Self {
        let audio_buffer = Vec::new();
        Self {
            processor_handle,
            audio_buffer,
        }
    }

    pub fn tick_visualization(&mut self) {
        if self.processor_handle.is_recording() {
            while let Some(sample) = self.processor_handle.queue.pop() {
                self.audio_buffer.push(sample);
            }
        }
    }

    pub fn clear_visualization(&mut self) {
        self.audio_buffer.clear();
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

        let num_pixels = frame.width() * 4.0;
        let step = (data.len() / num_pixels as usize).max(1);
        while index < data.len() {
            let item = data[index];
            let f_index = index as f32;
            let x = ((f_index + 1.0) / data.len() as f32) * frame.width();
            let y = (item as f32) * frame.height() / 2.0 + frame.height() / 2.0;

            path.line_to(Point::new(x, y));

            index += step;
        }

        let color = if self.processor_handle.is_recording() {
            Colors::error()
        } else {
            Colors::active_border_color()
        };
        frame.stroke(&path.build(), Stroke::default().with_color(color));

        if !data.is_empty() && !self.processor_handle.is_recording() {
            let mut playhead = iced::canvas::path::Builder::new();
            let playhead_ratio = self.processor_handle.playhead() as f32 / (data.len() as f32);
            let playhead_x = playhead_ratio * frame.width();
            playhead.move_to(Point::new(playhead_x, 0.0));
            playhead.line_to(Point::new(playhead_x, frame.height()));
            frame.stroke(
                &playhead.build(),
                Stroke::default()
                    .with_width(2.0)
                    .with_color(Colors::success()),
            );
        }

        vec![frame.into_geometry()]
    }
}
