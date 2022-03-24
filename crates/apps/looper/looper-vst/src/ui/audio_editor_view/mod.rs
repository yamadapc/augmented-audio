use std::cmp::Ordering;
use std::time::Duration;

use iced::{
    canvas::event::Status, canvas::Cursor, canvas::Event, canvas::Frame, canvas::Geometry,
    canvas::Program, canvas::Stroke, keyboard, keyboard::KeyCode, mouse, mouse::ScrollDelta,
    Canvas, Container, Element, Length, Point, Rectangle, Vector,
};

use audio_chart::{draw_rms_chart, draw_samples_chart};
use audio_processor_analysis::running_rms_processor::RunningRMSProcessor;
use audio_processor_analysis::transient_detection::stft::markers::AudioFileMarker;
use audio_processor_iced_design_system::colors::Colors;
use audio_processor_traits::{AudioProcessorSettings, SimpleAudioProcessor};

use crate::ui::style::ContainerStyle;

mod audio_chart;
pub mod story;

pub struct AudioFileModel {
    #[allow(dead_code)]
    settings: AudioProcessorSettings,
    samples: Vec<f32>,
    rms: Vec<f32>,
}

impl AudioFileModel {
    fn from_buffer(settings: AudioProcessorSettings, samples: Vec<f32>) -> Self {
        let max_sample = samples
            .iter()
            .cloned()
            .max_by(|f1, f2| f1.partial_cmp(f2).unwrap_or(Ordering::Equal))
            .unwrap_or(1.0);
        let samples: Vec<f32> = samples.iter().map(|f| f / max_sample).collect();
        let mut rms_processor = RunningRMSProcessor::new_with_duration(
            audio_garbage_collector::handle(),
            Duration::from_millis(30),
        );

        let mut rms_samples = vec![];
        rms_processor.s_prepare(settings);
        for sample in samples.iter() {
            rms_processor.s_process_frame(&mut [*sample]);
            rms_samples.push(rms_processor.handle().calculate_rms(0));
        }

        Self {
            settings,
            samples,
            rms: rms_samples,
        }
    }

    fn samples_len(&self) -> usize {
        self.samples.len()
    }

    fn samples(&self) -> impl Iterator<Item = &f32> {
        self.samples.iter()
    }

    fn rms_len(&self) -> usize {
        self.rms.len()
    }

    fn rms(&self) -> impl Iterator<Item = &f32> {
        self.rms.iter()
    }
}

enum ChartMode {
    Samples,
    RMS,
}

struct VisualizationModel {
    zoom_x: f32,
    zoom_y: f32,
    offset: f32,
    chart_mode: ChartMode,
}

impl Default for VisualizationModel {
    fn default() -> Self {
        Self {
            zoom_x: 1.0,
            zoom_y: 1.0,
            offset: 0.0,
            chart_mode: ChartMode::Samples,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Default)]
pub struct AudioEditorView {
    audio_file_model: Option<AudioFileModel>,
    visualization_model: VisualizationModel,
    markers: Vec<AudioFileMarker>,
    shift_down: bool,
}

impl AudioEditorView {
    pub fn update(&mut self, _message: Message) {}

