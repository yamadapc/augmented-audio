#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use cpal::traits::{DeviceTrait, HostTrait};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::Window;
use thiserror::Error;

struct AudioOptions {
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

struct AppState {
  audio_options: Arc<Mutex<AudioOptions>>,
}

impl Default for AppState {
  fn default() -> Self {
    AppState {
      audio_options: Arc::new(Mutex::new(AudioOptions::default())),
    }
  }
}

#[derive(Serialize)]
struct AudioDevice {
  name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DevicesList {
  input_devices: Vec<AudioDevice>,
  output_devices: Vec<AudioDevice>,
}

#[derive(Error, Debug, Serialize)]
enum DevicesListError {
  #[error("Failed to get host reference")]
  HostUnavailableError,
  #[error("Failed to get host devices list")]
  DevicesError,
  #[error("Failed to get device name")]
  DeviceNameError,
}

fn get_input_devices_list(host_id: Option<String>) -> Result<Vec<AudioDevice>, DevicesListError> {
  let host_id = host_id
    .map(|host_id| {
      cpal::available_hosts()
        .into_iter()
        .find(|host| host.name() == host_id)
    })
    .flatten()
    .unwrap_or_else(|| cpal::default_host().id());
  let host = cpal::host_from_id(host_id).map_err(|_| DevicesListError::HostUnavailableError)?;
  let devices = host
    .input_devices()
    .map_err(|_| DevicesListError::DevicesError)?;
  let devices_vec = devices
    .map(|device| {
      Ok(AudioDevice {
        name: device.name()?.to_string(),
      })
    })
    .collect::<Result<Vec<AudioDevice>, Box<dyn std::error::Error>>>()
    .map_err(|_| DevicesListError::DeviceNameError)?;

  Ok(devices_vec)
}

fn get_output_devices_list(host_id: Option<String>) -> Result<Vec<AudioDevice>, DevicesListError> {
  let host_id = host_id
    .map(|host_id| {
      cpal::available_hosts()
        .into_iter()
        .find(|host| host.name() == host_id)
    })
    .flatten()
    .unwrap_or_else(|| cpal::default_host().id());
  let host = cpal::host_from_id(host_id).map_err(|_| DevicesListError::HostUnavailableError)?;
  let devices = host
    .output_devices()
    .map_err(|_| DevicesListError::DevicesError)?;
  let devices_vec = devices
    .map(|device| {
      Ok(AudioDevice {
        name: device.name()?.to_string(),
      })
    })
    .collect::<Result<Vec<AudioDevice>, Box<dyn std::error::Error>>>()
    .map_err(|_| DevicesListError::DeviceNameError)?;

  Ok(devices_vec)
}

#[tauri::command]
fn list_devices_command(host_id: Option<String>) -> Result<DevicesList, DevicesListError> {
  log::info!("Listing devices");
  let input_devices = get_input_devices_list(host_id.clone())?;
  let output_devices = get_output_devices_list(host_id)?;
  Ok(DevicesList {
    input_devices,
    output_devices,
  })
}

#[tauri::command]
fn list_hosts_command() -> Vec<String> {
  log::info!("Listing hosts");
  let hosts = cpal::available_hosts();
  hosts
    .into_iter()
    .map(|host| host.name().to_string())
    .collect()
}

#[tauri::command]
fn subscribe_to_volume_command(window: Window) {
  log::info!("Setting-up fake volume event emitter");
  std::thread::spawn(move || loop {
    let random_f: f32 = rand::random();
    let random_f2: f32 = rand::random();
    let js_string = format!("window.volume1={};window.volume2={};", random_f, random_f2);
    window.eval(&js_string);
    std::thread::sleep(std::time::Duration::from_millis(50));
  });
  log::info!("Volume event loop will emit volume every 100ms");
}

#[tauri::command]
fn unsubscribe_to_volume_command(window: Window) {
  log::info!("Cleaning-up emitter");
}

#[tauri::command]
fn set_audio_driver_command(state: tauri::State<AppState>, host_id: String) {
  log::info!("Setting audio driver {}", host_id);
  state.audio_options.lock().unwrap().host_id = host_id;
}

#[tauri::command]
fn set_input_device_command(state: tauri::State<AppState>, input_device_id: String) {
  log::info!("Setting input device {}", input_device_id);
  state.audio_options.lock().unwrap().input_device_id = Some(input_device_id);
}

#[tauri::command]
fn set_output_device_command(state: tauri::State<AppState>, output_device_id: String) {
  log::info!("Setting output device {}", output_device_id);
  state.audio_options.lock().unwrap().output_device_id = Some(output_device_id);
}

fn main() {
  wisual_logger::init_from_env();

  tauri::Builder::default()
    .manage(AppState::default())
    .invoke_handler(tauri::generate_handler![
      set_audio_driver_command,
      set_input_device_command,
      set_output_device_command,
      list_devices_command,
      list_hosts_command,
      subscribe_to_volume_command,
      unsubscribe_to_volume_command,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
