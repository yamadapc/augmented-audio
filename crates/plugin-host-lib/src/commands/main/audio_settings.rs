#[derive(Clone, Copy)]
pub struct AudioSettings {
    sample_rate: f32,
    channels: usize,
    buffer_size: u32,
}

impl AudioSettings {
    pub fn new(sample_rate: f32, channels: usize, buffer_size: u32) -> Self {
        AudioSettings {
            sample_rate,
            channels,
            buffer_size,
        }
    }
}

impl AudioSettings {
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    pub fn channels(&self) -> usize {
        self.channels
    }

    pub fn buffer_size(&self) -> u32 {
        self.buffer_size
    }
}
