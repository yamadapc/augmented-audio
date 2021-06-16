use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use thiserror::Error;
use vst::host::{PluginInstance, PluginLoadError, PluginLoader};
use vst::plugin::Plugin;

pub use audio_io_service::*;
use audio_processor_traits::AudioProcessorSettings;
use audio_thread::AudioThread;

use crate::audio_io::audio_thread::AudioThreadError;
use crate::audio_settings::AudioSettings;
use crate::processors::audio_file_processor::{
    default_read_audio_file, AudioFileError, AudioFileSettings,
};
use crate::processors::test_host_processor::TestHostProcessor;
use crate::vst_host::AudioTestHost;

pub mod audio_io_service;
pub mod audio_thread;
pub mod cpal_vst_buffer_handler;

#[derive(Debug, Error)]
pub enum AudioHostPluginLoadError {
    #[error("Failed to load VST plugin")]
    PluginLoadError(#[from] PluginLoadError),
    #[error("Failed to find audio file path")]
    MissingPathError,
    #[error("Failed to open or decode the audio file")]
    AudioFileError(#[from] AudioFileError),
}

struct UnsafePluginRef(*mut PluginInstance);
unsafe impl Send for UnsafePluginRef {}
unsafe impl Sync for UnsafePluginRef {}

pub struct TestPluginHost {
    audio_thread: AudioThread,
    audio_settings: AudioProcessorSettings,
    audio_file_path: PathBuf,
    vst_plugin_instance: Option<Box<PluginInstance>>,
}

impl Default for TestPluginHost {
    fn default() -> Self {
        let audio_settings = AudioThread::settings().unwrap();
        TestPluginHost::new(audio_settings)
    }
}

impl TestPluginHost {
    pub fn new(audio_settings: AudioProcessorSettings) -> Self {
        let path = Path::new("").to_path_buf();
        TestPluginHost {
            audio_thread: AudioThread::new(),
            audio_settings,
            audio_file_path: path,
            vst_plugin_instance: None,
        }
    }

    pub fn start(&mut self) {
        self.audio_thread.start();
    }

    pub fn set_audio_file_path(&mut self, path: PathBuf) {
        self.audio_file_path = path;
    }

    pub fn load_plugin(&mut self, path: &Path) -> Result<(), AudioHostPluginLoadError> {
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

        let test_host_processor = Box::new(TestHostProcessor::new(
            audio_file_settings,
            (&mut instance) as *mut PluginInstance,
            audio_settings.sample_rate(),
            audio_settings.input_channels(),
            audio_settings.block_size(),
        ));
        self.audio_thread.set_processor(test_host_processor);

        // De-allocate old instance
        self.vst_plugin_instance = Some(Box::new(instance));
        Ok(())
    }

    pub fn plugin_instance(&mut self) -> *mut PluginInstance {
        self.vst_plugin_instance.as_mut().unwrap().as_mut() as *mut PluginInstance
    }

    pub fn wait(self) -> Result<(), AudioThreadError> {
        self.audio_thread.wait()
    }
}
