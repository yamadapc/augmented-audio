use audio_processor_traits::{Float, SimpleAudioProcessor};
use std::marker::PhantomData;

/// An `AudioProcessor` which will use a "source channel" as the output for all channels.
/// Does not perform bounds checking.
pub struct MonoToStereoProcessor<SampleType> {
    source_channel: usize,
    phantom: PhantomData<SampleType>,
}

impl<SampleType> Default for MonoToStereoProcessor<SampleType> {
    /// Use channel `0` as the source
    fn default() -> Self {
        MonoToStereoProcessor::new(0)
    }
}

impl<SampleType> MonoToStereoProcessor<SampleType> {
    /// Use channel `source_channel` as the source for all output channels
    pub fn new(source_channel: usize) -> Self {
        MonoToStereoProcessor {
            source_channel,
            phantom: Default::default(),
        }
    }

    /// Set the `source_channel` to use as the source for output
    pub fn set_source_channel(&mut self, source_channel: usize) {
        self.source_channel = source_channel;
    }

    /// Get the `source_channel` to use as the source for output
    pub fn source_channel(&self) -> usize {
        self.source_channel
    }
}

impl<SampleType> SimpleAudioProcessor for MonoToStereoProcessor<SampleType>
where
    SampleType: Float + Sync + Send,
{
    type SampleType = SampleType;

    fn s_process_frame(&mut self, frame: &mut [Self::SampleType]) {
        let source_sample = frame[self.source_channel];

        for sample in frame.iter_mut() {
            *sample = source_sample;
        }
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::assert_f_eq;
    use audio_processor_traits::{AudioBuffer, AudioProcessor, InterleavedAudioBuffer};

    use super::*;

    #[test]
    fn test_mono_to_stereo_handles_mono_input() {
        let mut mono = MonoToStereoProcessor::new(1);
        let mut samples = [1., 0.1, 1., 0.1, 1., 0.1, 1., 0.1, 1., 0.1, 1., 0.1];
        let mut input = InterleavedAudioBuffer::new(2, &mut samples);

        mono.process(&mut input);

        for sample_index in 0..input.num_samples() {
            for channel_index in 0..input.num_channels() {
                let sample = *input.get(channel_index, sample_index);
                assert_f_eq!(sample, 0.1);
            }
        }
    }

    #[test]
    fn test_mono_to_stereo_can_have_the_source_changed() {
        let mut mono = MonoToStereoProcessor::new(1);
        mono.set_source_channel(0);
        let mut samples = [1., 0.1, 1., 0.1, 1., 0.1, 1., 0.1, 1., 0.1, 1., 0.1];
        let mut input = InterleavedAudioBuffer::new(2, &mut samples);

        mono.process(&mut input);

        for sample_index in 0..input.num_samples() {
            for channel_index in 0..input.num_channels() {
                let sample = *input.get(channel_index, sample_index);
                assert_f_eq!(sample, 1.);
            }
        }
    }
}
