use std::cmp::Ordering;
use std::time::Duration;

use iced::canvas::event::Status;
use iced::canvas::Cursor;
use iced::canvas::Event;
use iced::canvas::Fill;
use iced::canvas::Frame;
use iced::canvas::Geometry;
use iced::canvas::Program;
use iced::canvas::Stroke;
use iced::mouse;
use iced::mouse::ScrollDelta;
use iced::Canvas;
use iced::Column;
use iced::Container;
use iced::Element;
use iced::Length;
use iced::Point;
use iced::Rectangle;
use iced::Text;

use audio_processor_iced_design_system::colors::Colors;
use audio_processor_traits::{AudioProcessor, InterleavedAudioBuffer, SimpleAudioProcessor};

use crate::ui::style::ContainerStyle;

pub struct AudioFileModel {
    samples: Vec<f32>,
}

impl AudioFileModel {
    fn from_buffer(samples: Vec<f32>) -> Self {
        let max_sample = samples
            .iter()
            .cloned()
            .max_by(|f1, f2| f1.partial_cmp(f2).unwrap_or(Ordering::Equal))
            .unwrap_or(1.0);
        let samples: Vec<f32> = samples.iter().map(|f| f / max_sample).collect();
        let mut rms_processor =
            audio_processor_analysis::running_rms_processor::RunningRMSProcessor::new_with_duration(
                audio_garbage_collector::handle(),
                Duration::from_millis(3),
            );

        let mut rms_samples = vec![];
        rms_processor.prepare(Default::default());
        for (index, sample) in samples.iter().enumerate() {
            rms_processor.s_process_frame(&mut [*sample]);
            if index % 400 == 0 {
                rms_samples.push(rms_processor.handle().calculate_rms(0));
            }
        }

        Self {
            samples: rms_samples,
        }
    }

    fn len(&self) -> usize {
        self.samples.len()
    }

    fn samples(&self) -> impl Iterator<Item = &f32> {
        self.samples.iter()
    }
}

struct VisualizationModel {
    zoom: f32,
    offset: f32,
}

impl Default for VisualizationModel {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            offset: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Default)]
pub struct AudioEditorView {
    audio_file_model: Option<AudioFileModel>,
    visualization_model: VisualizationModel,
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
}

impl Program<Message> for AudioEditorView {
    fn update(
        &mut self,
        event: Event,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> (Status, Option<Message>) {
        match event {
            Event::Mouse(mouse::Event::WheelScrolled {
                delta: ScrollDelta::Pixels { x, .. },
            }) => {
                self.visualization_model.zoom += x;
                self.visualization_model.zoom = self.visualization_model.zoom.min(100.0).max(0.5);
                (Status::Captured, None)
            }
            _ => (Status::Ignored, None),
        }
    }

    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let zoom = self.visualization_model.zoom;
        let mut frame = Frame::new(bounds.size());
        if let Some(audio_file_model) = &self.audio_file_model {
            let width = frame.width() * zoom;
            draw_audio_chart(
                &mut frame,
                width,
                audio_file_model.len() as f32,
                audio_file_model.samples().cloned(),
            );
        }
        vec![frame.into_geometry()]
    }
}

fn draw_audio_chart<'a>(
    frame: &mut Frame,
    width: f32,
    num_samples: f32,
    samples_iterator: impl Iterator<Item = f32>,
) {
    let color = Colors::active_border_color();
    let step_size = ((num_samples / (width * 2.0)) as usize).max(1);
    let mut samples = samples_iterator.collect::<Vec<f32>>();

    let mut path = iced::canvas::path::Builder::new();
    path.line_to(Point::new(0.0, frame.height() / 2.0));
    for (index, item) in samples.iter().enumerate().step_by(step_size) {
        let f_index = index as f32;
        let x = (f_index / num_samples) * width;
        let y = (*item as f32) * frame.height() / 2.0 + frame.height() / 2.0;

        if x > frame.width() {
            break;
        }

        if !x.is_finite() {
            continue;
        }

        path.line_to(Point::new(x, y));
    }
    path.line_to(Point::new(frame.width(), frame.height() / 2.0));
    path.line_to(Point::new(0.0, frame.height() / 2.0));
    frame.fill(&path.build(), Fill::from(color));

    let mut path = iced::canvas::path::Builder::new();
    path.line_to(Point::new(0.0, frame.height() / 2.0));
    for (index, item) in samples.iter().enumerate().step_by(step_size) {
        let f_index = index as f32;
        let x = (f_index / num_samples) * width;
        let y = (-item as f32) * frame.height() / 2.0 + frame.height() / 2.0;

        if x > frame.width() {
            break;
        }

        if !x.is_finite() {
            continue;
        }

        path.line_to(Point::new(x, y));
    }
    path.line_to(Point::new(frame.width(), frame.height() / 2.0));
    path.line_to(Point::new(0.0, frame.height() / 2.0));
    frame.fill(&path.build(), Fill::from(color));
}

pub mod story {
    use audio_processor_testing_helpers::relative_path;
    use iced::Command;

    use audio_processor_file::AudioFileProcessor;
    use audio_processor_iced_storybook::StoryView;
    use audio_processor_traits::AudioProcessorSettings;

    use super::*;

    pub fn default() -> Story {
        Story::default()
    }

    pub struct Story {
        editor: AudioEditorView,
    }

    impl Default for Story {
        fn default() -> Self {
            let mut editor = AudioEditorView::default();
            let settings = AudioProcessorSettings::default();
            log::info!("Reading audio file");
            let audio_file_buffer = get_example_file_buffer(settings);
            log::info!("Building editor model");
            editor.audio_file_model = Some(AudioFileModel::from_buffer(audio_file_buffer));
            log::info!("Starting");
            Self { editor }
        }
    }

    fn get_example_file_buffer(settings: AudioProcessorSettings) -> Vec<f32> {
        let mut processor = AudioFileProcessor::from_path(
            audio_garbage_collector::handle(),
            settings,
            &relative_path!("../../../../input-files/synthetizer-loop.mp3"),
        )
        .unwrap();
        processor.prepare(settings);
        let channels = processor.buffer().clone();
        let mut output = vec![];
        for (s1, s2) in channels[0].iter().zip(channels[1].clone()) {
            output.push(s1 + s2);
        }
        output
    }

    impl StoryView<Message> for Story {
        fn update(&mut self, message: Message) -> Command<Message> {
            self.editor.update(message);
            Command::none()
        }

        fn view(&mut self) -> Element<Message> {
            self.editor.view()
        }
    }
}
