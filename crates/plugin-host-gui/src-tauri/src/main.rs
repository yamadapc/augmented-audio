#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Menu, MenuItem};

use app_state::AppState;
use commands::*;

mod app_state;
mod commands;
mod volume_publisher;

fn main() {
  wisual_logger::init_from_env();
  let mut plugin_host = plugin_host_lib::TestPluginHost::default();
  if let Err(err) = plugin_host.start() {
    log::error!("Failed to start host: {}", err);
  }
  let menus = vec![Menu::new(
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
  )];

  tauri::Builder::default()
    .manage(AppState::new(plugin_host))
    .invoke_handler(tauri::generate_handler![
      set_audio_driver_command,
      set_input_device_command,
      set_output_device_command,
      set_input_file_command,
      set_plugin_path_command,
      list_devices_command,
      list_hosts_command,
      subscribe_to_volume_command,
      unsubscribe_to_volume_command,
      play_command,
      pause_command,
      stop_command,
    ])
    .menu(menus)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
