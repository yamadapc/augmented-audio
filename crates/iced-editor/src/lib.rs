#[macro_use]
extern crate objc;

use std::ffi::c_void;

use baseview::{Size, WindowOpenOptions, WindowScalePolicy};
use vst::editor::Editor;

use iced_baseview::{Application, IcedWindow, Settings, WindowHandle};
use plugin_window::PluginWindow;

#[cfg(target_os = "macos")]
mod macos;
mod plugin_window;

pub struct IcedEditor<App>
where
    App: Application,
{
    flags: App::Flags,
    handle: Option<WindowHandle<App::Message>>,
    size: (i32, i32),
    position: (i32, i32),
}

impl<App> IcedEditor<App>
where
    App: Application,
{
    pub fn new(flags: App::Flags) -> Self {
        IcedEditor {
            flags,
            handle: None,
            size: (500, 500),
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
        let window_open_options = WindowOpenOptions {
            title: "Iced Editor".to_string(),
            size: Size {
                width: 500.0,
                height: 500.0,
            },
            scale: WindowScalePolicy::SystemScaleFactor,
        };
        let settings = Settings {
            window: window_open_options,
            flags: self.flags.clone(),
        };
        let mut handle = IcedWindow::<App>::open_parented(&window_handle, settings);
        self.handle = Some(handle);
        true
    }

    fn is_open(&mut self) -> bool {
        self.handle.is_some()
    }
}
