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
use std::sync::Arc;

use vst::buffer::AudioBuffer;

use audio_parameter_store::ParameterStore;
use audio_processor_traits::{AudioProcessor, AudioProcessorSettings};
use augmented_oscillator::Oscillator;

use crate::constants::{DEPTH_PARAMETER_ID, PHASE_PARAMETER_ID, RATE_PARAMETER_ID};

pub struct Processor {
    parameters: Arc<ParameterStore>,
    oscillator_left: Oscillator<f32>,
    oscillator_right: Oscillator<f32>,
}

impl AudioProcessor for Processor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.set_sample_rate(settings.sample_rate());
    }

    fn process<BufferType: audio_processor_traits::AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        buffer: &mut BufferType,
    ) {
        let rate = self.parameters.value(RATE_PARAMETER_ID);
        let depth = self.parameters.value(DEPTH_PARAMETER_ID) / 100.0;
        let phase_offset = self.parameters.value(PHASE_PARAMETER_ID) / 360.0;

        self.oscillator_left.set_frequency(rate);
        self.oscillator_right.set_frequency(rate);

        let num_channels = buffer.num_channels();
        let num_samples = buffer.num_samples();

        for channel in 0..num_channels {
            let osc = if channel == 0 {
                &mut self.oscillator_left
            } else {
                &mut self.oscillator_right
            };

            for sample_index in 0..num_samples {
                let volume = if channel == 0 {
                    osc.next_sample()
                } else {
                    let value = osc.value_for_phase(osc.phase() + phase_offset);
                    osc.tick();
                    value
                };

                let dry_signal = buffer.get(channel % num_samples, sample_index);
                let wet_signal = volume * *dry_signal;
                // mixed_signal = (1.0 - depth) * dry + depth * wet
                // mixed_signal = dry - dry * depth + depth * wet
                let mixed_signal = dry_signal + depth * (wet_signal - dry_signal);

                let output = buffer.get_mut(channel % num_samples, sample_index);
                *output = mixed_signal;
            }
        }
    }
}

impl Processor {
    pub fn new(parameters: Arc<ParameterStore>) -> Self {
        Processor {
            parameters,
            oscillator_left: Processor::build_oscillator(),
            oscillator_right: Processor::build_oscillator(),
        }
    }

    fn build_oscillator() -> Oscillator<f32> {
        Oscillator::new_with_sample_rate(44100., augmented_oscillator::generators::sine_generator)
    }

    pub fn set_sample_rate(&mut self, rate: f32) {
        self.oscillator_left.set_sample_rate(rate);
        self.oscillator_right.set_sample_rate(rate);
    }

    pub fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let rate = self.parameters.value(RATE_PARAMETER_ID);
        let depth = self.parameters.value(DEPTH_PARAMETER_ID) / 100.0;
        let phase_offset = self.parameters.value(PHASE_PARAMETER_ID) / 360.0;

        self.oscillator_left.set_frequency(rate);
        self.oscillator_right.set_frequency(rate);

        let num_channels = buffer.input_count();
        let num_samples = buffer.samples();
        let (input, mut output) = buffer.split();

        for channel in 0..num_channels {
            let osc = if channel == 0 {
                &mut self.oscillator_left
            } else {
                &mut self.oscillator_right
            };

            let input_samples = input.get(channel % input.len());
            let output_samples = output.get_mut(channel % output.len());

            for sample_index in 0..num_samples {
                let volume = if channel == 0 {
                    osc.next_sample()
                } else {
                    let value = osc.value_for_phase(osc.phase() + phase_offset);
                    osc.tick();
                    value
                };

                let dry_signal = input_samples[sample_index];
                let wet_signal = volume * input_samples[sample_index];
                // mixed_signal = (1.0 - depth) * dry + depth * wet
                // mixed_signal = dry - dry * depth + depth * wet
                let mixed_signal = dry_signal + depth * (wet_signal - dry_signal);

                output_samples[sample_index] = mixed_signal;
            }
        }
    }
}
