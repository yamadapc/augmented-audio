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
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};

use actix::prelude::*;
use thiserror::Error;
use vst::host::{PluginInstance, PluginLoadError, PluginLoader};
use vst::plugin::Plugin;

use audio_garbage_collector::Shared;
use audio_processor_standalone_midi::host::{MidiError, MidiHost};
use audio_processor_traits::{AudioProcessor, AudioProcessorSettings, SilenceAudioProcessor};

use crate::actor_system::ActorSystem;
use crate::audio_io::audio_thread;
use crate::audio_io::audio_thread::error::AudioThreadError;
use crate::audio_io::audio_thread::options::AudioThreadOptions;
use crate::audio_io::audio_thread::{AudioThread, AudioThreadProcessor};
use crate::processors::audio_file_processor::file_io::{default_read_audio_file, AudioFileError};
use crate::processors::audio_file_processor::InMemoryAudioFile;
use crate::processors::running_rms_processor::RunningRMSProcessorHandle;
use crate::processors::shared_processor::SharedProcessor;
use crate::processors::test_host_processor::TestHostProcessor;
use crate::processors::volume_meter_processor::VolumeMeterProcessorHandle;
use crate::vst_host::AudioTestHost;

#[derive(Debug, Error)]
pub enum AudioHostPluginLoadError {
    #[error(transparent)]
    PluginLoadError(#[from] PluginLoadError),
    #[error("Failed to find audio file path")]
    MissingPathError,
    #[error(transparent)]
    AudioFileError(#[from] AudioFileError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum StartError {
    #[error(transparent)]
    AudioThreadError(#[from] AudioThreadError),
    #[error(transparent)]
    MidiError(#[from] MidiError),
}

#[derive(Debug, Error)]
pub enum WaitError {
    #[error(transparent)]
    AudioThreadError(#[from] AudioThreadError),
}

pub struct TestPluginHost {
    audio_thread: Addr<AudioThread>,
    audio_settings: AudioProcessorSettings,
    audio_file_path: Option<PathBuf>,
    plugin_file_path: Option<PathBuf>,
    vst_plugin_instance: Option<SharedProcessor<PluginInstance>>,
    processor: Option<SharedProcessor<AudioThreadProcessor>>,
    midi_host: Addr<MidiHost>,
    mono_input: Option<usize>,
    temporary_load_path: Option<String>,
    start_paused: bool,
}

impl Default for TestPluginHost {
    fn default() -> Self {
        let audio_settings = AudioThread::default_settings().unwrap();
        let audio_thread_options = AudioThreadOptions::default();
        TestPluginHost::new(audio_settings, audio_thread_options, false)
    }
}

impl TestPluginHost {
    pub fn new(
        audio_settings: AudioProcessorSettings,
        audio_thread_options: AudioThreadOptions,
        start_paused: bool,
    ) -> Self {
        let actor_system_thread = ActorSystem::current();

        // At the moment there's a design problem here.
        // actix does not work well with multi-threading, but we relied on `ActorSystem` do handle
        // that.
        // Since it does not handle multi-threading well, it is disabled, however this block is
        // broken without it.
        log::info!("Starting MIDI thread");
        let midi_host = actor_system_thread.spawn_result(async move {
            log::info!("MIDI from registry");
            MidiHost::from_registry()
        });

        log::info!("Starting audio-thread");
        let audio_thread =
            actor_system_thread.spawn_result(async move { AudioThread::from_registry() });
        log::info!("Restoring audio-thread settings");
        audio_thread.do_send(audio_thread::actor::AudioThreadMessage::SetOptions {
            host_id: audio_thread_options.host_id,
            input_device_id: audio_thread_options.input_device_id,
            output_device_id: audio_thread_options.output_device_id,
        });

        log::info!("Init sequence complete");
        TestPluginHost {
            audio_thread,
            audio_settings,
            audio_file_path: None,
            plugin_file_path: None,
            vst_plugin_instance: None,
            processor: None,
            midi_host,
            mono_input: None,
            temporary_load_path: None,
            start_paused,
        }
    }

    pub fn audio_thread(&self) -> Addr<AudioThread> {
        self.audio_thread.clone()
    }

    pub fn start_audio(&mut self) -> Result<(), StartError> {
        // TODO - handle errors
        self.midi_host
            .do_send(audio_processor_standalone_midi::host::StartMessage);
        self.audio_thread
            .do_send(audio_thread::actor::AudioThreadMessage::Start);
        Ok(())
    }

    pub fn set_audio_file_path(&mut self, path: PathBuf) -> Result<(), AudioHostPluginLoadError> {
        self.audio_file_path = Some(path);
        if let Some(path) = self.plugin_file_path.clone() {
            self.load_plugin(path.as_path())?;
        }
        Ok(())
    }

    pub fn audio_file_path(&self) -> &Option<PathBuf> {
        &self.audio_file_path
    }

    pub fn plugin_file_path(&self) -> &Option<PathBuf> {
        &self.plugin_file_path
    }

    pub fn rms_processor_handle(&self) -> Option<Shared<RunningRMSProcessorHandle>> {
        self.host_processor()
            .map(|h| h.running_rms_processor_handle().clone())
    }

    pub fn load_plugin(&mut self, path: &Path) -> Result<(), AudioHostPluginLoadError> {
        self.plugin_file_path = Some(path.into());

        // Force the old plugin to be dropped
        self.audio_thread
            .do_send(audio_thread::actor::AudioThreadMessage::SetProcessor {
                processor: SharedProcessor::new(
                    audio_garbage_collector::handle(),
                    AudioThreadProcessor::Silence(SilenceAudioProcessor::new()),
                ),
            });

        let old_processor = self.processor.take();
        let old_plugin = self.vst_plugin_instance.take();
        std::mem::drop(old_processor);
        std::mem::drop(old_plugin);
        audio_garbage_collector::current().blocking_collect();

        let path = self.prepare_load_path(path)?;
        let path = Path::new(&path);
        let vst_plugin_instance = Self::load_vst_plugin(path)?;
        let vst_plugin_instance =
            SharedProcessor::new(audio_garbage_collector::handle(), vst_plugin_instance);

        let audio_settings = &self.audio_settings;
        let maybe_audio_file_settings = self.audio_file_path.as_ref().map_or(
            Ok(None),
            |audio_file_path| -> Result<Option<InMemoryAudioFile>, AudioHostPluginLoadError> {
                let audio_file = default_read_audio_file(
                    audio_file_path
                        .to_str()
                        .ok_or(AudioHostPluginLoadError::MissingPathError)?,
                )?;
                Ok(Some(InMemoryAudioFile::new(audio_file)))
            },
        )?;

        let mut test_host_processor = TestHostProcessor::new(
            audio_garbage_collector::handle(),
            maybe_audio_file_settings,
            vst_plugin_instance.clone(),
            audio_settings.sample_rate(),
            audio_settings.input_channels(),
            audio_settings.block_size(),
            self.mono_input,
        );
        test_host_processor.prepare(*audio_settings);

        if self.processor.is_none() && self.start_paused {
            test_host_processor.pause();
        }

        let test_host_processor = AudioThreadProcessor::Active(test_host_processor);
        let test_host_processor =
            SharedProcessor::new(audio_garbage_collector::handle(), test_host_processor);
        self.processor = Some(test_host_processor.clone());

        self.audio_thread
            .do_send(audio_thread::actor::AudioThreadMessage::SetProcessor {
                processor: test_host_processor,
            });

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

    /// Copy the dylib to a new location so it'll be properly reloaded on macOS. Clean-up the old
    /// temporary plugin file.
    fn prepare_load_path(&mut self, path: &Path) -> Result<String, std::io::Error> {
        if let Some(temporary_load_path) = self.temporary_load_path.take() {
            log::info!("Cleaning-up temporary plugin at {}", temporary_load_path);
            std::fs::remove_file(&temporary_load_path)?;
        }

        let load_id = uuid::Uuid::new_v4().to_string();
        let load_path = format!("/tmp/plugin-host-{}", load_id);
        std::fs::copy(path, &load_path)?;
        log::info!("Copied plugin into {}", &load_path);
        self.temporary_load_path = Some(load_path.clone());

        #[cfg(target_os = "macos")]
        {
            log::info!("Tainting the library with install_name_tool. Multiple versions will be loaded at the same time due to macOS limitations.");
            let _ = Command::new("install_name_tool")
                .args(&["-id", &load_id, &load_path])
                .output();
        }

        Ok(load_path)
    }

    fn host_processor(&self) -> Option<&TestHostProcessor> {
        self.processor.as_ref().and_then(|processor| {
            if let AudioThreadProcessor::Active(host) = processor.deref() {
                Some(host)
            } else {
                None
            }
        })
    }

    pub fn volume_handle(&self) -> Option<Shared<VolumeMeterProcessorHandle>> {
        self.host_processor().map(|p| p.volume_handle().clone())
    }

    /// Resume playback
    pub fn play(&self) {
        if let Some(processor) = self.host_processor() {
            log::info!("Starting playback processor_id={}", processor.id());
            processor.play();
        }
    }

    /// Pause playback
    pub fn pause(&self) {
        if let Some(processor) = self.host_processor() {
            log::info!("Pausing playback processor_id={}", processor.id());
            processor.pause();
        }
    }

    /// Stop playback and go back to the start of the file
    pub fn stop(&self) {
        if let Some(processor) = self.host_processor() {
            log::info!("Stopping playback processor_id={}", processor.id());
            processor.stop();
        }
    }

    pub fn plugin_instance(&mut self) -> Option<SharedProcessor<PluginInstance>> {
        self.vst_plugin_instance.clone()
    }

    pub fn wait(&mut self) -> Result<(), WaitError> {
        self.audio_thread
            .do_send(audio_thread::actor::AudioThreadMessage::Wait);

        Ok(())
    }

    pub fn set_mono_input(&mut self, input_channel: Option<usize>) {
        self.mono_input = input_channel;
    }

    pub fn set_volume(&mut self, volume: f32) {
        if let Some(processor) = self.host_processor() {
            processor.set_volume(volume);
        }
    }
}

impl Drop for TestPluginHost {
    fn drop(&mut self) {
        self.stop();
        if let Some(temporary_load_path) = self.temporary_load_path.take() {
            log::warn!("Cleaning-up temporary plug-in file {}", temporary_load_path);
            let _ = std::fs::remove_file(temporary_load_path);
        }
    }
}

impl Supervised for TestPluginHost {}

impl SystemService for TestPluginHost {}

impl Actor for TestPluginHost {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("TestPluginHost started");
    }
}

#[derive(Message)]
#[rtype(result = "Option<SharedProcessor<PluginInstance>>")]
pub struct GetPluginInstanceMessage;

impl Handler<GetPluginInstanceMessage> for TestPluginHost {
    type Result = Option<SharedProcessor<PluginInstance>>;

    fn handle(&mut self, _msg: GetPluginInstanceMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.vst_plugin_instance.clone()
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), AudioHostPluginLoadError>")]
pub struct LoadPluginMessage {
    pub plugin_path: PathBuf,
}

impl Handler<LoadPluginMessage> for TestPluginHost {
    type Result = Result<(), AudioHostPluginLoadError>;

    fn handle(&mut self, msg: LoadPluginMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.load_plugin(&msg.plugin_path)
    }
}

#[derive(Message)]
#[rtype(result = "Addr<AudioThread>")]
pub struct GetAudioThreadMessage;

impl Handler<GetAudioThreadMessage> for TestPluginHost {
    type Result = Addr<AudioThread>;

    fn handle(&mut self, _msg: GetAudioThreadMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.audio_thread.clone()
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), AudioHostPluginLoadError>")]
pub struct SetAudioFilePathMessage(pub PathBuf);

impl Handler<SetAudioFilePathMessage> for TestPluginHost {
    type Result = Result<(), AudioHostPluginLoadError>;

    fn handle(&mut self, msg: SetAudioFilePathMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.set_audio_file_path(msg.0)
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct WaitMessage;

impl Handler<WaitMessage> for TestPluginHost {
    type Result = ();

    fn handle(&mut self, _msg: WaitMessage, _ctx: &mut Self::Context) -> Self::Result {
        match self.wait() {
            Ok(_) => log::info!("Plugin host stopped"),
            Err(err) => log::error!("Failed to stop: {}", err),
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct StartMessage;

impl Handler<StartMessage> for TestPluginHost {
    type Result = ();

    fn handle(&mut self, _msg: StartMessage, _ctx: &mut Self::Context) -> Self::Result {
        let _ = self.start_audio(); // todo - log error
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct StopMessage;

impl Handler<StopMessage> for TestPluginHost {
    type Result = ();

    fn handle(&mut self, _msg: StopMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.stop();
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), AudioHostPluginLoadError>")]
pub struct ReloadPluginMessage;

impl Handler<ReloadPluginMessage> for TestPluginHost {
    type Result = Result<(), AudioHostPluginLoadError>;

    fn handle(&mut self, _msg: ReloadPluginMessage, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(plugin_file_path) = self.plugin_file_path().clone() {
            self.load_plugin(&plugin_file_path)?;
        }
        Ok(())
    }
}
