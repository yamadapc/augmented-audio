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

use crate::AudioBuffer;

/// An AudioBuffer that stores samples as interleaved frames, used for [`cpal`] compatibility.
///
/// Example layout:
///
/// [
///   0, 0, // <- left_sample, right_sample,
///   ...,
/// ]
pub struct InterleavedAudioBuffer<'a, SampleType> {
    num_channels: usize,
    num_samples: usize,
    inner: &'a mut [SampleType],
}

impl<'a, SampleType> InterleavedAudioBuffer<'a, SampleType> {
    pub fn new(num_channels: usize, inner: &'a mut [SampleType]) -> Self {
        let num_samples = inner.len() / num_channels;
        Self {
            num_channels,
            num_samples,
            inner,
        }
    }
}

impl<'a, SampleType> AudioBuffer for InterleavedAudioBuffer<'a, SampleType> {
    type SampleType = SampleType;

    #[inline]
    fn num_channels(&self) -> usize {
        self.num_channels
    }

    #[inline]
    fn num_samples(&self) -> usize {
        self.num_samples
    }

    #[inline]
    fn slice(&self) -> &[Self::SampleType] {
        self.inner
    }

    #[inline]
    fn slice_mut(&mut self) -> &mut [Self::SampleType] {
        self.inner
    }

    #[inline]
    fn get(&self, channel: usize, sample: usize) -> &SampleType {
        &self.inner[sample * self.num_channels + channel]
    }

    #[inline]
    fn get_mut(&mut self, channel: usize, sample: usize) -> &mut SampleType {
        &mut self.inner[sample * self.num_channels + channel]
    }

    #[inline]
    fn set(&mut self, channel: usize, sample: usize, value: SampleType) {
        let sample_ref = self.get_mut(channel, sample);
        *sample_ref = value;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_interleaved_buffer() {
        let mut slice = [0.0, 0.0, 2.0, 2.0];
        let buffer = InterleavedAudioBuffer::new(2, &mut slice);
        drop(buffer);
    }

    #[test]
    fn test_num_samples() {
        let mut slice = [0.0, 0.0, 2.0, 2.0, 3.0, 3.0];
        let buffer = InterleavedAudioBuffer::new(2, &mut slice);
        assert_eq!(buffer.num_samples(), 3);
    }

    #[test]
    fn test_num_channels() {
        let mut slice = [0.0, 0.0, 2.0, 2.0];
        let buffer = InterleavedAudioBuffer::new(2, &mut slice);
        assert_eq!(buffer.num_channels(), 2);
    }

    #[test]
    fn test_frames() {
        let mut slice = [0.0, 0.0, 2.0, 2.0];
        let buffer = InterleavedAudioBuffer::new(2, &mut slice);
        assert_eq!(
            buffer.frames().map(|f| f.to_vec()).collect::<Vec<_>>(),
            vec![vec![0.0, 0.0], vec![2.0, 2.0]],
        );
    }

    #[test]
    fn test_frames_mut() {
        let mut slice = [0.0, 0.0, 2.0, 2.0];
        let mut buffer = InterleavedAudioBuffer::new(2, &mut slice);

        for frame in buffer.frames_mut() {
            frame[1] = 1.0;
        }
        assert_eq!(
            buffer.frames().map(|f| f.to_vec()).collect::<Vec<_>>(),
            vec![vec![0.0, 1.0], vec![2.0, 1.0]],
        );
    }

    #[test]
    fn test_get() {
        let mut slice = [0.0, 0.0, 2.0, 2.0, 3.0, 4.0];
        let buffer = InterleavedAudioBuffer::new(2, &mut slice);
        assert_eq!(*buffer.get(0, 1), 2.0);
        assert_eq!(*buffer.get(1, 2), 4.0);
    }

    #[test]
    fn test_get_mut() {
        let mut slice = [0.0, 0.0, 2.0, 2.0, 3.0, 4.0];
        let mut buffer = InterleavedAudioBuffer::new(2, &mut slice);
        let s = buffer.get_mut(0, 1);
        *s = 10.0;
        assert_eq!(*buffer.get(0, 1), 10.0);
    }

    #[test]
    fn test_set() {
        let mut slice = [0.0, 0.0, 2.0, 2.0, 3.0, 4.0];
        let mut buffer = InterleavedAudioBuffer::new(2, &mut slice);
        buffer.set(0, 1, 10.0);
        assert_eq!(*buffer.get(0, 1), 10.0);
    }
}
