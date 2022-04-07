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
use audio_processor_traits::simple_processor::SimpleAudioProcessor;
use audio_processor_traits::Float;

/// An `AudioProcessor` that applies panning on its input.
///
/// Does not perform any bounds checking.
pub struct PanProcessor<SampleType> {
    /// A number between -1 and 1
    /// -1 represents using the left channel only, 1 represents using the right channel only.
    panning: SampleType,
}

impl<SampleType: Float> Default for PanProcessor<SampleType> {
    fn default() -> Self {
        Self::new(SampleType::from(0.0).unwrap())
    }
}

impl<SampleType: Float> PanProcessor<SampleType> {
    /// Create a processor with panning.
    /// -1 represents using the left channel only, 1 represents using the right channel only.
    pub fn new(panning: SampleType) -> Self {
        PanProcessor { panning }
    }

    /// -1 represents using the left channel only, 1 represents using the right channel only.
    pub fn panning(&self) -> SampleType {
        self.panning
    }

    /// Set the panning.
    ///
    /// -1 represents using the left channel only, 1 represents using the right channel only.
    pub fn set_panning(&mut self, panning: SampleType) {
        self.panning = panning;
    }
}

impl<SampleType> SimpleAudioProcessor for PanProcessor<SampleType>
where
    SampleType: Float + Sync + Send,
{
    type SampleType = SampleType;

    fn s_process_frame(&mut self, frame: &mut [SampleType]) {
        let zero = SampleType::zero();
        let one = SampleType::one();
        let panning = self.panning;

        let left_input = frame[0];
        let right_input = frame[1];

        if panning > zero {
            let left_output = left_input * (one - panning);
            let right_output = right_input + left_input * panning;

            frame[0] = left_output;
            frame[1] = right_output;
        } else if panning < zero {
            let left_output = left_input + right_input * (-panning);
            let right_output = right_input * (one + panning);

            frame[0] = left_output;
            frame[1] = right_output;
        }
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::assert_f_eq;
    use audio_processor_traits::simple_processor::process_buffer;
    use audio_processor_traits::AudioBuffer;

    use audio_processor_traits::InterleavedAudioBuffer;

    use super::*;

    #[test]
    fn test_pan_noop() {
        let mut pan = PanProcessor::default();
        let mut samples = [1., 1., 1., 1., 1., 1.];
        let mut input = InterleavedAudioBuffer::new(2, &mut samples);

        process_buffer(&mut pan, &mut input);

        for frame in input.frames() {
            for sample in frame.iter() {
                assert_f_eq!(*sample, 1.);
            }
        }
    }

    #[test]
    fn test_hard_pan_to_left() {
        let mut pan = PanProcessor::new(-1.0);
        let mut samples = [1., 1., 1., 1., 1., 1.];
        let mut input = InterleavedAudioBuffer::new(2, &mut samples);

        process_buffer(&mut pan, &mut input);

        for sample_index in 0..input.num_samples() {
            let left = *input.get(0, sample_index);
            let right = *input.get(1, sample_index);
            assert_f_eq!(left, 2.0);
            assert_f_eq!(right, 0.0);
        }
    }

    #[test]
    fn test_hard_pan_to_right() {
        let mut pan = PanProcessor::new(1.0);
        let mut samples = [1., 1., 1., 1., 1., 1.];
        let mut input = InterleavedAudioBuffer::new(2, &mut samples);

        process_buffer(&mut pan, &mut input);

        for sample_index in 0..input.num_samples() {
            let left = *input.get(0, sample_index);
            let right = *input.get(1, sample_index);
            assert_f_eq!(right, 2.0);
            assert_f_eq!(left, 0.0);
        }
    }
}
