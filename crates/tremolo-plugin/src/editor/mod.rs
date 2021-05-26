use cocoa::appkit::{NSView, NSWindow, NSWindowStyleMask};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use darwin_webkit::helpers::dwk_webview::{string_from_nsstring, DarwinWKWebView};
use darwin_webkit::webkit::wk_script_message_handler::{make_new_handler, WKScriptMessage};
use darwin_webkit::webkit::WKUserContentController;
use log::info;
use objc::runtime::{BOOL, YES};
use plugin_parameter::ParameterStore;
use std::ffi::c_void;
use std::sync::Arc;
use std::time::Instant;
use vst::editor::Editor;

pub struct TremoloEditor {
    parameters: Arc<ParameterStore>,
    webview: Option<DarwinWKWebView>,
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
        if let Some(webview) = &self.webview {
            let (width, height) = self.size();
            let ns_size = NSSize::new(width as f64, height as f64);
            TremoloEditor::attach_to_parent(parent, ns_size, &webview);
            return Some(true);
        }

        let (ns_size, webview) = self.create_webview();
        TremoloEditor::attach_to_parent(parent, ns_size, &webview);

        self.attach_message_handler();
        webview.load_url("http://127.0.0.1:3000");
        self.webview = Some(webview);

        Some(true)
    }

    unsafe fn attach_message_handler(&mut self) {
        let self_ptr: *mut Self = self;
        let on_message_ptr = Box::into_raw(Box::new(move |_, wk_script_message| {
            (*self_ptr).on_message(wk_script_message);
        }));
        let webview = self.webview.as_mut().unwrap();
        let name = "editor";

        // This creates a new objective-c class for the message handler
        let handler = make_new_handler(
            format!(
                "DWKHandler_{}__{}",
                name,
                Instant::now().elapsed().as_micros()
            )
            .as_str(),
            on_message_ptr,
        );

        let name = NSString::alloc(nil).init_str(name);
        webview
            .get_user_content_controller_handle()
            .addScriptMessageHandler(handler, name);
    }

    unsafe fn create_webview(&self) -> (NSSize, DarwinWKWebView) {
        let origin = NSPoint::new(0.0, 0.0);
        let (width, height) = self.size();
        let size = NSSize::new(width as f64, height as f64);
        let frame = NSRect::new(origin, size);
        let webview = darwin_webkit::helpers::dwk_webview::DarwinWKWebView::new(frame);
        (size, webview)
    }

    unsafe fn attach_to_parent(parent: *mut c_void, ns_size: NSSize, webview: &DarwinWKWebView) {
        let parent_id = parent as id;
        parent_id.addSubview_(webview.get_native_handle());
        let window_id: id = msg_send![parent_id, window];
        window_id.setStyleMask_(
            NSWindowStyleMask::NSTitledWindowMask
                | NSWindowStyleMask::NSResizableWindowMask
                | NSWindowStyleMask::NSClosableWindowMask,
        );
        window_id.setMinSize_(ns_size);
    }

    unsafe fn on_message(&mut self, wk_script_message: id) {
        // https://developer.apple.com/documentation/webkit/wkscriptmessage/1417901-body?language=objc
        // Allowed types are NSNumber, NSString, NSDate, NSArray, NSDictionary, and NSNull.
        let body = wk_script_message.body();

        // only support string for simplicity
        let string_class = class!(NSString);
        let is_string: BOOL = msg_send![body, isKindOfClass: string_class];
        if is_string == YES {
            let str = string_from_nsstring(body);
            info!("Got message from JavaScript {}", str.as_ref().unwrap());
            let webview = self.webview.as_ref().unwrap().get_native_handle();

            let msg = NSString::alloc(nil).init_str("window.audioRuntime.ok()");
            let _: () = msg_send![webview, evaluateJavaScript: msg];
        }
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

    fn close(&mut self) {
        // self.webview = None
    }

    fn is_open(&mut self) -> bool {
        self.webview.is_some()
    }
}
