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
use iced_audio::{knob, IntRange, NormalParam};

pub struct ParameterViewModel<ParameterId> {
    pub id: ParameterId,
    pub name: String,
    pub suffix: String,
    pub value: f32,
    pub knob_state: knob::State,
    pub int_range: Option<IntRange>,
    pub range: (f32, f32),
}

impl<ParameterId> ParameterViewModel<ParameterId> {
    pub fn new(
        id: ParameterId,
        name: impl Into<String>,
        suffix: impl Into<String>,
        value: f32,
        range: (f32, f32),
    ) -> Self {
        ParameterViewModel {
            id,
            name: name.into(),
            suffix: suffix.into(),
            value,
            knob_state: knob::State::new(NormalParam::from(
                (value - range.0) / (range.1 - range.0),
            )),
            int_range: None,
            range,
        }
    }

    pub fn snap_int(mut self) -> Self {
        let range = IntRange::new(self.range.0 as i32, self.range.1 as i32);
        self.int_range = Some(range);
        self
    }
}
