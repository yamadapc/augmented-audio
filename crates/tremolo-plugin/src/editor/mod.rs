mod webview;

use editor::webview::WebviewHolder;
use plugin_parameter::ParameterStore;
use std::ffi::c_void;
use std::sync::Arc;
use vst::editor::Editor;

pub struct TremoloEditor {
    parameters: Arc<ParameterStore>,
    webview: Option<WebviewHolder>,
}

impl TremoloEditor {
    pub fn new(parameters: Arc<ParameterStore>) -> Self {
        TremoloEditor {
            parameters,
            webview: None,
        }
    }

    unsafe fn initialize_webview(&mut self, parent: *mut c_void) -> Option<bool> {
        // If there's already a webview just re-attach
        if let Some(webview) = &mut self.webview {
            webview.attach_to_parent(parent);
            return Some(true);
        }

        let mut webview = WebviewHolder::new(self.size());
        webview.initialize(parent);
        self.webview = Some(webview);

        Some(true)
    }
}

impl Editor for TremoloEditor {
    fn size(&self) -> (i32, i32) {
        (500, 500)
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn open(&mut self, parent: *mut c_void) -> bool {
        log::info!("Editor::open");
        unsafe { self.initialize_webview(parent).unwrap_or(false) }
    }

    fn close(&mut self) {
        log::info!("Editor::close");
    }

    fn is_open(&mut self) -> bool {
        self.webview.is_some()
    }
}
