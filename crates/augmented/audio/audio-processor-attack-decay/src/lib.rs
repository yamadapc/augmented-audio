use audio_processor_analysis::envelope_follower_processor::EnvelopeFollowerProcessor;
use audio_processor_analysis::fft_processor::{FftProcessor, FftProcessorOptions};
use audio_processor_traits::simple_processor::MonoAudioProcessor;
use audio_processor_traits::{AudioBuffer, AudioContext, AudioProcessor};
use rustfft::num_complex::Complex;
use rustfft::FftDirection;
use std::sync::Arc;
use std::time::Duration;

#[derive(Default)]
pub struct AttackDecayProcessor {
    fft: Vec<FftProcessor>,
    envelope_followers: Vec<Vec<EnvelopeFollowerProcessor>>,
    inverse_fft: Vec<Arc<dyn rustfft::Fft<f32>>>,
    output_buffer: Vec<Vec<f32>>,
    output_cursor: usize,
}

impl AudioProcessor for AttackDecayProcessor {
    type SampleType = f32;

    fn prepare(&mut self, context: &mut AudioContext) {
        self.fft.clear();
        self.output_buffer.clear();
        self.inverse_fft.clear();
        self.envelope_followers.clear();

        let mut planner = rustfft::FftPlanner::new();
        for _ in 0..context.settings.output_channels {
            let mut fft = FftProcessor::new(FftProcessorOptions {
                size: 8192,
                direction: FftDirection::Forward,
                overlap_ratio: 0.75,
                ..FftProcessorOptions::default()
            });
            fft.m_prepare(context);
            let mut envelope_followers = vec![];
            envelope_followers.reserve(fft.size());
            for _bin in 0..fft.size() {
                let envelope = EnvelopeFollowerProcessor::default();
                envelope.handle().set_attack(Duration::from_millis(400));
                envelope_followers.push(envelope);
            }

            let output_buffer = vec![0.0; fft.size()];
            self.output_buffer.push(output_buffer);

            let inverse_fft = planner.plan_fft_inverse(fft.size());
            self.inverse_fft.push(inverse_fft);
            self.fft.push(fft);
            self.envelope_followers.push(envelope_followers);
        }
    }

    fn process(&mut self, context: &mut AudioContext, data: &mut AudioBuffer<Self::SampleType>) {
        for (channel_num, channel) in data.channels_mut().iter_mut().enumerate() {
            let mut output_cursor = self.output_cursor;
            for sample in channel {
                self.fft[channel_num].m_process(context, *sample);
                if self.fft[channel_num].has_changed() {
                    self.on_fft(context, channel_num);
                }

                let output = self.output_buffer[channel_num][output_cursor];
                self.output_buffer[channel_num][output_cursor] = 0.0;
                *sample = output;
                output_cursor += 1;
                output_cursor %= self.fft[channel_num].size();
            }
        }
        self.output_cursor += data.num_samples();
        self.output_cursor %= self.fft[0].size();
    }
}

impl AttackDecayProcessor {
    fn on_fft(&mut self, context: &mut AudioContext, fft_index: usize) {
        let fft_size_i = self.fft[fft_index].size();
        let fft_size = fft_size_i as f32;
        let input_fft = self.fft[fft_index].buffer_mut();

        for (bin_index, value) in input_fft.iter_mut().enumerate() {
            let envelope = &mut self.envelope_followers[fft_index][bin_index];
            envelope.m_process(context, value.norm());
            *value = Complex::from_polar(
                envelope.handle().state().min(1.0) * value.norm(),
                value.arg(),
            );
        }
        for i in fft_size_i / 2..fft_size_i {
            input_fft[i] = Complex::from(0.0);
        }
        self.inverse_fft[fft_index].process(input_fft);

        // now need to copy this into an output section buffer
        let output_buffer = &mut self.output_buffer[fft_index];
        for (index, value) in input_fft.iter().enumerate() {
            output_buffer[(self.output_cursor + index) % fft_size_i] += (value.re / fft_size) * 0.5;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _attack_decay = AttackDecayProcessor::default();
    }
}
