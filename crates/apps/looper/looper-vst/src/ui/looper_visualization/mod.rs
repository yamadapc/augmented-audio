use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Deref;

use iced::{
    canvas::{Cursor, Frame, Geometry, Program, Stroke},
    Canvas, Column, Element, Length, Point, Rectangle,
};

use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::colors::Colors;
use audio_processor_traits::{AtomicF32, AudioBuffer, VecAudioBuffer};
use looper_processor::{AtomicRefCell, LoopSequencerProcessorHandle, LooperProcessorHandle};

struct LoopCache {
    // TODO: This is not the right cache key as overdubs won't render properly
    num_samples: usize,
    iterator: Vec<(usize, f32)>,
}

#[derive(Debug, Clone)]
pub enum Message {}

pub trait LooperVisualizationDrawModel {
    fn is_recording(&self) -> bool;
    fn num_samples(&self) -> usize;
    fn playhead(&self) -> usize;
    fn loop_iterator(&self) -> Vec<f32>;
}

pub struct LooperVisualizationDrawModelImpl {
    handle: Shared<LooperProcessorHandle>,
    sequencer_handle: Shared<LoopSequencerProcessorHandle>,
}

impl LooperVisualizationDrawModelImpl {
    pub fn new(
        handle: Shared<LooperProcessorHandle>,
        sequencer_handle: Shared<LoopSequencerProcessorHandle>,
    ) -> Self {
        LooperVisualizationDrawModelImpl {
            handle,
            sequencer_handle,
        }
    }
}

impl LooperVisualizationDrawModelImpl {
    fn is_recording(&self) -> bool {
        LooperProcessorHandle::is_recording(&self.handle)
    }

    fn num_samples(&self) -> usize {
        LooperProcessorHandle::num_samples(&self.handle)
    }

    fn playhead(&self) -> usize {
        let seq_playhead = LoopSequencerProcessorHandle::playhead(&self.sequencer_handle);
        seq_playhead.unwrap_or_else(|| LooperProcessorHandle::playhead(&self.handle))
    }

    fn loop_iterator(&self) -> Shared<AtomicRefCell<VecAudioBuffer<AtomicF32>>> {
        LooperProcessorHandle::looper_clip(&self.handle)
    }
}

pub struct LooperVisualizationView {
    model: LooperVisualizationDrawModelImpl,
}

impl LooperVisualizationView {
    pub fn new(model: LooperVisualizationDrawModelImpl) -> Self {
        Self { model }
    }

    pub fn tick_visualization(&mut self) {}

    pub fn clear_visualization(&mut self) {}

    pub fn view(&mut self) -> Element<()> {
        Column::with_children(vec![Canvas::new(self)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()])
        .into()
    }
}

impl Program<()> for LooperVisualizationView {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());

        let is_recording = self.model.is_recording();
        let num_samples = self.model.num_samples() as f32;

        let samples_iterator = self.model.loop_iterator();
        let samples_iterator = samples_iterator.deref().borrow();

        draw_audio_chart(
            &mut frame,
            num_samples,
            is_recording,
            samples_iterator.deref(),
        );

        if !is_recording {
            let playhead = self.model.playhead() as f32;
            draw_playhead(&mut frame, playhead, num_samples)
        }

        vec![frame.into_geometry()]
    }
}

fn draw_audio_chart<'a>(
    frame: &mut Frame,
    num_samples: f32,
    is_recording: bool,
    samples_iterator: &VecAudioBuffer<AtomicF32>,
) {
    let mut path = iced::canvas::path::Builder::new();
    let step = (samples_iterator.num_samples() / frame.width() as usize).max(1);
    for (index, samples_frame) in samples_iterator.frames().enumerate().step_by(step) {
        let item = samples_frame[0].get() + samples_frame[1].get();

        let f_index = index as f32;
        let x = ((f_index + 1.0) / num_samples) * frame.width();
        let y = item * frame.height() / 2.0 + frame.height() / 2.0;

        if !x.is_finite() {
            continue;
        }
        path.line_to(Point::new(x, y));
    }

    let color = if is_recording {
        Colors::error()
    } else {
        Colors::active_border_color()
    };
    frame.stroke(&path.build(), Stroke::default().with_color(color));
}

fn draw_playhead(frame: &mut Frame, playhead: f32, num_samples: f32) {
    let mut path = iced::canvas::path::Builder::new();
    let playhead_ratio = playhead / num_samples;
    if playhead_ratio.is_finite() {
        let playhead_x = playhead_ratio * frame.width();
        path.move_to(Point::new(playhead_x, 0.0));
        path.line_to(Point::new(playhead_x, frame.height()));
        frame.stroke(
            &path.build(),
            Stroke::default()
                .with_width(2.0)
                .with_color(Colors::success()),
        );
    }
}
