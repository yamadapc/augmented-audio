use std::sync::Arc;

use vst::buffer::AudioBuffer;

use oscillator::Oscillator;

use crate::constants::{DEPTH_PARAMETER_ID, RATE_PARAMETER_ID};
use crate::parameter_store::ParameterStore;

pub struct Processor {
    parameters: Arc<ParameterStore>,
    oscillator_left: Oscillator<f32>,
    oscillator_right: Oscillator<f32>,
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
        Oscillator::new_with_sample_rate(44100., oscillator::generators::sine_generator)
    }

    pub fn set_sample_rate(&mut self, rate: f32) {
        self.oscillator_left.set_sample_rate(rate);
        self.oscillator_right.set_sample_rate(rate);
    }

    pub fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let rate = self.parameters.value(RATE_PARAMETER_ID);
        let depth = self.parameters.value(DEPTH_PARAMETER_ID);

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
                let volume = osc.next_sample();
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
