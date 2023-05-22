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

use audio_garbage_collector::Shared;
use audio_processor_traits::atomic_float::AtomicFloatRepresentable;
use audio_processor_traits::parameters::{
    AudioProcessorHandle, FloatType, ParameterSpec, ParameterType, ParameterValue,
};
use audio_processor_traits::Float;

use crate::BitCrusherHandleImpl;

pub struct BitCrusherHandleRef<ST: AtomicFloatRepresentable + Float = f32>(
    Shared<BitCrusherHandleImpl<ST>>,
);

impl<ST> BitCrusherHandleRef<ST>
where
    ST: AtomicFloatRepresentable + Float,
{
    pub fn new(inner: Shared<BitCrusherHandleImpl<ST>>) -> Self {
        BitCrusherHandleRef(inner)
    }
}

impl<ST> AudioProcessorHandle for BitCrusherHandleRef<ST>
where
    ST: AtomicFloatRepresentable + Float,
    ST: From<f32>,
    f32: From<ST>,
    ST::AtomicType: Send + Sync,
{
    fn name(&self) -> String {
        "Bit-crusher".to_string()
    }

    fn parameter_count(&self) -> usize {
        1
    }

    fn get_parameter_spec(&self, _index: usize) -> ParameterSpec {
        ParameterSpec::new(
            "Bit rate".into(),
            ParameterType::Float(FloatType {
                range: (100.0, self.0.sample_rate()),
                step: None,
            }),
        )
    }

    fn get_parameter(&self, _index: usize) -> Option<ParameterValue> {
        Some(ParameterValue::Float {
            value: self.0.bit_rate(),
        })
    }

    fn set_parameter(&self, _index: usize, request: ParameterValue) {
        let ParameterValue::Float { value } = request;
        self.0.set_bit_rate(value);
    }
}
