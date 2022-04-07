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
use augmented::audio::gc::{make_shared, Shared};
use augmented::audio::oscillator::Oscillator;
use augmented::audio::processor::AtomicF32;
use augmented::vst::buffer::AudioBuffer;
use augmented::vst::plugin::PluginParameters;

pub struct ProcessorHandleRef(pub Shared<ProcessorHandle>);

pub struct ProcessorHandle {
    frequency: AtomicF32,
}

impl PluginParameters for ProcessorHandleRef {
    fn get_parameter_label(&self, _index: i32) -> String {
        "Frequency".to_string()
    }

    fn get_parameter_text(&self, _index: i32) -> String {
        "Frequency".to_string()
    }

    fn get_parameter_name(&self, _index: i32) -> String {
        "Frequency".to_string()
    }

    fn get_parameter(&self, _index: i32) -> f32 {
        self.0.frequency.get() / 8800.0
    }

    fn set_parameter(&self, _index: i32, value: f32) {
        self.0.frequency.set(value * 8800.0);
    }
}

pub struct Processor {
    oscillator: Oscillator<f32>,
    handle: Shared<ProcessorHandle>,
}

impl Default for Processor {
    fn default() -> Self {
        Self::new()
    }
}

impl Processor {
    pub fn new() -> Self {
        let mut oscillator = Oscillator::sine(44100.0);
        oscillator.set_frequency(220.0);
        Processor {
            oscillator,
            handle: make_shared(ProcessorHandle {
                frequency: 220.0.into(),
            }),
        }
    }

    pub fn handle(&self) -> ProcessorHandleRef {
        ProcessorHandleRef(self.handle.clone())
    }

    pub fn set_sample_rate(&mut self, rate: f32) {
        self.oscillator.set_sample_rate(rate);
    }

    pub fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        self.oscillator.set_frequency(self.handle.frequency.get());

        let num_channels = buffer.input_count();
        let num_samples = buffer.samples();

        let (_input, mut output) = buffer.split();

        for sample_index in 0..num_samples {
            let out = self.oscillator.next_sample();
            for channel in 0..num_channels {
                output.get_mut(channel)[sample_index] = out;
            }
        }
    }
}
