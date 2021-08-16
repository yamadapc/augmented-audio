use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::ffi::c_void;

#[cfg(target_os = "macos")]
use crate::macos::get_raw_window_handle;

#[cfg(not(target_os = "macos"))]
fn get_raw_window_handle(_parent: *mut c_void) -> RawWindowHandle {
    todo!("Unsupported OS")
}

pub struct WindowHolder {
    window_handle: RawWindowHandle,
}

impl WindowHolder {
    pub fn new(parent: *mut c_void) -> Self {
        WindowHolder {
            window_handle: get_raw_window_handle(parent),
        }
    }
}

unsafe impl HasRawWindowHandle for WindowHolder {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.window_handle
    }
}
