use audio_processor_traits::audio_buffer::OwnedAudioBuffer;

pub struct Connection<BufferType>
where
    BufferType: OwnedAudioBuffer,
{
    buffer: BufferType,
}

impl<BufferType> Connection<BufferType>
where
    BufferType: OwnedAudioBuffer,
{
    pub fn new() -> Self {
        Connection {
            buffer: BufferType::new(),
        }
    }

    pub fn buffer(&self) -> &BufferType {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut BufferType {
        &mut self.buffer
    }
}
