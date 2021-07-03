use num::Zero;

pub use audio_buffer::{AudioBuffer, InterleavedAudioBuffer};

pub mod audio_buffer;

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
