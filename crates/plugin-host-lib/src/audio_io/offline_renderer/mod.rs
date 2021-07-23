use std::time::{Duration, Instant};

use thiserror::Error;
use vst::plugin::Plugin;

use audio_processor_traits::{AudioProcessorSettings, InterleavedAudioBuffer};

use crate::audio_io::cpal_vst_buffer_handler::CpalVstBufferHandler;
use crate::audio_io::AudioHostPluginLoadError;
use crate::processors::audio_file_processor::file_io::AudioFileError;
use crate::processors::audio_file_processor::AudioFileProcessor;
use crate::processors::output_file_processor::OutputAudioFileProcessor;
use crate::processors::test_host_processor::flush_vst_output;
use crate::TestPluginHost;

#[derive(Debug, Error)]
pub enum OfflineRenderError {
    #[error("Failed to open or decode the audio file")]
    AudioFileError(#[from] AudioFileError),
    #[error("Failed to load plug-in")]
    AudioHostPluginLoadError(#[from] AudioHostPluginLoadError),
}

pub struct OfflineRenderer {
    audio_settings: AudioProcessorSettings,
    input_file_path: String,
    output_file_path: String,
    plugin_path: String,
}

impl OfflineRenderer {
    pub fn new(
        audio_settings: AudioProcessorSettings,
        input_file_path: &str,
        output_file_path: &str,
        plugin_path: &str,
    ) -> OfflineRenderer {
        OfflineRenderer {
            audio_settings,
            input_file_path: String::from(input_file_path),
            output_file_path: String::from(output_file_path),
            plugin_path: String::from(plugin_path),
        }
    }

    pub fn run(&self) -> Result<OfflineRenderDiagnostics, OfflineRenderError> {
        let mut buffer_handler = CpalVstBufferHandler::new(self.audio_settings);
        let mut audio_file_processor =
            AudioFileProcessor::from_path(self.audio_settings, &self.input_file_path)?;
        let mut plugin = TestPluginHost::load_vst_plugin(self.plugin_path.as_ref())?;
        let mut output_file_processor =
            OutputAudioFileProcessor::from_path(self.audio_settings, &self.output_file_path);

        plugin.set_sample_rate(self.audio_settings.sample_rate());
        plugin.set_block_size(self.audio_settings.block_size() as i64);
        audio_file_processor.prepare(self.audio_settings);
        output_file_processor.prepare(self.audio_settings);

        let audio_file_buffer = audio_file_processor.buffer();
        let audio_file_total_samples = audio_file_buffer[0].len();
        let num_channels = audio_file_buffer.len();
        let block_size = self.audio_settings.block_size() as usize;
        let total_blocks = audio_file_total_samples / block_size;
        log::info!("Going to process input file with {} blocks", total_blocks);

        let mut buffer = Vec::new();
        buffer.resize(block_size * self.audio_settings.input_channels(), 0.0);

        let mut audio_file_position = 0;
        let start = Instant::now();
        let mut audio_input_conversion_time = Duration::from_millis(0);
        let mut audio_output_time = Duration::from_millis(0);
        let mut plugin_time = Duration::from_millis(0);
        let mut plugin_conversions_time = Duration::from_millis(0);
        let mut audio_buffer_create_time = Duration::from_millis(0);
        let mut plugin_flush_time = Duration::from_millis(0);

        for _block_num in 0..total_blocks {
            let start = Instant::now();
            let mut channel_number = 0;
            #[allow(clippy::explicit_counter_loop)]
            for channel in audio_file_buffer {
                for i in 0..block_size {
                    let interleaved_index = i * num_channels + channel_number;
                    buffer[interleaved_index] = channel[audio_file_position + i]
                }
                channel_number += 1;
            }
            audio_file_position += block_size;
            audio_input_conversion_time += start.elapsed();

            let start = Instant::now();
            let mut interleaved_buffer = InterleavedAudioBuffer::new(num_channels, &mut buffer);
            buffer_handler.process(&interleaved_buffer);
            let audio_buffer_start = Instant::now();
            let mut audio_plugin_buffer = buffer_handler.get_audio_buffer();
            audio_buffer_create_time += audio_buffer_start.elapsed();
            plugin_conversions_time += start.elapsed();

            let start = Instant::now();
            plugin.process(&mut audio_plugin_buffer);
            plugin_time += start.elapsed();

            let start = Instant::now();
            let flush_start = Instant::now();
            flush_vst_output(
                num_channels,
                &mut audio_plugin_buffer,
                &mut interleaved_buffer,
            );
            plugin_flush_time += flush_start.elapsed();
            plugin_conversions_time += start.elapsed();

            let start = Instant::now();
            output_file_processor.process(&mut buffer);
            audio_output_time += start.elapsed();
        }
        let total_runtime = start.elapsed().as_millis();

        log::info!(
            "Output conversions duration={}ms",
            audio_output_time.as_millis()
        );
        log::info!(
            "Input conversions duration={}ms",
            audio_input_conversion_time.as_millis()
        );
        log::info!(
            "Plugin conversions duration={}ms - audio_buffer_create={}ms - flush_time={}ms",
            plugin_conversions_time.as_millis(),
            audio_buffer_create_time.as_millis(),
            plugin_flush_time.as_millis()
        );
        log::info!("Plugin runtime duration={}ms", plugin_time.as_millis());
        log::info!("Total runtime duration={}ms", total_runtime);

        let audio_duration =
            (total_blocks as f32 * block_size as f32) / self.audio_settings.sample_rate();
        let audio_duration = Duration::from_secs_f32(audio_duration);
        log::info!("Audio duration : {}ms", audio_duration.as_millis());
        let realtime_relation = audio_duration.as_millis() as f32 / total_runtime as f32;
        log::info!("{:.1}x realtime", realtime_relation);

        Ok(OfflineRenderDiagnostics {
            plugin_own_time: plugin_time,
            total_runtime: Duration::from_millis(total_runtime as u64),
            host_overhead_diagnostics: HostOverheadDiagnostics {
                audio_output_conversions_time: audio_output_time,
                audio_buffer_create_time,
                audio_input_conversions_time: audio_input_conversion_time,
                plugin_conversions_time,
                plugin_flush_time,
            },
            output_audio_duration: audio_duration,
            realtime_ration: realtime_relation,
        })
    }
}

pub struct OfflineRenderDiagnostics {
    /// Time spent inside the VST
    pub plugin_own_time: Duration,
    /// Total time to render
    pub total_runtime: Duration,
    /// Diagnostics over overhead the plugin-host causes (conversions between VST, CPAL, WAV types)
    pub host_overhead_diagnostics: HostOverheadDiagnostics,
    /// How many seconds of data was written to the AUDIO OUTPUT FILE
    pub output_audio_duration: Duration,
    /// Ratio between the offline render performance and a real-time workload
    /// e.g. 10x real-time
    pub realtime_ration: f32,
}

pub struct HostOverheadDiagnostics {
    /// Time spent converting samples/buffers into WAV
    pub audio_output_conversions_time: Duration,
    /// Time spent converting CPal interleaved samples into VST buffers for input processing
    pub audio_input_conversions_time: Duration,
    /// Time spent converting VST buffers into CPal interleaved samples for flushing plugin output
    pub plugin_conversions_time: Duration,
    /// Time to create audio buffers
    pub audio_buffer_create_time: Duration,
    /// Time to copy VST output into the output buffer
    pub plugin_flush_time: Duration,
}
