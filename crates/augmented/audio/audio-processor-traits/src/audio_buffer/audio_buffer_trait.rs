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

use std::slice::{Chunks, ChunksMut};

/// Represents an audio buffer. This decouples audio processing code from a certain representation
/// of multi-channel sample buffers.
///
/// This crate provides implementations of this trait for CPal style buffers, which use interleaved
/// internal representation.
///
/// When processing samples, it'll be more efficient to use `.slice` and `.slice_mut` than `.get` /
/// `.set` methods. For the VST buffer, these methods will not work.
///
/// It's recommended to convert the buffer into interleaved layout before processing as that will be
/// around as expensive as the overhead of `get`/`set` methods on a single loop through samples.
///
/// (due to bounds checking and other compiler optimisations that fail with them)
pub trait AudioBuffer {
    /// The type of samples within this buffer.
    type SampleType;

    /// The number of channels in this buffer
    fn num_channels(&self) -> usize;

    /// The number of samples in this buffer
    fn num_samples(&self) -> usize;

    /// Get a slice to the internal data. Will not work with VST adapter
    ///
    /// This is the faster way to process
    #[deprecated]
    fn slice(&self) -> &[Self::SampleType];

    /// Get a mutable slice to the internal data. Will not work with VST adapter
    ///
    /// This is the faster way to process
    #[deprecated]
    fn slice_mut(&mut self) -> &mut [Self::SampleType];

