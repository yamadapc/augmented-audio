#[derive(Clone)]
pub enum AudioHostId {
    Default,
    Id(String),
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

impl Default for AudioDeviceId {
    fn default() -> Self {
        AudioDeviceId::Default
    }
}

#[derive(Clone)]
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

#[derive(Default, Clone)]
pub struct AudioThreadOptions {
    pub host_id: AudioHostId,
    pub output_device_id: AudioDeviceId,
    pub buffer_size: BufferSize,
}

impl AudioThreadOptions {
    pub fn new(
        host_id: AudioHostId,
        output_device_id: AudioDeviceId,
        buffer_size: BufferSize,
    ) -> Self {
        AudioThreadOptions {
            host_id,
            output_device_id,
            buffer_size,
        }
    }
}
