use iced::Command;

pub use view::*;

use crate::services::host_options_service::HostState;

pub mod view;

pub struct Controller {
    view: view::View,
}

impl Controller {
    pub fn new(host_state: &HostState) -> Self {
        Self {
            view: view::View::new(
                host_state.audio_input_file_path.clone(),
                host_state.plugin_path.clone(),
            ),
        }
    }

    // pub fn update(&mut self, message: Message) -> Command<Message> {
    //     let command = match message {
    //         Message::SetInputFile(input_file) => self.set_input_file(input_file),
    //         // Message::OpenPluginWindow => self.open_plugin_window(),
    //         // Message::SetAudioPlugin(path) => self.set_audio_plugin_path(path),
    //         // Message::ReloadPlugin => self.reload_plugin(),
    //         _ => Command::none(),
    //     };
    //     let children = self.view.update(msg);
    //     Command::batch(vec![command, children])
    // }
    //
    // fn set_input_file(&mut self, input_file: &String) -> Command<Message> {
    //     let result = self
    //         .plugin_host
    //         .lock()
    //         .unwrap()
    //         .set_audio_file_path(PathBuf::from(input_file));
    //
    //     self.reset_handles();
    //     result.unwrap_or_else(|err| self.error = Some(Box::new(err)));
    //     self.host_state.audio_input_file_path = Some(input_file.clone());
    //     self.host_options_service
    //         .store(&self.host_state)
    //         .unwrap_or_else(|err| {
    //             log::error!("Failed to store {:?}", err);
    //         });
    //     Command::none()
    // }
    //
    // fn set_audio_plugin_path(&mut self, path: &String) -> Command<Message> {
    //     self.reset_handles();
    //     if let ClosePluginWindowResult::ClosedPlugin { window_frame } = self.close_plugin_window() {
    //         self.previous_plugin_window_frame = Some(window_frame);
    //     }
    //     let path = path.clone();
    //
    //     self.host_state.plugin_path = Some(path.clone());
    //     self.host_options_service
    //         .store(&self.host_state)
    //         .unwrap_or_else(|err| {
    //             log::error!("Failed to store {:?}", err);
    //         });
    //
    //     let host = self.plugin_host.clone();
    //     Command::perform(
    //         tokio::task::spawn_blocking(move || {
    //             let path = Path::new(&path);
    //             host.lock().unwrap().load_plugin(path)
    //         }),
    //         |result| match result {
    //             Err(err) => Message::SetStatus(StatusBar::new(
    //                 format!("Error loading plugin: {}", err),
    //                 status_bar::State::Error,
    //             )),
    //             Ok(_) => {
    //                 Message::SetStatus(StatusBar::new("Loaded plugin", status_bar::State::Idle))
    //             }
    //         },
    //     )
    // }
}
