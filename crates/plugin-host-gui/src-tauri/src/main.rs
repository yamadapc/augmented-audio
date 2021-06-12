#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use cpal::traits::{DeviceTrait, HostTrait};
use serde::Serialize;
use std::collections::HashMap;
use tauri::Window;
use thiserror::Error;

struct AppState {}

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
    input_devices: input_devices,
    output_devices: output_devices,
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
    // if let Err(err) = window.emit("volume", vec![random_f, random_f2]) {
    //   log::error!("Failed to emit {:?}", err);
    // }
    std::thread::sleep(std::time::Duration::from_millis(50));
  });
  log::info!("Volume event loop will emit volume every 100ms");
}

#[tauri::command]
fn unsubscribe_to_volume_command(window: Window) {
  log::info!("Cleaning-up emitter");
}

fn main() {
  wisual_logger::init_from_env();

  tauri::Builder::default()
    .manage(AppState {})
    .invoke_handler(tauri::generate_handler![
      list_devices_command,
      list_hosts_command,
      subscribe_to_volume_command,
      unsubscribe_to_volume_command,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
