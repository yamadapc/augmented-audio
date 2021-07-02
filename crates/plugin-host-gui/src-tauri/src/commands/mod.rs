use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::MutexGuard;

use serde::{Deserialize, Serialize};
use tauri::Window;

use plugin_host_lib::audio_io::{
  AudioIOService, AudioIOServiceError, AudioIOServiceResult, DevicesList,
};
use plugin_host_lib::TestPluginHost;

use crate::app_state::{AppState, AppStateRef};
use crate::services::host_options_service::HostState;
use crate::services::volume_publisher;
use tauri::api::path::home_dir;

#[derive(Serialize, Deserialize)]
struct CommandError {
  message: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogMessage {
  level: String,
  message: String,
  time: String,
  logger: String,
  variables: HashMap<String, serde_json::Value>,
  context: HashMap<String, serde_json::Value>,
}

#[tauri::command]
pub fn log_command(message: LogMessage) {
  let level = match message.level.as_str() {
    "info" => log::Level::Info,
    "debug" => log::Level::Debug,
    "warn" => log::Level::Warn,
    "error" => log::Level::Error,
    _ => log::Level::Info,
  };
  let target = format!("frontend::{}", message.logger);
  log::log!(
    target: &target,
    level,
    "{} variables={:?} context={:?}",
    message.message,
    message.variables,
    message.context
  );
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

#[tauri::command]
pub fn get_host_state_command(state: tauri::State<AppStateRef>) -> HostState {
  let state = state.lock().unwrap();
  let host = state.host().lock().unwrap();
  get_host_state(&host)
}

#[tauri::command(async)]
pub fn subscribe_to_volume_command(state: tauri::State<AppStateRef>, window: Window) -> String {
  log::info!("Setting-up fake volume event emitter");
  let mut state = state.inner().lock().unwrap();
  let volume_publisher_service = state.volume_publisher_service();
  let window = window.clone();
  volume_publisher_service.subscribe(move |volume| {
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
  let result = host.set_audio_file_path(PathBuf::from(input_file.clone()));
  match result {
    Ok(_) => {
      log::info!("Input file set");
      save_host_options(&state, &host);
    }
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
    Ok(_) => {
      log::info!("Plugin loaded");
      save_host_options(&state, &host);
    }
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

// TODO: This is a mess, HOST should own its state & persistence?
fn save_host_options(state: &MutexGuard<AppState>, host: &MutexGuard<TestPluginHost>) {
  let host_state = get_host_state(host);
  let result = state.audio_thread_options_service().store(&host_state);
  match result {
    Ok(_) => {
      log::info!("Saved host options");
    }
    Err(err) => {
      log::error!("Failed saving host options: {}", err);
    }
  }
}

fn get_host_state(host: &MutexGuard<TestPluginHost>) -> HostState {
  let home_dir = home_dir().unwrap();
  let plugin_path = host
    .plugin_file_path()
    .clone()
    .map(|plugin_file_path| strip_home_dir(&home_dir, plugin_file_path));
  let audio_input_file_path = host.audio_file_path().clone();
  let audio_input_file_path = strip_home_dir(&home_dir, audio_input_file_path);
  let audio_input_file_path = Some(audio_input_file_path);

  let host_state = HostState {
    plugin_path,
    audio_input_file_path,
  };
  host_state
}

fn strip_home_dir(home_dir: &PathBuf, path: PathBuf) -> String {
  if let Ok(without_home) = path.strip_prefix(&home_dir) {
    format!("~/{}", without_home.to_str().unwrap().to_string())
  } else {
    path.to_str().unwrap().to_string()
  }
}
