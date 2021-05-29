use crate::commands::main::audio_settings::AudioSettings;
use vst::buffer::AudioBuffer;
use vst::host::HostBuffer;

/// Handles conversion from CPAL buffers to VST buffers
pub struct CpalVstBufferHandler {
    audio_settings: AudioSettings,
    input_buffer: Vec<Vec<f32>>,
    output_buffer: Vec<Vec<f32>>,
    host_buffer: HostBuffer<f32>,
}

impl CpalVstBufferHandler {
    /// Create a buffer handler
    pub fn new(audio_settings: AudioSettings) -> Self {
        let num_channels = audio_settings.channels();
        let buffer_size = audio_settings.buffer_size();

        let input_buffer = Self::allocate_buffer(num_channels, buffer_size);
        let mut output_buffer = Self::allocate_buffer(num_channels, buffer_size);
        let mut host_buffer = HostBuffer::new(num_channels, num_channels);

        CpalVstBufferHandler {
            audio_settings,
            input_buffer,
            output_buffer,
            host_buffer,
        }
    }

    /// Prepare the handler given changed audio settings
    pub fn prepare(&mut self, audio_settings: &AudioSettings) {
        self.audio_settings = audio_settings.clone();

        let num_channels = audio_settings.channels();
        let buffer_size = audio_settings.buffer_size();

        self.input_buffer = Self::allocate_buffer(num_channels, buffer_size);
        self.output_buffer = Self::allocate_buffer(num_channels, buffer_size);
        self.host_buffer = HostBuffer::new(num_channels, num_channels);
    }

    /// Modify the input buffer
    pub fn set_input(&mut self, channel: usize, sample: usize, value: f32) {
        self.input_buffer[channel][sample] = value;
    }

    /// Modify the output buffer
    pub fn set_output(&mut self, channel: usize, sample: usize, value: f32) {
        self.output_buffer[channel][sample] = value;
    }

    /// Process cpal input samples
    pub fn process(&mut self, data: &[f32]) {
        for (sample_index, frame) in data.chunks(self.audio_settings.channels()).enumerate() {
            for (channel, sample) in frame.iter().enumerate() {
                self.set_input(channel, sample_index, *sample);
                self.set_output(channel, sample_index, *sample);
            }
        }
    }

    /// Get the VST audio buffer
    pub fn get_audio_buffer(&mut self) -> AudioBuffer<f32> {
        let mut audio_buffer = self
            .host_buffer
            .bind(&self.input_buffer, &mut self.output_buffer);
        audio_buffer
    }

    fn allocate_buffer(channels: usize, buffer_size: u32) -> Vec<Vec<f32>> {
        let mut buffer = Vec::new();
        buffer.reserve(channels);
        for _ in 0..channels {
            let mut channel_buffer = Vec::new();
            channel_buffer.reserve(buffer_size as usize);
            for _ in 0..buffer_size {
                channel_buffer.push(0.0);
            }
            buffer.push(channel_buffer);
        }
        buffer
    }
}
