use cocoa::appkit::{NSLayoutConstraint, NSView, NSWindow, NSWindowStyleMask};
use cocoa::base::{id, nil, NO, YES};
use cocoa::foundation::{NSArray, NSPoint, NSRect, NSSize, NSString};
use darwin_webkit::helpers::dwk_webview::{string_from_nsstring, DarwinWKWebView};
use darwin_webkit::webkit::wk_script_message_handler::{
    WKScriptMessage, WKScriptMessageHandlerBridge,
};
use darwin_webkit::webkit::WKUserContentController;
use log::{error, info};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, BOOL};
use serde::Serialize;
use std::error::Error;
use std::ffi::c_void;
use std::time::Instant;

extern "C" fn call_ptr<Func, T>(this: &mut Object, _sel: Sel, controller: id, message: id)
where
    Func: FnMut(*mut T, id, id),
{
    unsafe {
        let instance_ptr: *mut c_void = *this.get_ivar("instance_ptr");
        let instance_ptr: *mut Func = instance_ptr as *mut Func;
        let data: *mut c_void = *this.get_ivar("data");
        let data: *mut T = data as *mut T;
        (*instance_ptr)(data, controller, message);
    }
}

pub unsafe fn make_new_handler<Func, T>(name: &str, func: *mut Func, data: *mut T) -> id
where
    Func: FnMut(*mut T, id, id),
{
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new(name, superclass).unwrap();

    decl.add_ivar::<*const c_void>("instance_ptr");
    decl.add_ivar::<*const c_void>("data");
    decl.add_method::<extern "C" fn(&mut Object, Sel, id, id)>(
        sel!(userContentController:didReceiveScriptMessage:),
        call_ptr::<Func, T>,
    );
    decl.register();

    let class = Class::get(name).unwrap();
    let instance: id = msg_send![class, alloc];
    let mut instance: id = msg_send![instance, init];
    {
        let mut instance = instance.as_mut().unwrap();
        instance.set_ivar("instance_ptr", func as *mut c_void);
        instance.set_ivar("data", data as *mut c_void);
    }

    instance
}

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
        info!("WebviewHolder::initialize - Attaching to parent NSView");
        self.attach_to_parent(parent);

        info!("WebviewHolder::initialize - Setting-up message handler");
        self.attach_message_handler();

        // TODO - this should be read from somewhere
        info!("WebviewHolder::initialize - Loading app URL");
        self.webview.load_url("http://127.0.0.1:3000");
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

        let ns_rect: NSRect = NSView::frame(webview_id);
        let ns_size = ns_rect.size;
        window_id.setMinSize_(ns_size);
    }

    unsafe fn attach_message_handler(&mut self) {
        let on_message_ptr =
            Box::into_raw(Box::new(|self_ptr: *mut Self, _, wk_script_message| {
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
            self,
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
                    "Got message from JavaScript message='{}' - has_callback={}",
                    str,
                    self.on_message_callback.is_some()
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
        run().map_err(|err| {
            error!("Message handling failed: {}", err);
        });
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
