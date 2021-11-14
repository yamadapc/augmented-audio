use std::fmt::Formatter;

#[derive(PartialEq, Eq, Clone)]
pub enum AudioHostId {
    Default,
    Id(String),
}

impl std::fmt::Display for AudioHostId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioHostId::Default => write!(f, "Default audio host"),
            AudioHostId::Id(str) => write!(f, "{}", str),
        }
    }
}

impl Default for AudioHostId {
    fn default() -> Self {
        AudioHostId::Default
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum AudioDeviceId {
    Default,
    Id(String),
}

impl std::fmt::Display for AudioDeviceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioDeviceId::Default => write!(f, "Default audio device"),
            AudioDeviceId::Id(str) => write!(f, "{}", str),
        }
    }
}

impl Default for AudioDeviceId {
    fn default() -> Self {
        AudioDeviceId::Default
    }
}

#[derive(PartialEq, Clone)]
pub enum BufferSize {
    Default,
    Fixed(usize),
}

impl Default for BufferSize {
    fn default() -> Self {
        BufferSize::Fixed(512)
    }
}

impl From<BufferSize> for cpal::BufferSize {
    fn from(value: BufferSize) -> Self {
        match value {
            BufferSize::Default => cpal::BufferSize::Default,
            BufferSize::Fixed(size) => cpal::BufferSize::Fixed(size as u32),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct AudioThreadOptions {
    pub host_id: AudioHostId,
    pub output_device_id: AudioDeviceId,
    pub input_device_id: Option<AudioDeviceId>,
    pub buffer_size: BufferSize,
    pub num_channels: usize,
}

impl Default for AudioThreadOptions {
    fn default() -> Self {
        Self::new(
            Default::default(),
            Default::default(),
            None,
            Default::default(),
            2,
        )
    }
}

impl AudioThreadOptions {
    pub fn new(
        host_id: AudioHostId,
        output_device_id: AudioDeviceId,
        input_device_id: Option<AudioDeviceId>,
        buffer_size: BufferSize,
        num_channels: usize,
    ) -> Self {
        AudioThreadOptions {
            host_id,
            output_device_id,
            input_device_id,
            buffer_size,
            num_channels,
        }
    }
}
