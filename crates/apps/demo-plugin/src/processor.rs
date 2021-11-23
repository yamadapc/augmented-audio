use augmented::audio::oscillator::Oscillator;
use augmented::vst::buffer::AudioBuffer;

pub struct Processor {
    oscillator: Oscillator<f32>,
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
        Processor { oscillator }
    }

    pub fn set_sample_rate(&mut self, rate: f32) {
        self.oscillator.set_sample_rate(rate);
    }

    pub fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let _num_channels = buffer.input_count();
        let num_samples = buffer.samples();

        let (input, mut output) = buffer.split();

        #[allow(clippy::needless_range_loop)]
        for channel in 0..1 {
            let _input_samples = input.get(channel % input.len());
            let output_samples = output.get_mut(channel % output.len());

            for sample_index in 0..num_samples {
                output_samples[sample_index] = self.oscillator.next_sample();
            }
        }
    }
}
