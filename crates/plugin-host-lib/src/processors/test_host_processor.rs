use std::ops::Deref;

use vst::host::PluginInstance;
use vst::plugin::Plugin;

use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};

use crate::audio_io::cpal_vst_buffer_handler::CpalVstBufferHandler;
use crate::processors::audio_file_processor::{AudioFileProcessor, AudioFileSettings};
use crate::processors::shared_processor::SharedProcessor;
use crate::processors::volume_meter_processor::VolumeMeterProcessor;

/// The app's main processor
pub struct TestHostProcessor {
    plugin_instance: SharedProcessor<PluginInstance>,
    audio_settings: AudioProcessorSettings,
    buffer_handler: CpalVstBufferHandler,
    audio_file_processor: AudioFileProcessor,
    volume_meter_processor: VolumeMeterProcessor,
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
            volume_meter_processor: VolumeMeterProcessor::new(),
        }
    }

    pub fn current_output_volume(&self) -> (f32, f32) {
        self.volume_meter_processor.current_volume()
    }

    /// Resume playback
    pub fn play(&self) {
        self.audio_file_processor.play();
    }

    /// Pause playback
    pub fn pause(&self) {
        self.audio_file_processor.pause();
    }

    /// Stop playback and go back to the start of the file
    pub fn stop(&self) {
        self.audio_file_processor.stop();
    }

    /// Whether the file is being played back
    pub fn is_playing(&self) -> bool {
        self.audio_file_processor.is_playing()
    }
}

impl AudioProcessor for TestHostProcessor {
    type SampleType = f32;

    fn prepare(&mut self, audio_settings: AudioProcessorSettings) {
        self.audio_settings = audio_settings;
        self.buffer_handler.prepare(&audio_settings);
        self.audio_file_processor.prepare(audio_settings);
        self.volume_meter_processor.prepare(audio_settings);
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        output: &mut BufferType,
    ) {
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
        flush_vst_output(num_channels, &mut audio_buffer, output);

        // Volume meter
        self.volume_meter_processor.process(output);
    }
}

impl Drop for TestHostProcessor {
    fn drop(&mut self) {
        log::warn!("Dropping test host processor");
    }
}

/// Flush plugin output to output
pub fn flush_vst_output<BufferType: AudioBuffer<SampleType = f32>>(
    num_channels: usize,
    audio_buffer: &mut vst::buffer::AudioBuffer<f32>,
    output: &mut BufferType,
) {
    let (_, plugin_output) = audio_buffer.split();
    let channel_outs: Vec<&[f32]> = (0..num_channels)
        .into_iter()
        .map(|channel| plugin_output.get(channel))
        .collect();

    for sample_index in 0..output.num_samples() {
        for channel in 0..output.num_channels() {
            output.set(channel, sample_index, channel_outs[channel][sample_index]);
        }
    }
}
