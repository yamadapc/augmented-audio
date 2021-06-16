use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use thiserror::Error;
use vst::host::{PluginInstance, PluginLoadError, PluginLoader};
use vst::plugin::Plugin;

pub use audio_io_service::*;
use audio_thread::AudioThread;
use cpal_vst_buffer_handler::CpalVstBufferHandler;

use crate::audio_settings::AudioSettings;
use crate::processors::audio_file_processor::{
    default_read_audio_file, AudioFileError, AudioFileProcessor, AudioFileSettings,
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

pub struct AudioHost {
    audio_thread: AudioThread,
    audio_settings: AudioSettings,
    audio_file_path: PathBuf,
    vst_plugin_instance: Option<Box<PluginInstance>>,
}

impl AudioHost {
    pub fn new(audio_settings: AudioSettings) -> Self {
        let path = Path::new("").to_path_buf();
        AudioHost {
            audio_thread: AudioThread::new(),
            audio_settings,
            audio_file_path: path,
            vst_plugin_instance: None,
        }
    }

    pub fn start(&mut self) {
        self.audio_thread.start();
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

        let mut test_host_processor = TestHostProcessor::new(
            audio_file_settings,
            (&mut instance) as *mut PluginInstance,
            audio_settings.sample_rate(),
            audio_settings.channels(),
            audio_settings.buffer_size(),
        );

        // De-allocate old instance
        self.vst_plugin_instance = Some(Box::new(instance));
        Ok(())
    }
}
