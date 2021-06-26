use std::ops::Deref;

use vst::buffer::AudioBuffer;
use vst::host::PluginInstance;
use vst::plugin::Plugin;

use audio_processor_traits::{AudioProcessor, AudioProcessorSettings};

use crate::audio_io::cpal_vst_buffer_handler::CpalVstBufferHandler;
use crate::processors::audio_file_processor::{AudioFileProcessor, AudioFileSettings};
use crate::processors::shared_processor::SharedProcessor;

/// The app's main processor
pub struct TestHostProcessor {
    plugin_instance: SharedProcessor<PluginInstance>,
    audio_settings: AudioProcessorSettings,
    buffer_handler: CpalVstBufferHandler,
    audio_file_processor: AudioFileProcessor,
}

unsafe impl Send for TestHostProcessor {}
unsafe impl Sync for TestHostProcessor {}

impl TestHostProcessor {
    pub fn new(
        audio_file_settings: AudioFileSettings,
        plugin_instance: SharedProcessor<PluginInstance>,
        sample_rate: f32,
        channels: usize,
        buffer_size: u32,
    ) -> Self {
        let audio_settings =
            AudioProcessorSettings::new(sample_rate, channels, channels, buffer_size);
        TestHostProcessor {
            plugin_instance,
            audio_settings,
            buffer_handler: CpalVstBufferHandler::new(audio_settings),
            audio_file_processor: AudioFileProcessor::new(audio_file_settings, audio_settings),
        }
    }
}

impl AudioProcessor for TestHostProcessor {
    fn prepare(&mut self, audio_settings: AudioProcessorSettings) {
        self.audio_settings = audio_settings;
        self.buffer_handler.prepare(&audio_settings);
        self.audio_file_processor.prepare(audio_settings);
    }

    fn process(&mut self, output: &mut [f32]) {
        let num_channels = self.audio_settings.input_channels();

        // Input generation section
        self.audio_file_processor.process(output);

        // VST processing section
        self.buffer_handler.process(output);
        let mut audio_buffer = self.buffer_handler.get_audio_buffer();
        unsafe {
            let instance =
                self.plugin_instance.deref() as *const PluginInstance as *mut PluginInstance;
            (*instance).process(&mut audio_buffer);
        }
        flush_vst_output(num_channels, &mut audio_buffer, output)
    }
}

impl Drop for TestHostProcessor {
    fn drop(&mut self) {
        log::warn!("Dropping test host processor");
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
