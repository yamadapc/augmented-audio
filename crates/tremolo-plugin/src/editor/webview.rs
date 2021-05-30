use cocoa::appkit::{NSLayoutConstraint, NSView, NSWindow, NSWindowStyleMask};
use cocoa::base::{id, nil, NO, YES};
use cocoa::foundation::{NSArray, NSPoint, NSRect, NSSize, NSString};
use darwin_webkit::helpers::dwk_webview::{string_from_nsstring, DarwinWKWebView};
use darwin_webkit::webkit::wk_script_message_handler::WKScriptMessage;
use darwin_webkit::webkit::WKUserContentController;
use log::{error, info};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, BOOL};
use serde::Serialize;
use std::error::Error;
use std::ffi::c_void;

extern "C" fn call_ptr(this: &Object, _sel: Sel, controller: id, message: id) {
    unsafe {
        let instance_ptr: *mut c_void = *this.get_ivar("instance_ptr");
        let instance_ptr = std::mem::transmute::<
            *mut c_void,
            unsafe extern "C" fn(*mut c_void, id, id),
        >(instance_ptr);
        let data: &*mut c_void = this.get_ivar("internal_data");
        let data: *mut c_void = (*data) as *mut c_void;
        info!(
            "call_ptr - Received callback this={:?} data={:?} instance={:?}",
            this, data, instance_ptr as *mut c_void
        );
        instance_ptr(data, controller, message);
    }
}

unsafe extern "C" fn on_message_ptr(self_ptr: *mut c_void, _: id, wk_script_message: id) {
    let self_ptr = self_ptr as *mut WebviewHolder;
    info!("on_message_ptr - {}", (*self_ptr).id);
    (*self_ptr).on_message(wk_script_message);
}

pub unsafe fn make_new_handler<T>(
    name: &str,
    func: unsafe extern "C" fn(*mut T, id, id),
    data: *mut T,
) -> id {
    make_class_decl(name);

    let class = Class::get(name).unwrap();
    let instance: id = msg_send![class, alloc];
    let instance: id = msg_send![instance, init];
    info!(
        "make_new_handler - Creating class instance this={:?}",
        instance
    );
    {
        let instance = instance.as_mut().unwrap();
        instance.set_ivar("instance_ptr", func as *mut c_void);
        instance.set_ivar("internal_data", data as *mut c_void);
    }

    info!(
        "make_new_handler - Registering callback this={:?} data={:?} instance={:?}",
        instance, data as *mut c_void, func as *mut c_void
    );

    instance
}

static mut HAS_REGISTERED_HANDLER_CLASS: bool = false;

unsafe fn make_class_decl(name: &str) {
    if HAS_REGISTERED_HANDLER_CLASS {
        return;
    }

    info!("make_new_handler - Creating class decl name={}", name);
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new(name, superclass).unwrap();
    decl.add_ivar::<*const c_void>("instance_ptr");
    decl.add_ivar::<*const c_void>("internal_data");
    decl.add_method::<extern "C" fn(&Object, Sel, id, id)>(
        sel!(userContentController:didReceiveScriptMessage:),
        call_ptr,
    );
    decl.register();
    HAS_REGISTERED_HANDLER_CLASS = true;
}

pub struct WebviewHolder {
    webview: DarwinWKWebView,
    on_message_callback: Option<fn(msg: String)>,
    id: i32,
}

