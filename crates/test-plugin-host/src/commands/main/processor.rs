use crate::commands::main::audio_file_processor::{AudioFileProcessor, AudioFileSettings};
use crate::commands::main::audio_settings::AudioSettings;
use crate::commands::main::cpal_vst_buffer_handler::CpalVstBufferHandler;
use cpal::{OutputCallbackInfo, StreamError};
use vst::buffer::AudioBuffer;
use vst::host::PluginInstance;
use vst::plugin::Plugin;

/// The app's main processor
pub struct TestHostProcessor {
    plugin_instance: *mut PluginInstance,
    audio_settings: AudioSettings,
    buffer_handler: CpalVstBufferHandler,
    audio_file_processor: AudioFileProcessor,
}

unsafe impl Send for TestHostProcessor {}
unsafe impl Sync for TestHostProcessor {}

impl TestHostProcessor {
    pub fn new(
        audio_file_settings: AudioFileSettings,
        plugin_instance: *mut PluginInstance,
        sample_rate: f32,
        channels: usize,
        buffer_size: u32,
    ) -> Self {
        let audio_settings = AudioSettings::new(sample_rate, channels, buffer_size);
        TestHostProcessor {
            plugin_instance,
            audio_settings,
            buffer_handler: CpalVstBufferHandler::new(audio_settings),
            audio_file_processor: AudioFileProcessor::new(audio_file_settings, audio_settings),
        }
    }
}

impl TestHostProcessor {
    pub fn prepare(&mut self, audio_settings: AudioSettings) {
        self.audio_settings = audio_settings;
        self.buffer_handler.prepare(&audio_settings);
        self.audio_file_processor.prepare(audio_settings);
    }

    pub unsafe fn cpal_process(&mut self, output: &mut [f32], _output_info: &OutputCallbackInfo) {
        let num_channels = self.audio_settings.channels();

        // Input generation section
        self.audio_file_processor.process(output);

        // VST processing section
        self.buffer_handler.process(output);
        let mut audio_buffer = self.buffer_handler.get_audio_buffer();
        self.plugin_instance
            .as_mut()
            .unwrap()
            .process(&mut audio_buffer);
        flush_vst_output(num_channels, &mut audio_buffer, output)
    }

    pub fn cpal_error(err: StreamError) {
        log::error!("Stream error {:?}", err);
    }
}

/// Flush plugin output to output
fn flush_vst_output(num_channels: usize, audio_buffer: &mut AudioBuffer<f32>, output: &mut [f32]) {
    let (_, plugin_output) = audio_buffer.split();
    for (sample_index, frame) in output.chunks_mut(num_channels).enumerate() {
        for (channel, sample) in frame.iter_mut().enumerate() {
            let channel_out = plugin_output.get(channel);
            let value = channel_out.get(sample_index).unwrap();
            *sample = *value;
        }
    }
}
