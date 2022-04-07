// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::ffi::c_void;

#[cfg(target_os = "macos")]
use crate::macos::get_raw_window_handle;

#[cfg(not(target_os = "macos"))]
fn get_raw_window_handle(_parent: *mut c_void) -> RawWindowHandle {
    todo!("Unsupported OS")
}

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