impl Drop for WebviewHolder {
    fn drop(&mut self) {
        info!("WebviewHolder::drop");
    }
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
            id: 1234,
        }
    }

    pub unsafe fn initialize(&mut self, parent: *mut c_void) {
        info!("WebviewHolder::initialize - Attaching to parent NSView");
        self.attach_to_parent(parent);

        info!("WebviewHolder::initialize - Setting-up message handler");
        self.attach_message_handler();

        assert_eq!(self.on_message_callback.is_some(), false);

        // TODO - this should be read from somewhere
        info!("WebviewHolder::initialize - Loading app URL");
        self.webview.load_url("http://127.0.0.1:3000");
    }

    pub unsafe fn attach_to_parent(&mut self, parent: *mut c_void) {
        let parent_id = parent as id;
        let wk_webview = self.webview.get_native_handle();
        parent_id.addSubview_(wk_webview);
        let window_id: id = msg_send![parent_id, window];
        window_id.setStyleMask_(
            NSWindowStyleMask::NSTitledWindowMask
                | NSWindowStyleMask::NSResizableWindowMask
                | NSWindowStyleMask::NSClosableWindowMask,
        );

        let webview_id = self.webview.get_native_handle();
        pin_to_parent(parent_id, webview_id);

        let ns_rect: NSRect = NSView::frame(webview_id);
        let ns_size = ns_rect.size;
        window_id.setMinSize_(ns_size);
    }

    unsafe fn attach_message_handler(&mut self) {
        info!(
            "WebviewHolder::attach_message_handler has_callback={} id={} ptr={:?}",
            self.on_message_callback.is_some(),
            self.id,
            self as *const WebviewHolder as *const c_void,
        );
        let name = "editor";

        // This creates a new objective-c class for the message handler
        let handler = make_new_handler(
            format!("DWKHandler_{}", name).as_str(),
            on_message_ptr,
            self as *mut WebviewHolder as *mut c_void,
        );

        let name = NSString::alloc(nil).init_str(name);
        self.webview
            .get_user_content_controller_handle()
            .addScriptMessageHandler(handler, name);
    }

    unsafe fn on_message(&mut self, wk_script_message: id) {
        let run = || -> Result<(), &str> {
            // https://developer.apple.com/documentation/webkit/wkscriptmessage/1417901-body?language=objc
            // Allowed types are NSNumber, NSString, NSDate, NSArray, NSDictionary, and NSNull.
            let body = wk_script_message.body();

            // only support string for simplicity
            let string_class = class!(NSString);
            let is_string: BOOL = msg_send![body, isKindOfClass: string_class];
            if is_string == YES {
                let str = string_from_nsstring(body);
                let str = str.as_ref().ok_or("Failed to get message ref")?;
                info!(
                    "Got message from JavaScript message='{}' - has_callback={} self={:?} webview={:?} id={}",
                    str,
                    self.on_message_callback.is_some(),
                    self as *const WebviewHolder as *const c_void,
                    self.webview.get_native_handle(),
                    self.id
                );

                self.on_message_callback
                    .map(|cb| {
                        cb(str.clone());
                    })
                    .ok_or("No callback provided")?;

                Ok(())
            } else {
                Err("Message wasn't a string")
            }
        };

        if let Err(err) = run() {
            error!("Message handling failed: {}", err);
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
    let _: () = msg_send![webview_id, setTranslatesAutoresizingMaskIntoConstraints: NO];

    let mut constraints: Vec<id> = vec![];

    {
        let parent_anchor: id = msg_send![parent_id, leftAnchor];
        let target_anchor: id = msg_send![webview_id, leftAnchor];
        let constraint: id =
            msg_send![parent_anchor, constraintEqualToAnchor: target_anchor constant: 0.0];
        constraints.push(constraint)
    }
    {
        let parent_anchor: id = msg_send![parent_id, rightAnchor];
        let target_anchor: id = msg_send![webview_id, rightAnchor];
        let constraint: id =
            msg_send![parent_anchor, constraintEqualToAnchor: target_anchor constant: 0.0];
        constraints.push(constraint)
    }
    {
        let parent_anchor: id = msg_send![parent_id, topAnchor];
        let target_anchor: id = msg_send![webview_id, topAnchor];
        let constraint: id =
            msg_send![parent_anchor, constraintEqualToAnchor: target_anchor constant: 0.0];
        constraints.push(constraint)
    }
    {
        let parent_anchor: id = msg_send![parent_id, bottomAnchor];
        let target_anchor: id = msg_send![webview_id, bottomAnchor];
        let constraint: id =
            msg_send![parent_anchor, constraintEqualToAnchor: target_anchor constant: 0.0];
        constraints.push(constraint)
    }

    let bundle = NSArray::arrayWithObjects(nil, &constraints);
    NSLayoutConstraint::activateConstraints(nil, bundle);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ptr::null;

    #[test]
    fn test_ptr_dance() {
        unsafe {
            struct Test {
                field: f32,
            }

            let mut t = Test { field: 0.21 };
            let t_ref = &mut t;
            let t_ptr = t_ref as *mut Test as *mut c_void;
            let t_ref2 = &mut *(t_ptr as *mut Test);
            assert_eq!(t_ref2.field, 0.21)
        }
    }

    #[test]
    fn test_make_new_handler() {
        struct Test {
            value: f32,
            other: Option<f32>,
        }

        unsafe {
            static mut CALLED_WITH: *const c_void = null::<c_void>();
            unsafe extern "C" fn on_message(self_ptr: *mut c_void, _: id, _: id) {
                CALLED_WITH = self_ptr;
            }

            let mut test = Test {
                value: 0.32,
                other: None,
            };
            let data = (&mut test) as *mut Test as *mut c_void;
            let handler = make_new_handler("test", on_message, data);

            let _: () = msg_send![handler, userContentController:nil didReceiveScriptMessage: nil];
            assert_ne!(CALLED_WITH, null());
            assert_eq!(CALLED_WITH, data);

            let data_called_with: &mut Test = &mut *(CALLED_WITH as *mut Test);
            assert_eq!(data_called_with.value, 0.32);
            assert_eq!(data_called_with.other, None);
        }
    }

    // Activating constraints in a test will panic.
    #[test]
    fn test_pin_to_parent() {
        unsafe {
            let origin = NSPoint::new(0.0, 0.0);
            let size = NSSize::new(500.0, 500.0);
            let frame = NSRect::new(origin, size);
            let _parent_view = NSView::initWithFrame_(NSView::alloc(nil), frame);
            let _child_view = NSView::initWithFrame_(NSView::alloc(nil), frame);

            // pin_to_parent(parent_view, child_view, false);
        }
    }
}