    pub fn view(&mut self) -> Element<Message> {
        // let empty_state = Text::new("Drop a file").into();
        Container::new(Canvas::new(self).width(Length::Fill).height(Length::Fill))
            .center_x()
            .center_y()
            .style(ContainerStyle)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn on_scroll_zoom(&mut self, x: f32, y: f32) {
        self.visualization_model.zoom_x += x;
        self.visualization_model.zoom_x = self.visualization_model.zoom_x.min(100.0).max(1.0);
        self.visualization_model.zoom_y += y;
        self.visualization_model.zoom_y = self.visualization_model.zoom_y.min(2.0).max(0.2);
    }

    fn on_scroll(&mut self, bounds: Rectangle, x: f32) {
        let size = bounds.size();
        let width = size.width * self.visualization_model.zoom_x;
        let offset = (self.visualization_model.offset + x)
            .max(0.0)
            .min(width - size.width);
        self.visualization_model.offset = offset;
    }

    fn toggle_chart_mode(&mut self) {
        self.visualization_model.chart_mode = match self.visualization_model.chart_mode {
            ChartMode::RMS => ChartMode::Samples,
            ChartMode::Samples => ChartMode::RMS,
        };
    }
}

impl Program<Message> for AudioEditorView {
    fn update(
        &mut self,
        event: Event,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> (Status, Option<Message>) {
        match event {
            Event::Mouse(mouse::Event::WheelScrolled {
                delta: ScrollDelta::Pixels { x, y },
            }) => {
                if self.shift_down {
                    self.on_scroll_zoom(x, y);
                    (Status::Captured, None)
                } else {
                    self.on_scroll(bounds, x);
                    (Status::Captured, None)
                }
            }
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                self.shift_down = modifiers.shift();
                (Status::Ignored, None)
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key_code: KeyCode::M,
                modifiers,
            }) => {
                if modifiers.command() {
                    self.toggle_chart_mode();
                    (Status::Captured, None)
                } else {
                    (Status::Ignored, None)
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key_code: KeyCode::Equals,
                modifiers,
            }) => {
                if modifiers.shift() && modifiers.command() {
                    self.visualization_model.zoom_y += 0.1;
                    self.visualization_model.zoom_y =
                        self.visualization_model.zoom_y.min(2.0).max(0.2);
                    (Status::Captured, None)
                } else if modifiers.command() {
                    self.visualization_model.zoom_x *= 10.0;
                    self.visualization_model.zoom_x =
                        self.visualization_model.zoom_x.min(100.0).max(1.0);
                    (Status::Captured, None)
                } else {
                    (Status::Ignored, None)
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key_code: KeyCode::Minus,
                modifiers,
            }) => {
                if modifiers.shift() && modifiers.command() {
                    self.visualization_model.zoom_y -= 0.1;
                    self.visualization_model.zoom_y =
                        self.visualization_model.zoom_y.min(2.0).max(0.2);
                    (Status::Captured, None)
                } else if modifiers.command() {
                    self.visualization_model.zoom_x /= 10.0;
                    self.visualization_model.zoom_x =
                        self.visualization_model.zoom_x.min(100.0).max(1.0);
                    (Status::Captured, None)
                } else {
                    (Status::Ignored, None)
                }
            }
            _ => (Status::Ignored, None),
        }
    }

    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let zoom_x = self.visualization_model.zoom_x;
        let zoom_y = self.visualization_model.zoom_y;
        let mut frame = Frame::new(bounds.size());
        if let Some(audio_file_model) = &self.audio_file_model {
            let width = frame.width() * zoom_x;
            let height = frame.height() * zoom_y;
            let offset = self.visualization_model.offset.min(width - frame.width());
            frame.translate(Vector::from([-offset, 0.0]));

            match self.visualization_model.chart_mode {
                ChartMode::Samples => {
                    let num_samples = audio_file_model.samples_len() as f32;

                    draw_samples_chart(
                        &mut frame,
                        width,
                        height,
                        offset,
                        num_samples,
                        audio_file_model.samples().cloned(),
                    );

                    let markers = &self.markers;
                    draw_markers(&mut frame, num_samples, width, markers);
                }
                ChartMode::RMS => {
                    draw_rms_chart(
                        &mut frame,
                        width,
                        height,
                        offset,
                        audio_file_model.rms_len() as f32,
                        audio_file_model.rms().cloned(),
                    );
                }
            }
        }

        vec![frame.into_geometry()]
    }
}

fn draw_markers(
    frame: &mut Frame,
    num_samples: f32,
    total_width: f32,
    markers: &Vec<AudioFileMarker>,
) {
    for marker in markers {
        let mut path = iced::canvas::path::Builder::new();
        let x = (marker.position_samples as f32 / num_samples) * total_width;
        path.move_to(Point::new(x, 0.0));
        path.line_to(Point::new(x, frame.height()));
        frame.stroke(&path.build(), Stroke::default().with_color(Colors::error()));
    }
}
