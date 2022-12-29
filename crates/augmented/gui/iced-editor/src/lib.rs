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
#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

use std::ffi::c_void;

use baseview::{Size, Window, WindowHandle, WindowOpenOptions, WindowScalePolicy};
use iced_baseview::{settings::IcedBaseviewSettings, Application, Settings};
use vst::editor::Editor;

use plugin_window::PluginWindow;

#[cfg(target_os = "macos")]
mod macos;
mod plugin_window;

pub struct IcedEditor<App>
where
    App: Application,
    App::Message: 'static,
{
    flags: App::Flags,
    handle: Option<WindowHandle>,
    size: (i32, i32),
    position: (i32, i32),
}

impl<App> IcedEditor<App>
where
    App: Application,
{
    pub fn new(flags: App::Flags) -> Self {
        Self::new_with(flags, (500, 300))
    }

    pub fn new_with(flags: App::Flags, size: (i32, i32)) -> Self {
        IcedEditor {
            flags,
            handle: None,
            size,
            position: (0, 0),
        }
    }
}

impl<App> Editor for IcedEditor<App>
where
    App: Application + Send,
    App::Flags: Clone,
{
    fn size(&self) -> (i32, i32) {
        self.size
    }

    fn position(&self) -> (i32, i32) {
        self.position
    }

    fn open(&mut self, parent: *mut c_void) -> bool {
        let window_handle = PluginWindow::new(parent);
        let settings = Settings {
            // window: window_open_options,
            window: WindowOpenOptions {
                title: "Iced editor".to_string(),
                size: Size {
                    width: self.size.0 as f64,
                    height: self.size.1 as f64,
                },
                scale: WindowScalePolicy::SystemScaleFactor,
                #[cfg(feature = "glow")]
                gl_config: None,
            },
            flags: self.flags.clone(),
            iced_baseview: IcedBaseviewSettings {
                ignore_non_modifier_keys: false,
                always_redraw: true,
            },
        };
        let handle = Window::open_parented(&window_handle, settings);
        self.handle = Some(handle);
        true
    }

    fn is_open(&mut self) -> bool {
        self.handle.is_some()
    }
}
