#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};

use tauri::api::path::home_dir;
use tauri::{GlobalWindowEvent, Menu, MenuItem, WindowEvent};

use app_state::AppState;
use commands::*;

use crate::config::AppConfig;

mod app_state;
mod commands;
mod config;
mod services;

fn main() {
  wisual_logger::init_from_env();
  let mut plugin_host = plugin_host_lib::TestPluginHost::default();
  if let Err(err) = plugin_host.start() {
    log::error!("Failed to start host: {}", err);
  }
  // TODO - Menus not working anymore after tauri bump
  let main_menus = vec![
    MenuItem::About(String::from("plugin-host")),
    MenuItem::Separator,
    MenuItem::Hide,
    MenuItem::HideOthers,
    MenuItem::ShowAll,
    MenuItem::Separator,
    MenuItem::Quit,
  ];
  let mut main_menu = Menu::new();
  for item in main_menus {
    main_menu = main_menu.add_native_item(item);
  }

  let home_dir = home_dir().expect("Failed to get user HOME directory. App will fail to work.");
  let home_config_dir = home_dir.join(".plugin-host-gui");
  std::fs::create_dir_all(&home_config_dir).expect("Failed to create configuration directory.");

  let app_config = AppConfig {
    audio_thread_config_path: String::from(
      home_config_dir
        .join("audio-thread-config.json")
        .to_str()
        .unwrap(),
    ),
    storage_config: plugin_host_lib::audio_io::storage::StorageConfig {
      audio_io_state_storage_path: String::from(
        home_config_dir
          .join("audio-io-state.json")
          .to_str()
          .unwrap(),
      ),
    },
  };
  let mut app_state = AppState::new(plugin_host, app_config);
  app_state.try_reload();
  let app_state = Arc::new(Mutex::new(app_state));

  tauri::Builder::default()
    .manage(app_state.clone())
    .on_window_event({
      let state = app_state;
      move |window_event| on_window_event(&state, window_event)
    })
    .on_page_load(|window, _page_load| {
      let _ = window.emit("status_bar_change", "Loaded");
    })
    .invoke_handler(tauri::generate_handler![
      set_audio_driver_command,
      set_input_device_command,
      set_output_device_command,
      set_input_file_command,
      set_plugin_path_command,
      list_devices_command,
      list_hosts_command,
      get_host_state_command,
      subscribe_to_volume_command,
      unsubscribe_to_volume_command,
      play_command,
      pause_command,
      stop_command,
      log_command,
    ])
    .menu(main_menu)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn on_window_event(state: &Arc<Mutex<AppState>>, window_event: GlobalWindowEvent) {
  match window_event.event() {
    WindowEvent::CloseRequested => {
      let mut state = state.lock().unwrap();
      let volume_publisher_service = state.volume_publisher_service();
      volume_publisher_service.stop();
    }
    WindowEvent::Resized(_) => {}
    WindowEvent::Moved(_) => {}
    WindowEvent::Destroyed => {}
    WindowEvent::Focused(_) => {}
    WindowEvent::ScaleFactorChanged { .. } => {}
    _ => {}
  }
}
