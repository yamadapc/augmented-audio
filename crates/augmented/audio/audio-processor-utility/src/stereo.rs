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
use audio_processor_traits::{AudioContext, Float, SimpleAudioProcessor};
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

    fn s_process_frame(&mut self, _context: &mut AudioContext, frame: &mut [Self::SampleType]) {
        let source_sample = frame[self.source_channel];

        for sample in frame.iter_mut() {
            *sample = source_sample;
        }
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::assert_f_eq;
    use audio_processor_traits::{simple_processor, AudioBuffer, InterleavedAudioBuffer};

    use super::*;

    #[test]
    fn test_mono_to_stereo_handles_mono_input() {
        let mut mono = MonoToStereoProcessor::new(1);
        let mut samples = [1., 0.1, 1., 0.1, 1., 0.1, 1., 0.1, 1., 0.1, 1., 0.1];
        let mut input = InterleavedAudioBuffer::new(2, &mut samples);
        let mut context = AudioContext::default();

        simple_processor::process_buffer(&mut context, &mut mono, &mut input);

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
        let mut context = AudioContext::default();

        simple_processor::process_buffer(&mut context, &mut mono, &mut input);

        for sample_index in 0..input.num_samples() {
            for channel_index in 0..input.num_channels() {
                let sample = *input.get(channel_index, sample_index);
                assert_f_eq!(sample, 1.);
            }
        }
    }
}