    /// Shortcut for `.slice().chunks(num_channels)`. Default implementation assumes the buffer is
    /// interleaved.
    fn frames(&self) -> Chunks<'_, Self::SampleType> {
        self.slice().chunks(self.num_channels())
    }

    /// Shortcut for `.slice_mut().chunks_mut(num_channels)`
    ///
    /// This is a frame representing a sample in time, for all
    /// channels.
    fn frames_mut(&mut self) -> ChunksMut<'_, Self::SampleType> {
        let channels = self.num_channels();
        self.slice_mut().chunks_mut(channels)
    }

    /// Get a ref to an INPUT sample in this buffer.
    ///
    /// Calling this on a loop will be ~20x slower than reading from `slice`.
    fn get(&self, channel: usize, sample: usize) -> &Self::SampleType;

    /// Get a mutable ref to an OUTPUT sample in this buffer
    ///
    /// On some implementations this may yield a different value than `.get`.
    ///
    /// Calling this on a loop will be ~20x slower than reading from `slice`.
    fn get_mut(&mut self, channel: usize, sample: usize) -> &mut Self::SampleType;

    /// Set an OUTPUT sample in this buffer
    fn set(&mut self, channel: usize, sample: usize, value: Self::SampleType);

    /// Unsafe, no bounds check - Get a ref to an INPUT sample in this buffer
    ///
    /// Calling this on a loop will be ~10x slower than reading from `slice`.
    ///
    /// # Safety
    /// This performs no bounds checks. Make sure indexes are in range.
    #[deprecated]
    unsafe fn get_unchecked(&self, channel: usize, sample: usize) -> &Self::SampleType {
        self.get(channel, sample)
    }

    /// Unsafe, no bounds check - Get a mutable ref to an OUTPUT sample in this buffer
    ///
    /// On some implementations this may yield a different value than `.get`.
    ///
    /// Calling this on a loop will be ~10x slower than reading from `slice`.
    ///
    /// # Safety
    /// This performs no bounds checks. Make sure indexes are in range.
    #[deprecated]
    unsafe fn get_unchecked_mut(&mut self, channel: usize, sample: usize) -> &mut Self::SampleType {
        self.get_mut(channel, sample)
    }

    /// Unsafe, no bounds check - Set an OUTPUT sample in this buffer
    ///
    /// Calling this on a loop will be ~10x slower than reading from `slice`.
    ///
    /// # Safety
    /// This performs no bounds checks. Make sure indexes are in range.
    #[deprecated]
    unsafe fn set_unchecked(&mut self, channel: usize, sample: usize, value: Self::SampleType) {
        self.set(channel, sample, value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct MockAudioBuffer {
        data: Vec<f32>,
        num_channels: usize,
    }

    impl AudioBuffer for MockAudioBuffer {
        type SampleType = f32;

        fn num_channels(&self) -> usize {
            self.num_channels
        }

        fn num_samples(&self) -> usize {
            self.data.len() / self.num_channels
        }

        fn slice(&self) -> &[Self::SampleType] {
            &self.data
        }

        fn slice_mut(&mut self) -> &mut [Self::SampleType] {
            &mut self.data
        }

        fn get(&self, channel: usize, sample: usize) -> &Self::SampleType {
            &self.data[channel + sample * self.num_channels]
        }

        fn get_mut(&mut self, channel: usize, sample: usize) -> &mut Self::SampleType {
            &mut self.data[channel + sample * self.num_channels]
        }

        fn set(&mut self, channel: usize, sample: usize, value: Self::SampleType) {
            self.data[channel + sample * self.num_channels] = value;
        }
    }

    #[test]
    fn test_num_channels() {
        let mock_audio_buffer = MockAudioBuffer {
            data: vec![0.0; 4 * 2],
            num_channels: 2,
        };
        assert_eq!(mock_audio_buffer.num_channels(), 2);
    }

    #[test]
    fn test_num_samples() {
        let mock_audio_buffer = MockAudioBuffer {
            data: vec![0.0; 4 * 2],
            num_channels: 2,
        };
        assert_eq!(mock_audio_buffer.num_samples(), 4);
    }

    #[test]
    fn test_slice() {
        let mock_audio_buffer = MockAudioBuffer {
            data: vec![
                0.0, 4.0, // 1
                1.0, 5.0, // 2
                2.0, 6.0, // 3
                3.0, 7.0, // 4
            ],
            num_channels: 2,
        };
        assert_eq!(mock_audio_buffer.num_samples(), 4);
        assert_eq!(
            mock_audio_buffer.slice(),
            [
                0.0, 4.0, // 1
                1.0, 5.0, // 2
                2.0, 6.0, // 3
                3.0, 7.0, // 4
            ]
        );
    }

    #[test]
    fn test_slice_mut() {
        let mut mock_audio_buffer = MockAudioBuffer {
            data: vec![
                0.0, 4.0, // 1
                1.0, 5.0, // 2
                2.0, 6.0, // 3
                3.0, 7.0, // 4
            ],
            num_channels: 2,
        };
        assert_eq!(
            mock_audio_buffer.slice_mut(),
            [
                0.0, 4.0, // 1
                1.0, 5.0, // 2
                2.0, 6.0, // 3
                3.0, 7.0, // 4
            ]
        );
        for sample in mock_audio_buffer.slice_mut() {
            *sample *= 2.0;
        }
        assert_eq!(
            mock_audio_buffer.data,
            [
                0.0, 8.0, // 1
                2.0, 10.0, // 2
                4.0, 12.0, // 3
                6.0, 14.0, // 4
            ]
        );
    }

    #[test]
    fn test_frames() {
        let mock_audio_buffer = MockAudioBuffer {
            data: vec![
                0.0, 4.0, // 1
                1.0, 5.0, // 2
                2.0, 6.0, // 3
                3.0, 7.0, // 4
            ],
            num_channels: 2,
        };
        assert_eq!(mock_audio_buffer.num_samples(), 4);
        assert_eq!(
            mock_audio_buffer
                .frames()
                .map(|f| f.to_vec())
                .collect::<Vec<_>>(),
            [
                vec![0.0, 4.0],
                vec![1.0, 5.0],
                vec![2.0, 6.0],
                vec![3.0, 7.0],
            ]
        );
    }

    #[test]
    fn test_frames_mut() {
        let mock_audio_buffer = MockAudioBuffer {
            data: vec![
                0.0, 4.0, // 1
                1.0, 5.0, // 2
                2.0, 6.0, // 3
                3.0, 7.0, // 4
            ],
            num_channels: 2,
        };
        assert_eq!(mock_audio_buffer.num_samples(), 4);
        assert_eq!(
            mock_audio_buffer
                .frames()
                .map(|f| f.to_vec())
                .collect::<Vec<_>>(),
            [
                vec![0.0, 4.0],
                vec![1.0, 5.0],
                vec![2.0, 6.0],
                vec![3.0, 7.0],
            ]
        );
    }

    #[test]
    fn test_get() {
        let mock_audio_buffer = MockAudioBuffer {
            data: vec![
                0.0, 4.0, // 1
                1.0, 5.0, // 2
                2.0, 6.0, // 3
                3.0, 7.0, // 4
            ],
            num_channels: 2,
        };
        assert_eq!(*mock_audio_buffer.get(0, 1), 1.0);
    }

    #[test]
    fn test_get_mut() {
        let mut mock_audio_buffer = MockAudioBuffer {
            data: vec![
                0.0, 4.0, // 1
                1.0, 5.0, // 2
                2.0, 6.0, // 3
                3.0, 7.0, // 4
            ],
            num_channels: 2,
        };
        let sample = mock_audio_buffer.get_mut(0, 1);
        *sample = 2.0;
        assert_eq!(
            mock_audio_buffer.data,
            [
                0.0, 4.0, // 1
                2.0, 5.0, // 2
                2.0, 6.0, // 3
                3.0, 7.0, // 4
            ]
        );
    }

    #[test]
    fn test_get_unchecked() {
        let mock_audio_buffer = MockAudioBuffer {
            data: vec![
                0.0, 4.0, // 1
                1.0, 5.0, // 2
                2.0, 6.0, // 3
                3.0, 7.0, // 4
            ],
            num_channels: 2,
        };
        unsafe {
            assert_eq!(*mock_audio_buffer.get_unchecked(0, 1), 1.0);
        }
    }

    #[test]
    fn test_get_mut_unchecked() {
        let mut mock_audio_buffer = MockAudioBuffer {
            data: vec![
                0.0, 4.0, // 1
                1.0, 5.0, // 2
                2.0, 6.0, // 3
                3.0, 7.0, // 4
            ],
            num_channels: 2,
        };
        let sample = unsafe { mock_audio_buffer.get_unchecked_mut(0, 1) };
        *sample = 2.0;
        assert_eq!(
            mock_audio_buffer.data,
            [
                0.0, 4.0, // 1
                2.0, 5.0, // 2
                2.0, 6.0, // 3
                3.0, 7.0, // 4
            ]
        );
    }

    #[test]
    fn test_set() {
        let mut mock_audio_buffer = MockAudioBuffer {
            data: vec![
                0.0, 4.0, // 1
                1.0, 5.0, // 2
                2.0, 6.0, // 3
                3.0, 7.0, // 4
            ],
            num_channels: 2,
        };
        mock_audio_buffer.set(0, 1, 2.0);
        assert_eq!(
            mock_audio_buffer.data,
            [
                0.0, 4.0, // 1
                2.0, 5.0, // 2
                2.0, 6.0, // 3
                3.0, 7.0, // 4
            ]
        );
    }

    #[test]
    fn test_set_unchecked() {
        let mut mock_audio_buffer = MockAudioBuffer {
            data: vec![
                0.0, 4.0, // 1
                1.0, 5.0, // 2
                2.0, 6.0, // 3
                3.0, 7.0, // 4
            ],
            num_channels: 2,
        };
        unsafe {
            mock_audio_buffer.set_unchecked(0, 1, 2.0);
        }
        assert_eq!(
            mock_audio_buffer.data,
            [
                0.0, 4.0, // 1
                2.0, 5.0, // 2
                2.0, 6.0, // 3
                3.0, 7.0, // 4
            ]
        );
    }
}
