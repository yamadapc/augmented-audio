#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use cpal::traits::{DeviceTrait, HostTrait};
use serde::Serialize;
use tauri::{Menu, MenuItem, Window};
use thiserror::Error;

use app_state::AppState;
use plugin_host_lib::audio_io::{AudioIOService, AudioIOServiceResult, DevicesList};

mod app_state;
mod volume_publisher;

#[tauri::command]
fn list_devices_command(host_id: Option<String>) -> AudioIOServiceResult<DevicesList> {
  log::info!("Listing devices");
  AudioIOService::devices_list(host_id)
}

#[tauri::command]
fn list_hosts_command() -> Vec<String> {
  log::info!("Listing hosts");
  AudioIOService::hosts()
}

#[tauri::command]
fn subscribe_to_volume_command(window: Window) {
  log::info!("Setting-up fake volume event emitter");
  std::thread::spawn(move || loop {
    let random_f: f32 = rand::random();
    let random_f2: f32 = rand::random();
    let js_string = format!("window.volume1={};window.volume2={};", random_f, random_f2);
    // TODO fix this
    let _ = window.eval(&js_string);
    std::thread::sleep(std::time::Duration::from_millis(50));
  });
  log::info!("Volume event loop will emit volume every 100ms");
}

#[tauri::command]
fn unsubscribe_to_volume_command(_window: Window) {
  // TODO implement unsubscribe
  log::info!("Cleaning-up emitter");
}

#[tauri::command]
fn set_audio_driver_command(state: tauri::State<AppState>, host_id: String) {
  log::info!("Setting audio driver {}", host_id);
  state.set_host_id(host_id);
}

#[tauri::command]
fn set_input_device_command(state: tauri::State<AppState>, input_device_id: String) {
  log::info!("Setting input device {}", input_device_id);
  state.set_input_device_id(input_device_id);
}

#[tauri::command]
fn set_output_device_command(state: tauri::State<AppState>, output_device_id: String) {
  log::info!("Setting output device {}", output_device_id);
  state.set_output_device_id(output_device_id);
}

fn main() {
  wisual_logger::init_from_env();
  // TODO - next step
  // let plugin_host = plugin_host_lib::PluginHost::new();
  let mut menus = Vec::new();
  menus.push(Menu::new(
    "plugin-host",
    vec![
      MenuItem::About(String::from("plugin-host")),
      MenuItem::Separator,
      MenuItem::Hide,
      MenuItem::HideOthers,
      MenuItem::ShowAll,
      MenuItem::Separator,
      MenuItem::Quit,
    ],
  ));

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
    .menu(menus)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
