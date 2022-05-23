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
use crate::MonoDelayProcessorHandle;
use audio_garbage_collector::Shared;
use audio_processor_traits::parameters::{
    AudioProcessorHandle, FloatType, ParameterSpec, ParameterType, ParameterValue,
};

pub struct GenericHandle(pub Shared<MonoDelayProcessorHandle>);

impl AudioProcessorHandle for GenericHandle {
    fn name(&self) -> String {
        "Delay".to_string()
    }

    fn parameter_count(&self) -> usize {
        2
    }

    fn get_parameter_spec(&self, index: usize) -> ParameterSpec {
        let specs = [
            ParameterSpec::new(
                "Delay".into(),
                ParameterType::Float(FloatType {
                    range: (0.01, 5.0),
                    step: None,
                }),
            ),
            ParameterSpec::new(
                "Feedback".into(),
                ParameterType::Float(FloatType {
                    range: (0.0, 1.0),
                    step: None,
                }),
            ),
        ];
        specs[index].clone()
    }

    fn get_parameter(&self, index: usize) -> Option<ParameterValue> {
        if index == 0 {
            Some(self.0.delay_time_secs.get().into())
        } else if index == 1 {
            Some(self.0.feedback.get().into())
        } else {
            None
        }
    }

    fn set_parameter(&self, index: usize, request: ParameterValue) {
        if let Ok(value) = request.try_into() {
            if index == 0 {
                self.0.delay_time_secs.set(value);
            } else if index == 1 {
                self.0.feedback.set(value);
            }
        }
    }
}
