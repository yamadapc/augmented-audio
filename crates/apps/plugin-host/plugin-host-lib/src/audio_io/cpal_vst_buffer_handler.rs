// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
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
    pub fn process(&mut self, data: &AudioBuffer<f32>) {
        for (channel, input_buffer_channel) in (0..data.num_channels()).zip(&mut self.input_buffer)
        {
            // The buffers have been pre-allocated to have at least num samples, however if the
            // input `data` buffer is smaller than num samples, we need to shrink the VST buffers.
            input_buffer_channel.resize(data.num_samples(), 0.0);
            self.output_buffer[channel].resize(data.num_samples(), 0.0);

            for sample_index in 0..data.num_samples() {
                input_buffer_channel[sample_index] = *data.get(channel, sample_index);
            }
        }
    }

    /// Get the VST audio buffer
    pub fn get_audio_buffer(&mut self) -> vst::buffer::AudioBuffer<f32> {
        self.host_buffer
            .bind(&self.input_buffer, &mut self.output_buffer)
    }

    fn allocate_buffer(channels: usize, buffer_size: usize) -> Vec<Vec<f32>> {
        let mut buffer = Vec::new();
        buffer.reserve(channels);
        for _ in 0..channels {
            let channel_buffer = vec![0.0; buffer_size];
            buffer.push(channel_buffer);
        }
        buffer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_the_buffer_handler_can_be_created() {
        let _handle = CpalVstBufferHandler::new(AudioProcessorSettings::default());
    }

    #[test]
    fn test_creating_the_handler_resizes_intermediary_buffers_to_match_input_channels() {
        // It doesn't matter the output channel count
        let settings = AudioProcessorSettings::new(1000.0, 2, 4, 512);
        let handle = CpalVstBufferHandler::new(settings);
        assert_eq!(handle.audio_settings, settings);
        assert_eq!(handle.input_buffer.len(), 2);
        assert_eq!(handle.output_buffer.len(), 2);
        assert_eq!(handle.host_buffer.input_count(), 2);
        assert_eq!(handle.host_buffer.output_count(), 2);
        assert_eq!(handle.input_buffer[0].len(), settings.block_size());
        assert_eq!(handle.input_buffer[1].len(), settings.block_size());
        assert_eq!(handle.output_buffer[0].len(), settings.block_size());
        assert_eq!(handle.output_buffer[1].len(), settings.block_size());
    }

    #[test]
    fn test_prepare_will_update_the_buffers() {
        let settings = AudioProcessorSettings::new(1000.0, 2, 4, 512);
        let mut handle = CpalVstBufferHandler::new(settings);
        let settings = AudioProcessorSettings::new(1000.0, 2, 4, 1024);
        handle.prepare(&settings);
        assert_eq!(handle.audio_settings, settings);
        assert_eq!(handle.input_buffer.len(), 2);
        assert_eq!(handle.output_buffer.len(), 2);
        assert_eq!(handle.host_buffer.input_count(), 2);
        assert_eq!(handle.host_buffer.output_count(), 2);
        assert_eq!(handle.input_buffer[0].len(), settings.block_size());
        assert_eq!(handle.input_buffer[1].len(), settings.block_size());
        assert_eq!(handle.output_buffer[0].len(), settings.block_size());
        assert_eq!(handle.output_buffer[1].len(), settings.block_size());
    }

    #[test]
    fn test_process_will_push_input_samples_onto_the_vst_buffer() {
        let mut input_buffer = AudioBuffer::empty();
        input_buffer.resize_with(2, 1000, || 1.0);
        let settings = AudioProcessorSettings::new(1000.0, 2, 2, 1000);
        let mut handle = CpalVstBufferHandler::new(settings);
        handle.process(&mut input_buffer);
        let mut vst_buffer = handle.get_audio_buffer();
        assert_eq!(vst_buffer.samples(), 1000);
        assert_eq!(vst_buffer.input_count(), 2);
        assert_eq!(vst_buffer.output_count(), 2);
        let (inputs, mut outputs) = vst_buffer.split();
        for channel in inputs.into_iter() {
            for sample in channel {
                assert_eq!(*sample, 1.0);
            }
        }
        for channel in outputs.into_iter() {
            for sample in channel {
                assert_eq!(*sample, 0.0);
            }
        }
    }
}
