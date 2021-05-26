use cocoa::appkit::{NSView, NSWindow, NSWindowStyleMask};
use cocoa::base::{id, nil, YES};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use darwin_webkit::helpers::dwk_webview::{string_from_nsstring, DarwinWKWebView};
use darwin_webkit::webkit::wk_script_message_handler::{make_new_handler, WKScriptMessage};
use darwin_webkit::webkit::WKUserContentController;
use log::info;
use objc::runtime::BOOL;
use serde::Serialize;
use std::error::Error;
use std::ffi::c_void;
use std::time::Instant;

pub struct WebviewHolder {
    webview: DarwinWKWebView,
    on_message_callback: Option<fn(msg: String)>,
}

impl WebviewHolder {
    pub unsafe fn new(size: (i32, i32)) -> WebviewHolder {
        let origin = NSPoint::new(0.0, 0.0);
        let (width, height) = size;
        let size = NSSize::new(width as f64, height as f64);
        let frame = NSRect::new(origin, size);
        let webview = DarwinWKWebView::new(frame);

        WebviewHolder {
            webview,
            on_message_callback: None,
        }
    }

    pub unsafe fn initialize(&mut self, parent: *mut c_void) {
        self.attach_to_parent(parent);

        // TODO - this should be read from somewhere
        self.webview.load_url("http://127.0.0.1:3000");

        self.attach_message_handler();
    }

    pub unsafe fn attach_to_parent(&mut self, parent: *mut c_void) {
        let parent_id = parent as id;
        parent_id.addSubview_(self.webview.get_native_handle());
        let window_id: id = msg_send![parent_id, window];
        window_id.setStyleMask_(
            NSWindowStyleMask::NSTitledWindowMask
                | NSWindowStyleMask::NSResizableWindowMask
                | NSWindowStyleMask::NSClosableWindowMask,
        );

        let webview_id = self.webview.get_native_handle();
        pin_to_parent(parent_id, webview_id);

        let ns_size = msg_send![webview_id, frame];
        window_id.setMinSize_(ns_size);
    }

    unsafe fn attach_message_handler(&mut self) {
        let self_ptr: *mut Self = self;
        let on_message_ptr = Box::into_raw(Box::new(move |_, wk_script_message| {
            (*self_ptr).on_message(wk_script_message);
        }));
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
        self.webview
            .get_user_content_controller_handle()
            .addScriptMessageHandler(handler, name);
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

            self.on_message_callback.map(|cb| {
                cb(str.as_ref().unwrap().to_string());
            });
        }
    }
}

impl WebviewHolder {
    pub fn set_on_message_callback(&mut self, on_message_callback: fn(String)) {
        self.on_message_callback = Some(on_message_callback);
    }

    pub fn clear_on_message_callback(&mut self) {
        self.on_message_callback = None;
    }

    pub fn send_message<Msg>(&self, message: &Msg) -> Result<(), Box<dyn Error>>
    where
        Msg: Serialize,
    {
        let message = serde_json::to_string(message)?;
        let javascript_string = format!("window.__onMessage({})", message);

        unsafe {
            let msg = NSString::alloc(nil).init_str(javascript_string.as_str());
            let _: () = msg_send![self.webview.get_native_handle(), evaluateJavaScript: msg];
        }

        Ok(())
    }
}

/// Pin one NSView to a parent NSView so it'll resize to fit it
unsafe fn pin_to_parent(parent_id: id, webview_id: id) {
    // let _: () = msg_send![webview_id, setTranslatesAutoresizingMaskIntoConstraints: NO];
    let anchors = vec![
        sel!(leftAnchor),
        sel!(rightAnchor),
        sel!(topAnchor),
        sel!(bottomAnchor),
    ];

    for anchor in anchors {
        let parent_anchor = objc::__send_message(parent_id, anchor, ()).unwrap();
        let target_anchor = objc::__send_message(webview_id, anchor, ()).unwrap();
        pin_anchors(parent_anchor, target_anchor);
    }
}

/// Pins two NSAnchors
unsafe fn pin_anchors(parent: id, target: id) {
    let constraint: id = msg_send![parent, constraintEqualToAnchor: target];
    let _: () = msg_send![constraint, setActive: YES];
}
