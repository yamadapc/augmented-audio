use num::Float;

/// Represents an audio buffer. This decouples audio processing code from a certain representation
/// of multi-channel sample buffers.
///
/// This crate provides implementations of this trait for VST & CPal style buffers, which have
/// different internal representations.
pub trait AudioBuffer {
    /// The type of samples within this buffer.
    type SampleType;

    /// The number of channels in this buffer
    fn num_channels(&self) -> usize;

    /// The number of samples in this buffer
    fn num_samples(&self) -> usize;

    /// Get a ref to an INPUT sample in this buffer
    fn get(&self, channel: usize, sample: usize) -> &Self::SampleType;

    /// Get a mutable ref to an OUTPUT sample in this buffer
    ///
    /// On some implementations this may yield a different value than `.get`.
    fn get_mut(&mut self, channel: usize, sample: usize) -> &mut Self::SampleType;

    /// Set an OUTPUT sample in this buffer
    fn set(&mut self, channel: usize, sample: usize, value: Self::SampleType);

    /// Unsafe, no bounds check - Get a ref to an INPUT sample in this buffer
    unsafe fn get_unchecked(&self, channel: usize, sample: usize) -> &Self::SampleType {
        self.get(channel, sample)
    }

    /// Unsafe, no bounds check - Get a mutable ref to an OUTPUT sample in this buffer
    ///
    /// On some implementations this may yield a different value than `.get`.
    unsafe fn get_unchecked_mut(&mut self, channel: usize, sample: usize) -> &mut Self::SampleType {
        self.get_mut(channel, sample)
    }

    /// Unsafe, no bounds check - Set an OUTPUT sample in this buffer
    unsafe fn set_unchecked(&mut self, channel: usize, sample: usize, value: Self::SampleType) {
        self.set(channel, sample, value)
    }

    /// Create a read only iterator
    fn iter(&self) -> AudioBufferIterator<Self> {
        AudioBufferIterator::new(&self)
    }
}

/// Iterator for audio buffers
pub struct AudioBufferIterator<'a, BufferType: AudioBuffer + ?Sized> {
    position: usize,
    buffer: &'a BufferType,
}

impl<'a, BufferType: AudioBuffer + ?Sized> AudioBufferIterator<'a, BufferType> {
    pub fn new(buffer: &'a BufferType) -> Self {
        AudioBufferIterator {
            position: 0,
            buffer,
        }
    }
}

impl<'a, BufferType: AudioBuffer> Iterator for AudioBufferIterator<'a, BufferType> {
    type Item = AudioFrameReference<'a, BufferType>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.buffer.num_samples() {
            return None;
        }

        let reference = AudioFrameReference::new(self.buffer, self.position);
        self.position += 1;
        Some(reference)
    }
}

pub struct AudioFrameReference<'a, BufferType> {
    sample_index: usize,
    buffer: &'a BufferType,
}

impl<'a, BufferType> AudioFrameReference<'a, BufferType> {
    fn new(buffer: &'a BufferType, sample_index: usize) -> Self {
        AudioFrameReference {
            sample_index,
            buffer,
        }
    }

    pub fn iter(&self) -> AudioFrameReferenceIterator<'a, BufferType> {
        AudioFrameReferenceIterator::new(self.buffer, self.sample_index)
    }
}

pub struct AudioFrameReferenceIterator<'a, BufferType> {
    buffer: &'a BufferType,
    sample_index: usize,
    channel_index: usize,
}

impl<'a, BufferType> AudioFrameReferenceIterator<'a, BufferType> {
    fn new(buffer: &'a BufferType, sample_index: usize) -> Self {
        AudioFrameReferenceIterator {
            buffer,
            sample_index,
            channel_index: 0,
        }
    }
}

impl<'a, BufferType: AudioBuffer> Iterator for AudioFrameReferenceIterator<'a, BufferType> {
    type Item = &'a BufferType::SampleType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.channel_index >= self.buffer.num_channels() {
            None
        } else {
            let r = self.buffer.get(self.channel_index, self.sample_index);
            self.channel_index += 1;
            Some(r)
        }
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

    pub fn inner(&self) -> &[SampleType] {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut [SampleType] {
        &mut self.inner
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

/// An AudioBuffer that stores samples as separate buffer slices. Similar the VST, but unused due to
/// an explicit wrapper on top of rust-vst also being exported.
///
/// Example:
/// `[left_channel_ptr, right_channel_ptr]`
///
/// `left_channel = [0, 1, 2, 3, 4]`
pub struct SliceAudioBuffer<'a, SampleType> {
    channels: &'a mut [&'a mut [SampleType]],
}

impl<'a, SampleType> SliceAudioBuffer<'a, SampleType> {
    #[inline]
    pub fn new(channels: &'a mut [&'a mut [SampleType]]) -> Self {
        Self { channels }
    }
}

impl<'a, SampleType> AudioBuffer for SliceAudioBuffer<'a, SampleType> {
    type SampleType = SampleType;

    #[inline]
    fn num_channels(&self) -> usize {
        self.channels.len()
    }

    #[inline]
    fn num_samples(&self) -> usize {
        if self.channels.is_empty() {
            0
        } else {
            self.channels[0].len()
        }
    }

    #[inline]
    fn get(&self, channel: usize, sample: usize) -> &Self::SampleType {
        &self.channels[channel][sample]
    }

    #[inline]
    fn get_mut(&mut self, channel: usize, sample: usize) -> &mut Self::SampleType {
        &mut self.channels[channel][sample]
    }

    #[inline]
    fn set(&mut self, channel: usize, sample: usize, value: Self::SampleType) {
        self.channels[channel][sample] = value;
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
        pub fn new(
            inputs: ::vst::buffer::Inputs<'a, SampleType>,
            outputs: ::vst::buffer::Outputs<'a, SampleType>,
        ) -> Self {
            VSTAudioBuffer { inputs, outputs }
        }

        pub fn with_buffer(buffer: &'a mut ::vst::buffer::AudioBuffer<'a, SampleType>) -> Self {
            let (inputs, outputs) = buffer.split();
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
