use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait};

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
  fn new(host: plugin_host_lib::TestPluginHost) -> Self {
    AppState {
      host: Arc::new(Mutex::new(host)),
      audio_options: Arc::new(Mutex::new(AudioOptions::default())),
    }
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
    self.audio_options.lock().unwrap().output_device_id = Some(output_device_id);
  }
}
