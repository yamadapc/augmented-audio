use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use plugin_host_lib::audio_io::test_plugin_host::TestPluginHost;
use plugin_host_lib::audio_io::AudioIOService;

use crate::config::AppConfig;
use crate::services::host_options_service::HostOptionsService;
use crate::services::volume_publisher::VolumePublisherService;

pub type AppStateRef = Arc<Mutex<AppState>>;

pub struct AppState {
  /// TODO - Double nested locks :P
  host: Arc<Mutex<TestPluginHost>>,
  audio_io_service: AudioIOService,
  volume_publisher_service: VolumePublisherService<TestPluginHost>,
  audio_thread_options_service: HostOptionsService,
}

impl AppState {
  pub fn new(host: plugin_host_lib::TestPluginHost, app_config: AppConfig) -> Self {
    let host = Arc::new(Mutex::new(host));
    AppState {
      host: host.clone(),
      audio_io_service: AudioIOService::new(host.clone(), app_config.storage_config.clone()),
      volume_publisher_service: VolumePublisherService::new(host, Duration::from_millis(100)),
      audio_thread_options_service: HostOptionsService::new(app_config.audio_thread_config_path),
    }
  }

  pub fn try_reload(&mut self) {
    match self.audio_io_service.reload() {
      Ok(_) => {
        log::info!("Reloaded Audio IO configuration from disk");
      }
      Err(err) => {
        log::warn!("Failed to load Audio IO configuration from disk: {:?}", err);
      }
    }

    match self.audio_thread_options_service.fetch() {
      Ok(config) => {
        log::info!("Reloaded audio thread configuration from disk. Configuring audio host.");
        let mut host = self.host.lock().unwrap();
        if let Some(audio_input_file_path) = &config.audio_input_file_path {
          if let Err(err) = host.set_audio_file_path(PathBuf::from(audio_input_file_path)) {
            log::error!("Failed to set input file path: {}", err);
          }
        }

        if let Some(plugin_path) = &config.plugin_path {
          let plugin_path = PathBuf::from(plugin_path);
          if let Err(err) = host.load_plugin(&plugin_path) {
            log::error!("Failed to load plugin: {}", err);
          }
        }
      }
      Err(err) => {
        log::warn!(
          "Failed to load audio thread configuration from disk: {:?}",
          err
        );
      }
    }

    log::info!("State reload sequence complete.");
  }

  pub fn host(&self) -> &Arc<Mutex<TestPluginHost>> {
    &self.host
  }

  pub fn audio_thread_options_service(&self) -> &HostOptionsService {
    &self.audio_thread_options_service
  }

  pub fn audio_io_service(&mut self) -> &mut AudioIOService {
    &mut self.audio_io_service
  }

  pub fn volume_publisher_service(&mut self) -> &mut VolumePublisherService<TestPluginHost> {
    &mut self.volume_publisher_service
  }
}
