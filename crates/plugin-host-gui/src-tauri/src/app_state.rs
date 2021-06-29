use std::sync::{Arc, Mutex};

use plugin_host_lib::audio_io::test_plugin_host::TestPluginHost;
use plugin_host_lib::audio_io::AudioIOService;

pub type AppStateRef = Arc<Mutex<AppState>>;

pub struct AppState {
  host: Arc<Mutex<TestPluginHost>>,
  audio_io_service: AudioIOService,
}

impl AppState {
  pub fn new(host: plugin_host_lib::TestPluginHost) -> Self {
    let host = Arc::new(Mutex::new(host));
    AppState {
      host: host.clone(),
      audio_io_service: AudioIOService::new(host),
    }
  }

  pub fn host(&self) -> &Arc<Mutex<TestPluginHost>> {
    &self.host
  }

  pub fn audio_io_service(&mut self) -> &mut AudioIOService {
    &mut self.audio_io_service
  }
}
