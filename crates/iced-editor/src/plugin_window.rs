use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::ffi::c_void;

#[cfg(target_os = "macos")]
use crate::macos::get_raw_window_handle;

pub struct PluginWindow {
    window_handle: RawWindowHandle,
}

impl PluginWindow {
    pub fn new(parent: *mut c_void) -> Self {
        PluginWindow {
            window_handle: get_raw_window_handle(parent),
        }
    }
}

unsafe impl HasRawWindowHandle for PluginWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.window_handle
    }
}
