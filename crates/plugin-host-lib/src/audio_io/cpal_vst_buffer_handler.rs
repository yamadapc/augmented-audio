use audio_processor_traits::{AudioBuffer, AudioProcessorSettings};

/// Handles conversion from CPAL buffers to VST buffers
pub struct CpalVstBufferHandler {
    audio_settings: AudioProcessorSettings,
    input_buffer: Vec<Vec<f32>>,
    output_buffer: Vec<Vec<f32>>,
    host_buffer: vst::host::HostBuffer<f32>,
}

impl CpalVstBufferHandler {
    /// Create a buffer handler
    pub fn new(audio_settings: AudioProcessorSettings) -> Self {
        let num_channels = audio_settings.input_channels();
        let buffer_size = audio_settings.block_size();

        let input_buffer = Self::allocate_buffer(num_channels, buffer_size);
        let output_buffer = Self::allocate_buffer(num_channels, buffer_size);
        let host_buffer = vst::host::HostBuffer::new(num_channels, num_channels);
        log::info!("Buffer handler: num_channels={}", num_channels);

        CpalVstBufferHandler {
            audio_settings,
            input_buffer,
            output_buffer,
            host_buffer,
        }
    }

    /// Prepare the handler given changed audio settings
    pub fn prepare(&mut self, audio_settings: &AudioProcessorSettings) {
        self.audio_settings = *audio_settings;

        let num_channels = audio_settings.input_channels();
        let buffer_size = audio_settings.block_size();

        self.input_buffer = Self::allocate_buffer(num_channels, buffer_size);
        self.output_buffer = Self::allocate_buffer(num_channels, buffer_size);
        self.host_buffer = vst::host::HostBuffer::new(num_channels, num_channels);
    }

    /// Process cpal input samples
    pub fn process<BufferType: AudioBuffer<SampleType = f32>>(&mut self, data: &BufferType) {
        for sample_index in 0..data.num_samples() {
            for channel in 0..data.num_channels() {
                self.input_buffer[channel][sample_index] = *data.get(channel, sample_index);
            }
        }
    }

    /// Get the VST audio buffer
    pub fn get_audio_buffer(&mut self) -> vst::buffer::AudioBuffer<f32> {
        self.host_buffer
            .bind(&self.input_buffer, &mut self.output_buffer)
    }

    fn allocate_buffer(channels: usize, buffer_size: u32) -> Vec<Vec<f32>> {
        let mut buffer = Vec::new();
        buffer.reserve(channels);
        for _ in 0..channels {
            let channel_buffer = vec![0.0; buffer_size as usize];
            buffer.push(channel_buffer);
        }
        buffer
    }
}
