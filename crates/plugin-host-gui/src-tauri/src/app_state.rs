use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait};

use plugin_host_lib::audio_io::audio_thread::options::AudioDeviceId;
use plugin_host_lib::audio_io::test_plugin_host::TestPluginHost;

pub struct AudioOptions {
  host_id: String,
  input_device_id: Option<String>,
  output_device_id: Option<String>,
}

impl Default for AudioOptions {
  fn default() -> Self {
    let host = cpal::default_host();
    let input_device = host.default_input_device();
    let output_device = host.default_output_device();

    let host_id = host.id().name().to_string();
    let input_device_id = input_device.map(|d| d.name().ok()).flatten();
    let output_device_id = output_device.map(|d| d.name().ok()).flatten();

    AudioOptions {
      host_id,
      input_device_id,
      output_device_id,
    }
  }
}

pub struct AppState {
  host: Arc<Mutex<plugin_host_lib::TestPluginHost>>,
  audio_options: Arc<Mutex<AudioOptions>>,
}

impl AppState {
  pub fn new(host: plugin_host_lib::TestPluginHost) -> Self {
    AppState {
      host: Arc::new(Mutex::new(host)),
      audio_options: Arc::new(Mutex::new(AudioOptions::default())),
    }
  }

  pub fn host(&self) -> Arc<Mutex<TestPluginHost>> {
    self.host.clone()
  }
}

impl AppState {
  pub fn set_host_id(&self, host_id: String) {
    self.audio_options.lock().unwrap().host_id = host_id;
  }

  pub fn set_input_device_id(&self, input_device_id: String) {
    self.audio_options.lock().unwrap().input_device_id = Some(input_device_id);
  }

  pub fn set_output_device_id(&self, output_device_id: String) {
    self.audio_options.lock().unwrap().output_device_id = Some(output_device_id.clone());
    if let Ok(mut host) = self.host.lock() {
      let result = host.set_output_device_id(AudioDeviceId::Id(output_device_id));
      match result {
        Ok(_) => log::info!("Output device set"),
        Err(err) => log::error!("Failure to set output device: {}", err),
      }
    }
  }

  pub fn set_input_file(&self, input_file: String) {
    if let Ok(mut host) = self.host.lock() {
      let result = host.set_audio_file_path(PathBuf::from(input_file));
      match result {
        Ok(_) => log::info!("Input file set"),
        Err(err) => log::error!("Failure to set input: {}", err),
      }
    }
  }

  pub fn set_plugin_path(&self, path: String) {
    if let Ok(mut host) = self.host.lock() {
      let path = Path::new(&path);
      let result = host.load_plugin(path);
      match result {
        Ok(_) => log::info!("Plugin loaded"),
        Err(err) => log::error!("Failure to load plugin: {}", err),
      }
    }
  }
}
