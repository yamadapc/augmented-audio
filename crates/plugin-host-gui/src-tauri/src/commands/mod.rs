use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tauri::Window;

use plugin_host_lib::audio_io::{
  AudioIOService, AudioIOServiceError, AudioIOServiceResult, DevicesList,
};

use crate::app_state::AppStateRef;
use crate::volume_publisher;

#[derive(Serialize, Deserialize)]
struct CommandError {
  message: String,
}

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

#[tauri::command(async)]
pub fn subscribe_to_volume_command(state: tauri::State<AppStateRef>, window: Window) -> String {
  log::info!("Setting-up fake volume event emitter");
  let mut state = state.inner().lock().unwrap();
  let volume_publisher_service = state.volume_publisher_service();
  volume_publisher_service.subscribe(move |volume| loop {
    let (volume_left, volume_right) = volume;
    let js_string = format!(
      "window.volume1={};window.volume2={};",
      volume_left, volume_right
    );
    let _ = window.eval(&js_string);
  })
}

#[tauri::command(async)]
pub fn unsubscribe_to_volume_command(
  state: tauri::State<AppStateRef>,
  subscriber_id: volume_publisher::ReceiverId,
) {
  let mut state = state.inner().lock().unwrap();
  let volume_publisher_service = state.volume_publisher_service();
  volume_publisher_service.unsubscribe(&subscriber_id)
}

#[tauri::command]
pub fn set_audio_driver_command(state: tauri::State<AppStateRef>, host_id: String) {
  log::info!("Setting audio driver {}", host_id);
  let mut state = state.lock().unwrap();
  let audio_io_service = state.audio_io_service();
  audio_io_service.set_host_id(host_id);
}

#[tauri::command]
pub fn set_input_device_command(state: tauri::State<AppStateRef>, input_device_id: String) {
  log::info!("Setting input device {}", input_device_id);
  let mut state = state.lock().unwrap();
  let audio_io_service = state.audio_io_service();
  audio_io_service.set_input_device_id(input_device_id);
}

#[tauri::command]
pub fn set_output_device_command(
  state: tauri::State<AppStateRef>,
  output_device_id: String,
) -> Result<(), AudioIOServiceError> {
  log::info!("Setting output device {}", output_device_id);
  let mut state = state.lock().unwrap();
  let audio_io_service = state.audio_io_service();
  audio_io_service.set_output_device_id(output_device_id)
}

#[tauri::command]
pub fn set_input_file_command(state: tauri::State<AppStateRef>, input_file: String) {
  log::info!("Setting audio input file {}", input_file);
  let state = state.lock().unwrap();
  let mut host = state.host().lock().unwrap();
  let result = host.set_audio_file_path(PathBuf::from(input_file));
  match result {
    Ok(_) => log::info!("Input file set"),
    Err(err) => log::error!("Failure to set input: {}", err),
  }
}

#[tauri::command]
pub fn set_plugin_path_command(state: tauri::State<AppStateRef>, path: String) {
  log::info!("Setting plugin path {}", path);
  let state = state.lock().unwrap();
  let mut host = state.host().lock().unwrap();
  let path = Path::new(&path);
  let result = host.load_plugin(path);
  match result {
    Ok(_) => log::info!("Plugin loaded"),
    Err(err) => log::error!("Failure to load plugin: {}", err),
  }
}

#[tauri::command]
pub fn play_command(state: tauri::State<AppStateRef>) {
  let state = state.lock().unwrap();
  let host = state.host().lock().unwrap();
  host.play();
}

#[tauri::command]
pub fn pause_command(state: tauri::State<AppStateRef>) {
  let state = state.lock().unwrap();
  let host = state.host().lock().unwrap();
  host.pause();
}

#[tauri::command]
pub fn stop_command(state: tauri::State<AppStateRef>) {
  let state = state.lock().unwrap();
  let host = state.host().lock().unwrap();
  host.stop();
}
