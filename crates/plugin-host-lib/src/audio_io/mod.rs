use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use thiserror::Error;
use vst::host::{PluginInstance, PluginLoadError, PluginLoader};
use vst::plugin::Plugin;

pub use audio_io_service::*;
use audio_processor_traits::{AudioProcessor, AudioProcessorSettings};
use audio_thread::error::AudioThreadError;
use audio_thread::AudioThread;
use garbage_collector::GarbageCollector;
use garbage_collector::GarbageCollectorError;

use crate::audio_io::audio_thread::options::AudioDeviceId;
use crate::audio_io::audio_thread::AudioThreadProcessor;
use crate::processors::audio_file_processor::{
    default_read_audio_file, AudioFileError, AudioFileSettings,
};
use crate::processors::shared_processor::SharedProcessor;
use crate::processors::test_host_processor::TestHostProcessor;
use crate::vst_host::AudioTestHost;

pub mod audio_io_service;
pub mod audio_thread;
pub mod cpal_vst_buffer_handler;
mod garbage_collector;

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
        TestPluginHost {
            audio_thread: AudioThread::new(garbage_collector.handle()),
            audio_settings,
            audio_file_path: path,
            plugin_file_path: None,
            vst_plugin_instance: None,
            garbage_collector,
        }
    }

    pub fn start(&mut self) -> Result<(), StartError> {
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

    pub fn load_plugin(&mut self, path: &Path) -> Result<(), AudioHostPluginLoadError> {
        self.plugin_file_path = Some(path.into());
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

        let audio_settings = &self.audio_settings;
        let audio_file = default_read_audio_file(
            &self
                .audio_file_path
                .to_str()
                .ok_or(AudioHostPluginLoadError::MissingPathError)?,
        )?;
        let audio_file_settings = AudioFileSettings::new(audio_file);
        let instance = SharedProcessor::new(self.garbage_collector.handle(), instance);

        let mut test_host_processor = TestHostProcessor::new(
            audio_file_settings,
            instance.clone(),
            audio_settings.sample_rate(),
            audio_settings.input_channels(),
            audio_settings.block_size(),
        );
        test_host_processor.prepare(*audio_settings);
        let test_host_processor = AudioThreadProcessor::Active(test_host_processor);
        let test_host_processor =
            SharedProcessor::new(self.garbage_collector.handle(), test_host_processor);
        self.audio_thread.set_processor(test_host_processor);

        // De-allocate old instance
        self.vst_plugin_instance = Some(instance);
        Ok(())
    }

    pub fn plugin_instance(&mut self) -> Option<SharedProcessor<PluginInstance>> {
        self.vst_plugin_instance.clone()
    }

    pub fn wait(&mut self) -> Result<(), WaitError> {
        self.garbage_collector.stop()?;
        Ok(self.audio_thread.wait()?)
    }
}
