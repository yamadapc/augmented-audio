use num::Float;
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
/// It's recommended to convert the buffer into interleaved layout before processing as that'll be
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
    fn slice(&self) -> &[Self::SampleType];

    /// Get a mutable slice to the internal data. Will not work with VST adapter
    ///
    /// This is the faster way to process
    fn slice_mut(&mut self) -> &mut [Self::SampleType];

    /// Shortcut for `.slice().chunks(num_channels)`
    fn frames(&self) -> Chunks<'_, Self::SampleType> {
        self.slice().chunks(self.num_channels())
    }

    /// Shortcut for `.slice_mut().chunks_mut(num_channels)`
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
    unsafe fn get_unchecked_mut(&mut self, channel: usize, sample: usize) -> &mut Self::SampleType {
        self.get_mut(channel, sample)
    }

    /// Unsafe, no bounds check - Set an OUTPUT sample in this buffer
    ///
    /// Calling this on a loop will be ~10x slower than reading from `slice`.
    ///
    /// # Safety
    /// This performs no bounds checks. Make sure indexes are in range.
    unsafe fn set_unchecked(&mut self, channel: usize, sample: usize, value: Self::SampleType) {
        self.set(channel, sample, value)
    }
}

/// An AudioBuffer that stores samples as interleaved frames, used for CPAL compatibility.
///
/// Example layout:
///
/// [
///   0, 0, // <- left_sample, right_sample,
///   ...,
/// ]
pub struct InterleavedAudioBuffer<'a, SampleType> {
    num_channels: usize,
    inner: &'a mut [SampleType],
}

