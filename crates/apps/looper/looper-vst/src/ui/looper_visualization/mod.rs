use iced::canvas::{Frame, Stroke};
use iced::{Column, Point};
use iced_baseview::canvas::{Cursor, Geometry, Program};
use iced_baseview::{Canvas, Element, Length, Rectangle};
use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::ops::Deref;

use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::colors::Colors;
use looper_processor::LooperProcessorHandle;

struct LoopCache {
    // TODO: This is not the right cache key as overdubs won't render properly
    num_samples: usize,
    iterator: Vec<(usize, f32)>,
}

#[derive(Debug, Clone)]
pub enum Message {}

pub struct LooperVisualizationView {
    processor_handle: Shared<LooperProcessorHandle>,
    loop_cache: RefCell<Option<LoopCache>>,
}

impl LooperVisualizationView {
    pub fn new(processor_handle: Shared<LooperProcessorHandle>) -> Self {
        Self {
            processor_handle,
            loop_cache: RefCell::new(None),
        }
    }

    pub fn tick_visualization(&mut self) {}

    pub fn clear_visualization(&mut self) {}

    pub fn view(&mut self) -> Element<()> {
        Column::with_children(vec![
            // Text::new(self.processor_handle.debug()).into(),
            Canvas::new(self)
                .height(Length::Fill)
                .width(Length::Fill)
                .into(),
        ])
        .into()
    }
}

impl Program<()> for LooperVisualizationView {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());

        let is_recording = self.processor_handle.is_recording();
        let looper_state = self.processor_handle.state();
        let num_samples = looper_state.num_samples() as f32;
        let has_valid_cache = self
            .loop_cache
            .borrow()
            .as_ref()
            .map(|cache| cache.num_samples == num_samples as usize)
            .unwrap_or(false);

        let loop_cache = if !has_valid_cache {
            let step = 400;
            let iterator = looper_state
                .loop_iterator()
                .enumerate()
                .step_by(step)
                .collect();
            *self.loop_cache.borrow_mut() = Some(LoopCache {
                iterator,
                num_samples: num_samples as usize,
            });

            self.loop_cache.borrow()
        } else {
            self.loop_cache.borrow()
        };
        let samples_iterator = &loop_cache.borrow().as_ref().unwrap().iterator;

        draw_audio_chart(
            &mut frame,
            num_samples,
            is_recording,
            samples_iterator.iter(),
        );

        if !is_recording {
            let playhead = self.processor_handle.playhead() as f32;
            draw_playhead(&mut frame, playhead, num_samples)
        }

        vec![frame.into_geometry()]
    }
}

fn draw_audio_chart<'a>(
    frame: &mut Frame,
    num_samples: f32,
    is_recording: bool,
    samples_iterator: impl Iterator<Item = &'a (usize, f32)>,
) {
    let mut path = iced::canvas::path::Builder::new();
    for (index, item) in samples_iterator {
        let f_index = *index as f32;
        let x = ((f_index + 1.0) / num_samples) * frame.width();
        let y = (*item as f32) * frame.height() / 2.0 + frame.height() / 2.0;

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
