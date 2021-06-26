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

pub trait AudioProcessor: Send + Sync {
    fn prepare(&mut self, _settings: AudioProcessorSettings) {}
    fn process(&mut self, data: &mut [f32]);
}

pub struct NoopAudioProcessor;

impl AudioProcessor for NoopAudioProcessor {
    fn process(&mut self, _data: &mut [f32]) {}
}

pub struct SilenceAudioProcessor;

impl AudioProcessor for SilenceAudioProcessor {
    fn process(&mut self, output: &mut [f32]) {
        for out in output {
            *out = 0.0;
        }
    }
}
