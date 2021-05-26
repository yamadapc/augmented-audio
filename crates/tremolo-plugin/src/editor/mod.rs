mod webview;

use cocoa::appkit::{NSView, NSWindow, NSWindowStyleMask};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use darwin_webkit::helpers::dwk_webview::{string_from_nsstring, DarwinWKWebView};
use darwin_webkit::webkit::wk_script_message_handler::{make_new_handler, WKScriptMessage};
use darwin_webkit::webkit::WKUserContentController;
use editor::webview::WebviewHolder;
use log::info;
use objc::runtime::{Object, BOOL, YES};
use plugin_parameter::ParameterStore;
use std::ffi::c_void;
use std::sync::Arc;
use std::time::Instant;
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
        unsafe { self.initialize_webview(parent).unwrap_or(false) }
    }

    fn is_open(&mut self) -> bool {
        self.webview.is_some()
    }
}
