use std::ops::Deref;
use std::sync::{Arc, Mutex};

use iced::Rectangle;
use vst::editor::Editor;
use vst::host::PluginInstance;
use vst::plugin::Plugin;

use plugin_host_lib::TestPluginHost;

mod view;

pub enum ClosePluginWindowResult {
    NoWindow,
    ClosedPlugin { window_frame: Rectangle },
}

pub struct EditorController {
    plugin_host: Arc<Mutex<TestPluginHost>>,
    editor: Option<Box<dyn Editor>>,
    window_handle: Option<view::PluginWindowHandle>,
    /// Whether the editor window is above all others
    editor_is_floating: bool,
    /// Cached window frame from a previous editor open
    previous_plugin_window_frame: Option<Rectangle>,
}

impl EditorController {
    pub fn new(plugin_host: Arc<Mutex<TestPluginHost>>) -> Self {
        EditorController {
            plugin_host,
            editor: None,
            window_handle: None,
            editor_is_floating: false,
            previous_plugin_window_frame: None,
        }
    }
}

impl EditorController {
    pub fn open_plugin_window(&mut self) {
        if self.window_handle.is_some() {
            log::warn!("Refusing to open 2 plugin editors");
            return;
        }

        log::info!("Opening plugin editor");
        let frame = self
            .previous_plugin_window_frame
            .map(|frame| frame.position());
        let editor = self.editor();

        if let Some(editor) = editor {
            log::info!("Found plugin editor");
            let size = editor.size();
            let window = view::open_plugin_window(editor, size, frame);
            log::info!("Opened editor window");
            self.window_handle = Some(window);
        } else {
            log::error!("No editor returned");
        }

        if self.editor_is_floating {
            let _ = self.float_plugin_window();
        }
    }

    pub fn close_plugin_window(&mut self) -> ClosePluginWindowResult {
        if let Some(window_handle) = self.window_handle.take() {
            log::info!("Closing plugin editor");
            if let Some(editor) = &mut self.editor {
                editor.close();
            }
            log::info!("Closing plugin window");
            let frame = view::close_window(window_handle.raw_window_handle);
            frame
                .map(|window_frame| ClosePluginWindowResult::ClosedPlugin { window_frame })
                .unwrap_or(ClosePluginWindowResult::NoWindow)
        } else {
            log::warn!("Close requested, but there's no plugin window handle");
            ClosePluginWindowResult::NoWindow
        }
    }

    pub fn float_plugin_window(&mut self) {
        self.editor_is_floating = true;
        if let Some(handle) = &mut self.window_handle {
            handle.float();
        }
    }

    pub fn on_reload(&mut self) -> bool {
        self.editor = None;
        if let ClosePluginWindowResult::ClosedPlugin { window_frame } = self.close_plugin_window() {
            self.previous_plugin_window_frame = Some(window_frame);
            true
        } else {
            false
        }
    }

    fn editor(&mut self) -> Option<&mut Box<dyn Editor>> {
        if self.editor.is_some() {
            return self.editor.as_mut();
        }

        let instance = self.plugin_host.lock().unwrap().plugin_instance();

        log::info!("Found plugin instance");
        let instance_ptr = instance?.deref() as *const PluginInstance as *mut PluginInstance;
        if let Some(editor) = unsafe { instance_ptr.as_mut() }.unwrap().get_editor() {
            self.editor = Some(editor);
            self.editor.as_mut()
        } else {
            None
        }
    }
}
