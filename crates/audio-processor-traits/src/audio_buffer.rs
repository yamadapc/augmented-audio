/// Passing around `&mut [f32]` as audio buffers isn't good because:
///
/// * Some libraries / APIs will use interleaved buffers
/// * Some will not
/// * If you pick one all your processor code is bound to a buffer layout
/// * If there's an abstraction on top the processor code can work for any buffer layout while
///   still having the sample performance
/// * Currently `AudioProcessor` is made to work with cpal interleaved buffers; it then needs
///   conversion to work with VST.
/// * That's very unfortunate. I'd like to write a single processor that can work with both buffer
///   types with no overhead.
pub trait AudioBuffer {
    type SampleType: num::Float + Sync + Send;

    fn num_channels(&self) -> usize;
    fn num_samples(&self) -> usize;
    fn get(&self, channel: usize, sample: usize) -> &Self::SampleType;
    fn get_mut(&mut self, channel: usize, sample: usize) -> &mut Self::SampleType;
    fn set(&mut self, channel: usize, sample: usize, value: Self::SampleType);
}

/// An AudioBuffer that stores samples as interleaved frames, used for CPAL.
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

impl<'a, SampleType: num::Float + Sync + Send> AudioBuffer
    for InterleavedAudioBuffer<'a, SampleType>
{
    type SampleType = SampleType;

    fn num_channels(&self) -> usize {
        self.num_channels
    }

    fn num_samples(&self) -> usize {
        self.inner.len() / self.num_channels
    }

    fn get(&self, channel: usize, sample: usize) -> &SampleType {
        &self.inner[sample * self.num_channels + channel]
    }

    fn get_mut(&mut self, channel: usize, sample: usize) -> &mut SampleType {
        &mut self.inner[sample * self.num_channels + channel]
    }

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
    pub fn new(channels: &'a mut [&'a mut [SampleType]]) -> Self {
        Self { channels }
    }
}

impl<'a, SampleType: num::Float + Sync + Send> AudioBuffer for SliceAudioBuffer<'a, SampleType> {
    type SampleType = SampleType;

    fn num_channels(&self) -> usize {
        self.channels.len()
    }

    fn num_samples(&self) -> usize {
        if self.channels.is_empty() {
            0
        } else {
            self.channels[0].len()
        }
    }

    fn get(&self, channel: usize, sample: usize) -> &Self::SampleType {
        &self.channels[channel][sample]
    }

    fn get_mut(&mut self, channel: usize, sample: usize) -> &mut Self::SampleType {
        &mut self.channels[channel][sample]
    }

    fn set(&mut self, channel: usize, sample: usize, value: Self::SampleType) {
        self.channels[channel][sample] = value;
    }
}

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
    pub struct VSTAudioBuffer<'a, SampleType: num::Float> {
        inputs: ::vst::buffer::Inputs<'a, SampleType>,
        outputs: ::vst::buffer::Outputs<'a, SampleType>,
    }

    impl<'a, SampleType: num::Float> VSTAudioBuffer<'a, SampleType> {
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

    impl<'a, SampleType: num::Float + Sync + Send> AudioBuffer for VSTAudioBuffer<'a, SampleType> {
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
