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

pub use audio_buffer_trait::*;
pub use interleaved_buffers::*;
pub use owned_audio_buffer_trait::*;
pub use util::*;

mod audio_buffer_trait;
mod interleaved_buffers;
mod owned_audio_buffer_trait;
mod util;

#[cfg(feature = "vst")]
pub mod vst;

/// Copy from an interleaved buffer into a target non-interleaved buffer.
pub fn copy_from_interleaved<SampleType: Copy>(
    source: &[SampleType],
    target: &mut [impl AsMut<[SampleType]>],
) {
    if target.is_empty() {
        return;
    }

    let num_channels = target.len();
    for (sample_idx, frame) in source.chunks(num_channels).enumerate() {
        for (channel_idx, sample) in frame.iter().enumerate() {
            target[channel_idx].as_mut()[sample_idx] = *sample;
        }
    }
}

pub fn to_interleaved<SampleType: Copy + num::Zero>(
    source: &[SampleType],
    num_channels: usize,
) -> Vec<Vec<SampleType>> {
    let mut result = vec![vec![SampleType::zero(); source.len() / num_channels]; num_channels];
    copy_from_interleaved(source, &mut result);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_copy_from_interleaved_empty() {
        let mut target: Vec<Vec<f32>> = vec![vec![0.0; 0]; 0];
        let source = vec![];
        copy_from_interleaved(&source, &mut target);
        assert_eq!(target, Vec::<Vec<f32>>::new());
    }

    #[test]
    fn test_copy_from_interleaved() {
        let mut target: Vec<Vec<f32>> = vec![vec![0.0; 2]; 2];
        let source = vec![1.0, 2.0, 3.0, 4.0];
        copy_from_interleaved(&source, &mut target);
        assert_eq!(target, vec![vec![1.0, 3.0], vec![2.0, 4.0]]);
    }

    #[test]
    fn test_to_interleaved() {
        let source = vec![1.0, 2.0, 3.0, 4.0];
        let result = to_interleaved(&source, 2);
        assert_eq!(result, vec![vec![1.0, 3.0], vec![2.0, 4.0]]);
    }
}
