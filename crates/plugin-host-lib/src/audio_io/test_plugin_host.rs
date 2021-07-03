use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use thiserror::Error;
use vst::host::{PluginInstance, PluginLoadError, PluginLoader};
use vst::plugin::Plugin;

use audio_garbage_collector::{GarbageCollector, GarbageCollectorError};
use audio_processor_standalone_midi::host::{MidiError, MidiHost};
use audio_processor_traits::{AudioProcessor, AudioProcessorSettings};

use crate::audio_io::audio_thread::error::AudioThreadError;
use crate::audio_io::audio_thread::options::AudioDeviceId;
use crate::audio_io::audio_thread::{AudioThread, AudioThreadProcessor};
use crate::processors::audio_file_processor::{
    default_read_audio_file, AudioFileError, AudioFileSettings,
};
use crate::processors::shared_processor::SharedProcessor;
use crate::processors::test_host_processor::TestHostProcessor;
use crate::vst_host::AudioTestHost;

#[derive(Debug, Error)]
pub enum AudioHostPluginLoadError {
    #[error("Failed to load VST plugin")]
    PluginLoadError(#[from] PluginLoadError),
    #[error("Failed to find audio file path")]
    MissingPathError,
    #[error("Failed to open or decode the audio file")]
    AudioFileError(#[from] AudioFileError),
}

#[derive(Debug, Error)]
pub enum StartError {
    #[error("Failed to start the audio thread")]
    AudioThreadError(#[from] AudioThreadError),
    #[error("Failed to start MIDI handler")]
    MidiError(#[from] MidiError),
}

#[derive(Debug, Error)]
pub enum WaitError {
    #[error("Failed to stop the GC thread")]
    GarbageCollectorError(#[from] GarbageCollectorError),
    #[error("Failed to wait on the audio thread")]
    AudioThreadError(#[from] AudioThreadError),
}

pub struct TestPluginHost {
    audio_thread: AudioThread,
    audio_settings: AudioProcessorSettings,
    audio_file_path: PathBuf,
    plugin_file_path: Option<PathBuf>,
    vst_plugin_instance: Option<SharedProcessor<PluginInstance>>,
    processor: Option<SharedProcessor<AudioThreadProcessor>>,
    midi_host: MidiHost,
    garbage_collector: GarbageCollector,
}

impl Default for TestPluginHost {
    fn default() -> Self {
        let audio_settings = AudioThread::default_settings().unwrap();
        log::info!(
            "\
            Using audio settings:\n\t\
                Sample rate: {}\n\t\
                Block size: {}\n\t\
                Input channels: {}\n\t\
                Output channels: {}\
            ",
            audio_settings.sample_rate(),
            audio_settings.block_size(),
            audio_settings.input_channels(),
            audio_settings.output_channels()
        );
        TestPluginHost::new(audio_settings)
    }
}

impl TestPluginHost {
    pub fn new(audio_settings: AudioProcessorSettings) -> Self {
        let path = Path::new("").to_path_buf();
        let garbage_collector = GarbageCollector::new(Duration::from_secs(1));
        let midi_host = MidiHost::default_with_handle(garbage_collector.handle());

        TestPluginHost {
            audio_thread: AudioThread::new(
                garbage_collector.handle(),
                midi_host.messages().clone(),
            ),
            audio_settings,
            audio_file_path: path,
            plugin_file_path: None,
            vst_plugin_instance: None,
            processor: None,
            midi_host,
            garbage_collector,
        }
    }

    pub fn start(&mut self) -> Result<(), StartError> {
        self.midi_host.start()?;
        self.audio_thread.start()?;
        Ok(())
    }

    pub fn set_output_device_id(
        &mut self,
        output_device_id: AudioDeviceId,
    ) -> Result<(), AudioThreadError> {
        self.audio_thread.set_output_device_id(output_device_id)?;
        Ok(())
    }

    pub fn set_audio_file_path(&mut self, path: PathBuf) -> Result<(), AudioHostPluginLoadError> {
        self.audio_file_path = path;
        if let Some(path) = self.plugin_file_path.clone() {
            self.load_plugin(path.as_path())?;
        }
        Ok(())
    }

    pub fn audio_file_path(&self) -> &PathBuf {
        &self.audio_file_path
    }

    pub fn plugin_file_path(&self) -> &Option<PathBuf> {
        &self.plugin_file_path
    }

    pub fn load_plugin(&mut self, path: &Path) -> Result<(), AudioHostPluginLoadError> {
        self.plugin_file_path = Some(path.into());
        let vst_plugin_instance = Self::load_vst_plugin(path)?;
        let vst_plugin_instance =
            SharedProcessor::new(self.garbage_collector.handle(), vst_plugin_instance);

        let audio_settings = &self.audio_settings;
        let audio_file = default_read_audio_file(
            &self
                .audio_file_path
                .to_str()
                .ok_or(AudioHostPluginLoadError::MissingPathError)?,
        )?;
        let audio_file_settings = AudioFileSettings::new(audio_file);

        let mut test_host_processor = TestHostProcessor::new(
            audio_file_settings,
            vst_plugin_instance.clone(),
            audio_settings.sample_rate(),
            audio_settings.input_channels(),
            audio_settings.block_size(),
        );
        test_host_processor.prepare(*audio_settings);
        let test_host_processor = AudioThreadProcessor::Active(test_host_processor);
        let test_host_processor =
            SharedProcessor::new(self.garbage_collector.handle(), test_host_processor);
        self.processor = Some(test_host_processor.clone());
        self.audio_thread.set_processor(test_host_processor);

        // De-allocate old instance
        self.vst_plugin_instance = Some(vst_plugin_instance);
        Ok(())
    }

    pub(crate) fn load_vst_plugin(path: &Path) -> Result<PluginInstance, AudioHostPluginLoadError> {
        let host = Arc::new(Mutex::new(AudioTestHost));
        let mut loader = PluginLoader::load(path, Arc::clone(&host))?;
        let mut instance = loader.instance()?;
        let info = instance.get_info();
        log::info!(
            "Loaded '{}':\n\t\
             Vendor: {}\n\t\
             Presets: {}\n\t\
             Parameters: {}\n\t\
             VST ID: {}\n\t\
             Version: {}\n\t\
             Initial Delay: {} samples",
            info.name,
            info.vendor,
            info.presets,
            info.parameters,
            info.unique_id,
            info.version,
            info.initial_delay
        );
        // Initialize the instance
        instance.init();
        log::info!("Initialized instance!");
        Ok(instance)
    }

    fn host_processor(&self) -> Option<&TestHostProcessor> {
        self.processor
            .as_ref()
            .map(|processor| {
                if let AudioThreadProcessor::Active(host) = processor.deref() {
                    Some(host)
                } else {
                    None
                }
            })
            .flatten()
    }

    pub fn current_volume(&self) -> (f32, f32) {
        self.host_processor()
            .map(|p| p.current_output_volume())
            .unwrap_or((0.0, 0.0))
    }

    /// Resume playback
    pub fn play(&self) {
        if let Some(processor) = self.host_processor() {
            processor.play();
        }
    }

    /// Pause playback
    pub fn pause(&self) {
        if let Some(processor) = self.host_processor() {
            processor.pause();
        }
    }

    /// Stop playback and go back to the start of the file
    pub fn stop(&self) {
        if let Some(processor) = self.host_processor() {
            processor.stop();
        }
    }

    /// Whether the file is being played back
    pub fn is_playing(&self) -> bool {
        self.host_processor()
            .map(|p| p.is_playing())
            .unwrap_or(false)
    }

    pub fn plugin_instance(&mut self) -> Option<SharedProcessor<PluginInstance>> {
        self.vst_plugin_instance.clone()
    }

    pub fn wait(&mut self) -> Result<(), WaitError> {
        self.garbage_collector.stop()?;
        Ok(self.audio_thread.wait()?)
    }
}
