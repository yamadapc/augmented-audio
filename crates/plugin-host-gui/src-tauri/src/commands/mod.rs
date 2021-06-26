use tauri::Window;

use plugin_host_lib::audio_io::{AudioIOService, AudioIOServiceResult, DevicesList};

use crate::app_state::AppState;

#[tauri::command]
pub fn list_devices_command(host_id: Option<String>) -> AudioIOServiceResult<DevicesList> {
  log::info!("Listing devices");
  AudioIOService::devices_list(host_id)
}

#[tauri::command]
pub fn list_hosts_command() -> Vec<String> {
  log::info!("Listing hosts");
  AudioIOService::hosts()
}

#[tauri::command]
pub fn subscribe_to_volume_command(state: tauri::State<AppState>, window: Window) -> String {
  log::info!("Setting-up fake volume event emitter");
  let host = state.inner().host();
  std::thread::spawn(move || loop {
    let (volume_left, volume_right) = host.lock().unwrap().current_volume();
    let js_string = format!(
      "window.volume1={};window.volume2={};",
      volume_left, volume_right
    );
    // TODO fix this
    let _ = window.eval(&js_string);
    std::thread::sleep(std::time::Duration::from_millis(50));
  });
  log::info!("Volume event loop will emit volume every 100ms");
  String::from("")
}

#[tauri::command]
pub fn unsubscribe_to_volume_command(_window: Window) {
  // TODO implement unsubscribe
  log::info!("Cleaning-up emitter");
}

#[tauri::command]
pub fn set_audio_driver_command(state: tauri::State<AppState>, host_id: String) {
  log::info!("Setting audio driver {}", host_id);
  state.set_host_id(host_id);
}

#[tauri::command]
pub fn set_input_device_command(state: tauri::State<AppState>, input_device_id: String) {
  log::info!("Setting input device {}", input_device_id);
  state.set_input_device_id(input_device_id);
}

#[tauri::command]
pub fn set_output_device_command(state: tauri::State<AppState>, output_device_id: String) {
  log::info!("Setting output device {}", output_device_id);
  state.set_output_device_id(output_device_id);
}

#[tauri::command]
pub fn set_input_file_command(state: tauri::State<AppState>, input_file: String) {
  log::info!("Setting audio input file {}", input_file);
  state.set_input_file(input_file);
}

#[tauri::command]
pub fn set_plugin_path_command(state: tauri::State<AppState>, path: String) {
  log::info!("Setting plugin path {}", path);
  state.set_plugin_path(path);
}
