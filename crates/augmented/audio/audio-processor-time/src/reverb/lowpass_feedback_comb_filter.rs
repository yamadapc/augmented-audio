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
use super::utils::undenormalize;
use crate::reverb::utils::make_vec;

pub struct LowpassFeedbackCombFilter {
    feedback: f32,
    filter_store: f32,
    damp1: f32,
    damp2: f32,
    buffer: Vec<f32>,
    buffer_cursor: usize,
}

impl LowpassFeedbackCombFilter {
    pub fn new(size: usize) -> Self {
        Self {
            feedback: 0.0,
            filter_store: 0.0,
            damp1: 0.0,
            damp2: 0.0,
            buffer: make_vec(size),
            buffer_cursor: 0,
        }
    }

    pub fn set_damp(&mut self, value: f32) {
        self.damp1 = value;
        self.damp2 = 1.0 - value;
    }

    pub fn set_feedback(&mut self, value: f32) {
        self.feedback = value;
    }

    pub fn process(&mut self, sample: f32) -> f32 {
        let output = undenormalize(self.buffer[self.buffer_cursor]);

        self.filter_store = undenormalize(output * self.damp2 + self.filter_store * self.damp1);

        self.buffer[self.buffer_cursor] = sample + self.filter_store * self.feedback;
        self.buffer_cursor += 1;
        if self.buffer_cursor >= self.buffer.len() {
            self.buffer_cursor = 0;
        }

        output
    }
}
