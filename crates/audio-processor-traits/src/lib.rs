use num::Zero;

#[derive(Clone, Copy)]
pub struct AudioProcessorSettings {
    sample_rate: f32,
    input_channels: usize,
    output_channels: usize,
    block_size: u32,
}

impl Default for AudioProcessorSettings {
    fn default() -> Self {
        Self::new(44100.0, 2, 2, 512)
    }
}

impl AudioProcessorSettings {
    pub fn new(
        sample_rate: f32,
        input_channels: usize,
        output_channels: usize,
        block_size: u32,
    ) -> Self {
        AudioProcessorSettings {
            sample_rate,
            input_channels,
            output_channels,
            block_size,
        }
    }

    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    pub fn input_channels(&self) -> usize {
        self.input_channels
    }

    pub fn output_channels(&self) -> usize {
        self.output_channels
    }

    pub fn block_size(&self) -> u32 {
        self.block_size
    }
}

pub trait AudioProcessor<BufferType: AudioBuffer>: Send + Sync {
    fn prepare(&mut self, _settings: AudioProcessorSettings) {}
    fn process(&mut self, data: &mut BufferType);
}

pub struct NoopAudioProcessor;

impl<BufferType: AudioBuffer> AudioProcessor<BufferType> for NoopAudioProcessor {
    fn process(&mut self, _data: &mut BufferType) {}
}

pub struct SilenceAudioProcessor;

impl<BufferType: AudioBuffer> AudioProcessor<BufferType> for SilenceAudioProcessor {
    fn process(&mut self, output: &mut BufferType) {
        for sample_index in 0..output.num_samples() {
            for channel_index in 0..output.num_channels() {
                output.set(
                    channel_index,
                    sample_index,
                    <BufferType as AudioBuffer>::SampleType::zero(),
                );
            }
        }
    }
}

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
