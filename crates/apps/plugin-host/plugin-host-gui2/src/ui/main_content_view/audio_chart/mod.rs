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

// use std::cell::RefCell;
use std::time::SystemTime;

use iced::{
    widget::{
        canvas::{self, Cursor, Fill, Frame, Geometry, Program},
        Canvas,
    },
    Element, Length, Point, Rectangle,
};

use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::colors::{darken_color, Colors};
use audio_processor_traits::AudioBuffer;
use augmented::gui::iced_baseview::renderer::Theme;
use plugin_host_lib::processors::running_rms_processor::RunningRMSProcessorHandle;

pub type Message = ();

pub struct AudioChart {
    handle: Shared<RunningRMSProcessorHandle>,
    rms_buffer: AudioBuffer<f32>,
    last_update: usize,
    cursor: usize,
}

impl AudioChart {
    pub fn new(handle: Shared<RunningRMSProcessorHandle>) -> Self {
        let mut rms_buffer = AudioBuffer::empty();
        rms_buffer.resize(1, 500);
        Self {
            // frame: RefCell::new(Frame::new(Size::new(100., 100.))),
            handle,
            rms_buffer,
            last_update: 0,
            cursor: 0,
        }
    }

    pub fn update(&mut self) {
        let now: usize = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as usize;
        if now - self.last_update >= 16 {
            self.last_update = now;
            let left_volume = self.handle.calculate_rms(0);
            let right_volume = self.handle.calculate_rms(1);
            self.rms_buffer
                .set(0, self.cursor, (left_volume + right_volume) / 2.0);
            self.cursor += 1;
            if self.cursor >= self.rms_buffer.num_samples() {
                self.cursor = 0;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        AudioChartView::new(&self.rms_buffer, self.cursor).view()
    }
}

// TODO - Use a view model here rather than a raw buffer
pub struct AudioChartView<'a> {
    // frame: &'a mut RefCell<Frame>,
    audio_buffer: &'a AudioBuffer<f32>,
    position: usize,
}

impl<'a> AudioChartView<'a> {
    pub fn new(
        /* frame: &'a mut RefCell<Frame>, */ audio_buffer: &'a AudioBuffer<f32>,
        position: usize,
    ) -> Self {
        Self {
            // frame,
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

impl<'a> Program<Message> for AudioChartView<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());
        let mut path = canvas::path::Builder::new();

        let num_samples = self.audio_buffer.num_samples();

        path.line_to(Point::new(frame.width(), frame.height()));
        let position = self.position;
        for sample_index in 0..num_samples {
            let read_index = ((num_samples - sample_index) + position) % num_samples;
            let sample = *self.audio_buffer.get(0, read_index); // TODO - This is "mono" just ignoring channels
            if sample.is_nan() {
                log::error!("NaN sample in volume buffer");
                return vec![];
            }

            let item = sample * 10.0;
            let f_index = sample_index as f32;
            let x2_coord = frame.width() - ((f_index + 1.0) / num_samples as f32) * frame.width();
            let y2_coord = frame.height() - item * frame.height();
            path.line_to(Point::new(x2_coord, y2_coord));
        }
        path.line_to(Point::new(0.0, frame.height()));
        path.line_to(Point::new(frame.width(), frame.height()));

        frame.fill(
            &path.build(),
            Fill::from(darken_color(Colors::border_color(), -0.3)),
        );

        vec![frame.into_geometry()]
    }
}
