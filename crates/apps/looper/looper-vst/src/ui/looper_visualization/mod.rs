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
use std::ops::Deref;

use iced::{
    widget::canvas::{Cursor, Frame, Geometry, Program, Stroke},
    widget::Canvas,
    widget::Column,
    Element, Length, Point, Rectangle,
};

use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::colors::Colors;
use audio_processor_traits::{AtomicF32, AudioBuffer};
use looper_processor::{AtomicRefCell, LoopShufflerProcessorHandle, LooperProcessorHandle};

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
    sequencer_handle: Shared<LoopShufflerProcessorHandle>,
}

impl LooperVisualizationDrawModelImpl {
    pub fn new(
        handle: Shared<LooperProcessorHandle>,
        sequencer_handle: Shared<LoopShufflerProcessorHandle>,
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
        let seq_playhead = LoopShufflerProcessorHandle::playhead(&self.sequencer_handle);
        seq_playhead.unwrap_or_else(|| LooperProcessorHandle::playhead(&self.handle))
    }

    fn loop_iterator(&self) -> Shared<AtomicRefCell<AudioBuffer<AtomicF32>>> {
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

    pub fn view(&self) -> Element<()> {
        Column::with_children(vec![Canvas::new(self)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()])
        .into()
    }
}

impl Program<()> for LooperVisualizationView {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
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

fn draw_audio_chart(
    frame: &mut Frame,
    num_samples: f32,
    is_recording: bool,
    samples_iterator: &AudioBuffer<AtomicF32>,
) {
    if samples_iterator.num_channels() == 0 {
        return;
    }
    let mut path = iced::widget::canvas::path::Builder::new();
    let step = (samples_iterator.num_samples() / frame.width() as usize).max(1);
    for sample_index in (0..samples_iterator.num_samples()).step_by(step) {
        let item = samples_iterator.get(0, sample_index).get()
            + samples_iterator.get(1, sample_index).get();

        let f_index = sample_index as f32;
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
    let mut path = iced::widget::canvas::path::Builder::new();
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