impl<'a, SampleType> InterleavedAudioBuffer<'a, SampleType> {
    pub fn new(num_channels: usize, inner: &'a mut [SampleType]) -> Self {
        Self {
            num_channels,
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
        self.inner.len() / self.num_channels
    }

    #[inline]
    fn slice(&self) -> &[Self::SampleType] {
        &self.inner
    }

    #[inline]
    fn slice_mut(&mut self) -> &mut [Self::SampleType] {
        &mut self.inner
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

/// A trait for buffer types that own the data they hold & can be constructed / resized.
pub trait OwnedAudioBuffer: AudioBuffer {
    /// Create an empty buffer of this type
    fn new() -> Self;
    /// Resize the buffer to fit `num_channels` and `num_samples`
    fn resize(&mut self, num_channels: usize, num_samples: usize, sample: Self::SampleType);
}

/// An owned version of the interleaved buffer implementation. Can be converted onto an
/// `InterleavedAudioBuffer`.
pub struct VecAudioBuffer<SampleType> {
    buffer: Vec<SampleType>,
    num_channels: usize,
    num_samples: usize,
}

impl<SampleType> VecAudioBuffer<SampleType> {
    pub fn new_with(buffer: Vec<SampleType>, num_channels: usize, num_samples: usize) -> Self {
        Self {
            buffer,
            num_samples,
            num_channels,
        }
    }
}

impl<SampleType: Clone> From<Vec<SampleType>> for VecAudioBuffer<SampleType> {
    fn from(simple_vec: Vec<SampleType>) -> Self {
        let num_samples = simple_vec.len();
        VecAudioBuffer::new_with(simple_vec, 1, num_samples)
    }
}

impl<SampleType> AudioBuffer for VecAudioBuffer<SampleType> {
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
        &self.buffer
    }

    #[inline]
    fn slice_mut(&mut self) -> &mut [Self::SampleType] {
        &mut self.buffer
    }

    #[inline]
    fn get(&self, channel: usize, sample: usize) -> &Self::SampleType {
        &self.buffer[sample * self.num_channels + channel]
    }

    #[inline]
    fn get_mut(&mut self, channel: usize, sample: usize) -> &mut Self::SampleType {
        &mut self.buffer[sample * self.num_channels + channel]
    }

    #[inline]
    fn set(&mut self, channel: usize, sample: usize, value: Self::SampleType) {
        self.buffer[sample * self.num_channels + channel] = value;
    }

    #[inline]
    unsafe fn get_unchecked(&self, channel: usize, sample: usize) -> &Self::SampleType {
        self.buffer
            .get_unchecked(sample * self.num_channels + channel)
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, channel: usize, sample: usize) -> &mut Self::SampleType {
        self.buffer
            .get_unchecked_mut(sample * self.num_channels + channel)
    }

    #[inline]
    unsafe fn set_unchecked(&mut self, channel: usize, sample: usize, value: Self::SampleType) {
        let sample = self
            .buffer
            .get_unchecked_mut(sample * self.num_channels + channel);
        *sample = value;
    }
}

impl<SampleType: Clone> OwnedAudioBuffer for VecAudioBuffer<SampleType> {
    #[inline]
    fn new() -> Self {
        VecAudioBuffer {
            num_channels: 0,
            num_samples: 0,
            buffer: Vec::new(),
        }
    }

    #[inline]
    fn resize(&mut self, num_channels: usize, num_samples: usize, sample: Self::SampleType) {
        self.num_samples = num_samples;
        self.num_channels = num_channels;
        self.buffer.resize(num_channels * num_samples, sample);
    }
}

impl<SampleType> VecAudioBuffer<SampleType> {
    /// Get an `InterleavedAudioBuffer` reference type out this `VecAudioBuffer`.
    pub fn interleaved(&mut self) -> InterleavedAudioBuffer<SampleType> {
        InterleavedAudioBuffer::new(self.num_channels, &mut self.buffer)
    }
}

/// VST compatibility, enabled by the `vst_support` feature.
#[cfg(feature = "vst_support")]
pub mod vst {
    use super::*;

    /// Wraps a VST buffer with a generic AudioBuffer.
    ///
    /// ## NOTE:
    /// Due to Rust VST using different references for input & output buffers the API here is
    /// slightly dubious.
    ///
    /// `audio_buffer.get(channel, sample)` will return a sample from the INPUT buffer.
    /// Meanwhile `audio_buffer.get_mut(channel, sample)` will return a sample from the OUTPUT
    /// buffer.
    ///
    /// This means it might be that `audio_buffer.get(channel, sample)` is different to
    /// `audio_buffer.get_mut(channel, sample)`.
    pub struct VSTAudioBuffer<'a, SampleType> {
        inputs: ::vst::buffer::Inputs<'a, SampleType>,
        outputs: ::vst::buffer::Outputs<'a, SampleType>,
    }

    impl<'a, SampleType: Float> VSTAudioBuffer<'a, SampleType> {
        #[deprecated]
        pub fn new(
            inputs: ::vst::buffer::Inputs<'a, SampleType>,
            outputs: ::vst::buffer::Outputs<'a, SampleType>,
        ) -> Self {
            VSTAudioBuffer { inputs, outputs }
        }

        #[deprecated]
        pub fn with_buffer(buffer: &'a mut ::vst::buffer::AudioBuffer<'a, SampleType>) -> Self {
            let (inputs, outputs) = buffer.split();
            #[allow(deprecated)]
            Self::new(inputs, outputs)
        }
    }

    impl<'a, SampleType> AudioBuffer for VSTAudioBuffer<'a, SampleType> {
        type SampleType = SampleType;

        fn num_channels(&self) -> usize {
            self.outputs.len()
        }

        fn num_samples(&self) -> usize {
            if self.outputs.is_empty() {
                0
            } else {
                self.outputs.get(0).len()
            }
        }

        fn slice(&self) -> &[Self::SampleType] {
            &[]
        }

        fn slice_mut(&mut self) -> &mut [Self::SampleType] {
            &mut []
        }

        fn get(&self, channel: usize, sample: usize) -> &Self::SampleType {
            &self.inputs.get(channel)[sample]
        }

        fn get_mut(&mut self, channel: usize, sample: usize) -> &mut Self::SampleType {
            &mut self.outputs.get_mut(channel)[sample]
        }

        fn set(&mut self, channel: usize, sample: usize, value: Self::SampleType) {
            self.outputs.get_mut(channel)[sample] = value;
        }
    }
}
