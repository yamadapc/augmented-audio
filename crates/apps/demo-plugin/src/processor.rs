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
