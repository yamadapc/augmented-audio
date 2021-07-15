use raw_window_handle::RawWindowHandle;
use vst::editor::Editor;

/// Holds the plugin editor and its window
pub struct PluginWindowHandle {
    pub editor: Box<dyn Editor>,
    pub raw_window_handle: RawWindowHandle,
}
