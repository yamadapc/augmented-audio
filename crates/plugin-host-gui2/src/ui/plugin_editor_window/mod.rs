//! Module for opening the plugin window.
mod common;
#[cfg(target_os = "macos")]
mod macos;

pub use common::PluginWindowHandle;
#[cfg(target_os = "macos")]
pub use macos::{close_window, open_plugin_window};
#[cfg(not(target_os = "macos"))]
use vst::editor::Editor;

#[cfg(not(target_os = "macos"))]
pub fn open_plugin_window(
    mut editor: Box<dyn Editor>,
    size: (i32, i32),
    position: Option<iced::Point>,
) -> PluginWindowHandle {
    todo!("Not implemented")
}

#[cfg(not(target_os = "macos"))]
pub fn close_window(raw_window_handle: raw_window_handle::RawWindowHandle) -> Option<Rectangle> {
    todo!("Not implemented")
}
