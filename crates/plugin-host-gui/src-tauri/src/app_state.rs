use std::sync::{Arc, Mutex};

use crate::volume_publisher::VolumePublisherService;
use plugin_host_lib::audio_io::test_plugin_host::TestPluginHost;
use plugin_host_lib::audio_io::AudioIOService;

pub type AppStateRef = Arc<Mutex<AppState>>;

pub struct AppState {
  host: Arc<Mutex<TestPluginHost>>,
  audio_io_service: AudioIOService,
  volume_publisher_service: VolumePublisherService<TestPluginHost>,
}

impl AppState {
  pub fn new(host: plugin_host_lib::TestPluginHost) -> Self {
    let host = Arc::new(Mutex::new(host));
    AppState {
      host: host.clone(),
      audio_io_service: AudioIOService::new(host.clone()),
      volume_publisher_service: VolumePublisherService::new(host),
    }
  }

  pub fn host(&self) -> &Arc<Mutex<TestPluginHost>> {
    &self.host
  }

  pub fn audio_io_service(&mut self) -> &mut AudioIOService {
    &mut self.audio_io_service
  }

  pub fn volume_publisher_service(&mut self) -> &mut VolumePublisherService<TestPluginHost> {
    &mut self.volume_publisher_service
  }
}
