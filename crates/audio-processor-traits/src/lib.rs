use std::marker::PhantomData;

pub use audio_buffer::{AudioBuffer, InterleavedAudioBuffer};

pub mod audio_buffer;
pub mod midi;

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

impl AudioProcessorSettings {
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    pub fn set_input_channels(&mut self, input_channels: usize) {
        self.input_channels = input_channels;
    }

    pub fn set_output_channels(&mut self, output_channels: usize) {
        self.output_channels = output_channels;
    }

    pub fn set_block_size(&mut self, block_size: u32) {
        self.block_size = block_size;
    }
}

pub trait AudioProcessor: Send + Sync {
    type SampleType;
    fn prepare(&mut self, _settings: AudioProcessorSettings) {}
    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    );
}

pub struct NoopAudioProcessor<SampleType>(PhantomData<SampleType>);

impl<SampleType: Send + Sync> AudioProcessor for NoopAudioProcessor<SampleType> {
    type SampleType = SampleType;

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        _data: &mut BufferType,
    ) {
    }
}

pub struct SilenceAudioProcessor<SampleType>(PhantomData<SampleType>);

impl<SampleType> SilenceAudioProcessor<SampleType> {
    pub fn new() -> Self {
        SilenceAudioProcessor(PhantomData)
    }
}

impl<SampleType> Default for SilenceAudioProcessor<SampleType> {
    fn default() -> Self {
        Self::new()
    }
}

impl<SampleType: num::Float + Send + Sync> AudioProcessor for SilenceAudioProcessor<SampleType> {
    type SampleType = SampleType;

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        output: &mut BufferType,
    ) {
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
