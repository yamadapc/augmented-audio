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
use audio_processor_traits::{AudioBuffer, AudioContext, AudioProcessor, Float};
use std::marker::PhantomData;
use std::ops::AddAssign;

/// An `AudioProcessor` which will sum all input channels into input 0.
///
/// If there are no channels it'll no-op. It'll not mute the remaining channels.
pub struct StereoToMonoProcessor<SampleType> {
    phantom: PhantomData<SampleType>,
}

impl<SampleType> Default for StereoToMonoProcessor<SampleType> {
    fn default() -> Self {
        Self::new()
    }
}

impl<SampleType> StereoToMonoProcessor<SampleType> {
    pub fn new() -> Self {
        StereoToMonoProcessor {
            phantom: PhantomData::default(),
        }
    }
}

impl<SampleType> AudioProcessor for StereoToMonoProcessor<SampleType>
where
    SampleType: Float + Sync + Send + AddAssign,
{
    type SampleType = SampleType;

    fn process(&mut self, _context: &mut AudioContext, buffer: &mut AudioBuffer<SampleType>) {
        if buffer.is_empty() {
            return;
        }

        for sample_num in 0..buffer.num_samples() {
            let mut sum: SampleType = SampleType::zero();

            for channel_num in 0..buffer.num_channels() {
                sum += *buffer.get(channel_num, sample_num);
                buffer.set(channel_num, sample_num, SampleType::zero());
            }

            buffer.set(0, sample_num, sum);
        }
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::assert_f_eq;
    use audio_processor_traits::AudioBuffer;

    use super::*;

    #[test]
    fn test_stereo_to_mono_processor_sums_channels() {
        let mut mono = StereoToMonoProcessor::new();
        let samples = [1., 0.1, 1., 0.1, 1., 0.1, 1., 0.1, 1., 0.1, 1., 0.1];
        let mut input = AudioBuffer::from_interleaved(2, &samples);
        let mut context = AudioContext::default();

        mono.process(&mut context, &mut input);

        for sample_index in 0..input.num_samples() {
            let sample = *input.get(0, sample_index);
            assert_f_eq!(sample, 1.1);
        }
    }

    #[test]
    fn test_stereo_to_mono_can_handle_mono_input() {
        let mut mono = StereoToMonoProcessor::new();
        let samples = [1., 1., 1., 1., 1., 1.];
        let mut input = AudioBuffer::from_interleaved(1, &samples);
        let mut context = AudioContext::default();

        mono.process(&mut context, &mut input);

        for sample_index in 0..input.num_samples() {
            let sample = *input.get(0, sample_index);
            assert_f_eq!(sample, 1.0);
        }
    }

    #[test]
    fn test_stereo_to_mono_can_handle_empty_input() {
        let mut mono = StereoToMonoProcessor::new();
        let samples: [f32; 0] = [];
        let mut input = AudioBuffer::from_interleaved(1, &samples);
        let mut context = AudioContext::default();

        mono.process(&mut context, &mut input);
    }
}
