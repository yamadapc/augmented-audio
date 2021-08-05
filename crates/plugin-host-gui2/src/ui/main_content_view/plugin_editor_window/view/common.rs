use raw_window_handle::RawWindowHandle;
use vst::editor::Editor;

/// Holds the plugin editor and its window
pub struct PluginWindowHandle {
    pub raw_window_handle: RawWindowHandle,
}

impl PluginWindowHandle {
    pub fn float(&mut self) {
        super::float_window(&self.raw_window_handle);
    }
}
