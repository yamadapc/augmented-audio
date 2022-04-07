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
//! Module for opening the plugin window.
mod common;
#[cfg(target_os = "macos")]
mod macos;

pub use common::PluginWindowHandle;
#[cfg(target_os = "macos")]
pub use macos::{close_window, float_window, open_plugin_window};

#[cfg(not(target_os = "macos"))]
use vst::editor::Editor;

#[cfg(not(target_os = "macos"))]
pub fn open_plugin_window(
    editor: &mut Box<dyn Editor>,
    size: (i32, i32),
    position: Option<iced::Point>,
) -> PluginWindowHandle {
    todo!("Not implemented")
}

#[cfg(not(target_os = "macos"))]
pub fn close_window(
    raw_window_handle: raw_window_handle::RawWindowHandle,
) -> Option<iced::Rectangle> {
    todo!("Not implemented")
}

#[cfg(not(target_os = "macos"))]
pub fn float_window(handle: &raw_window_handle::RawWindowHandle) {
    todo!("Not implemented")
}
