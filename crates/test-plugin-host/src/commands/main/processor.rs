use cpal::{OutputCallbackInfo, StreamError};
use vst::host::PluginInstance;
use vst::plugin::Plugin;

use crate::commands::main::audio_settings::AudioSettings;
use crate::commands::main::cpal_vst_buffer_handler::CpalVstBufferHandler;
use vst::buffer::AudioBuffer;

/// The app's main processor
pub struct TestHostProcessor {
    plugin_instance: PluginInstance,
    audio_settings: AudioSettings,
    buffer_handler: CpalVstBufferHandler,
}

unsafe impl Send for TestHostProcessor {}
unsafe impl Sync for TestHostProcessor {}

impl TestHostProcessor {
    pub fn new(
        plugin_instance: PluginInstance,
        sample_rate: f32,
        channels: usize,
        buffer_size: u32,
    ) -> Self {
        let audio_settings = AudioSettings::new(sample_rate, channels, buffer_size);
        TestHostProcessor {
            plugin_instance,
            audio_settings,
            buffer_handler: CpalVstBufferHandler::new(audio_settings),
        }
    }
}

impl TestHostProcessor {
    // Allocate buffers that'll be used for CPAL -> VST audio forwarding
    pub fn prepare() {}

    pub unsafe fn cpal_process(&mut self, output: &mut [f32], _output_info: &OutputCallbackInfo) {
        let num_channels = self.audio_settings.channels();

        // Input generation section
        for frame in output.chunks_mut(num_channels) {
            for sample in frame.iter_mut() {
                let value = 0.0;
                *sample = value;
            }
        }

        // VST processing section
        self.buffer_handler.process(output);
        let mut audio_buffer = self.buffer_handler.get_audio_buffer();
        self.plugin_instance.process(&mut audio_buffer);

        // Flush plugin output to output
        flush_vst_output(output, num_channels, &mut audio_buffer)
    }

    pub fn cpal_error(err: StreamError) {
        log::error!("Stream error {:?}", err);
    }
}

unsafe fn flush_vst_output(
    output: &mut [f32],
    num_channels: usize,
    audio_buffer: &mut AudioBuffer<f32>,
) {
    let (_, plugin_output) = audio_buffer.split();
    for (sample_index, frame) in output.chunks_mut(num_channels).enumerate() {
        for (channel, sample) in frame.iter_mut().enumerate() {
            let channel_out = plugin_output.get(channel);
            let value = channel_out.get(sample_index).unwrap();
            *sample = *value;
        }
    }
}
