//! Wraps the WKScriptMessageHandler protocol in Rust

use cocoa::base::id;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use std::os::raw::c_void;

pub trait WKScriptMessage: Sized {
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn body(self) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn frameInfo(self) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn name(self) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn webView(self) -> id;
}

impl WKScriptMessage for id {
    unsafe fn body(self) -> id {
        msg_send![self, body]
    }

    unsafe fn frameInfo(self) -> id {
        msg_send![self, frameInfo]
    }

    unsafe fn name(self) -> id {
        msg_send![self, name]
    }

    unsafe fn webView(self) -> id {
        msg_send![self, webView]
    }
}

extern "C" fn call_ptr<Func>(this: &Object, _sel: Sel, controller: id, message: id)
where
    Func: FnMut(id, id),
{
    unsafe {
        let instance_ptr: *mut c_void = msg_send![this, instancePtr];
        let instance_ptr: *mut Func = instance_ptr as *mut Func;
        (*instance_ptr)(controller, message);
    }
}

extern "C" fn get_instance_ptr(this: &Object, _sel: Sel) -> *const c_void {
    unsafe { *this.get_ivar("_instance_ptr") }
}

extern "C" fn set_instance_ptr(this: &mut Object, _sel: Sel, instance_ptr: *const c_void) {
    unsafe { this.set_ivar("_instance_ptr", instance_ptr) };
}

/// Wraps a callback of type `FnMut(id /* WKUserContentController */, id /* WKScriptMessage */)` so
/// it can be registered onto the `WKUserContentController` with
/// `WKUserContentController::addScriptMessageHandler`.
///
/// # Safety
/// All the FFI functions are unsafe.
///
/// Your callback will be called from WebKit. If the WebView outlives it: 💥.
pub unsafe fn make_new_handler<Func>(name: &str, func: *mut Func) -> id
where
    Func: FnMut(id, id),
{
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new(name, superclass).unwrap();

    decl.add_ivar::<*const c_void>("_instance_ptr");
    decl.add_method::<extern "C" fn(&Object, Sel) -> *const c_void>(
        sel!(instancePtr),
        get_instance_ptr,
    );
    decl.add_method::<extern "C" fn(&mut Object, Sel, *const c_void)>(
        sel!(setInstancePtr:),
        set_instance_ptr,
    );
    decl.add_method::<extern "C" fn(&Object, Sel, id, id)>(
        sel!(userContentController:didReceiveScriptMessage:),
        call_ptr::<Func>,
    );
    decl.register();

    let class = Class::get(name).unwrap();
    let instance: id = msg_send![class, alloc];
    let instance: id = msg_send![instance, init];
    instance.setInstancePtr(func as *mut c_void);

    instance
}

pub trait WKScriptMessageHandlerBridge: Sized {
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn instancePtr(self) -> *mut c_void;

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn setInstancePtr(self, _: *mut c_void);
}

impl WKScriptMessageHandlerBridge for id {
    unsafe fn instancePtr(self) -> *mut c_void {
        msg_send![self, instancePtr]
    }

    unsafe fn setInstancePtr(self, instance_ptr: *mut c_void) {
        msg_send![self, setInstancePtr: instance_ptr]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cocoa::base::nil;
    use std::borrow::{Borrow, BorrowMut};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_casting_fn_pointer() {
        unsafe {
            {
                let mut called: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
                let callback: Box<dyn FnMut(id, id)> = Box::new(|_: id, _: id| {
                    called.borrow_mut().replace(true);
                });
                let callback_void: *mut dyn FnMut(id, id) = Box::into_raw(callback);
                let new_callback: *mut dyn FnMut(id, id) = callback_void as *mut dyn FnMut(id, id);
                (*new_callback)(nil, nil);
                let was_called: &RefCell<bool> = called.borrow();
                assert!(was_called.clone().into_inner());
            }
        }
    }

    #[test]
    fn test_webview_message_handler() {
        unsafe {
            {
                let called: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
                let local_called = called.clone();
                let callback = Box::new(Box::new(move |_: id, _: id| {
                    let cell: &RefCell<bool> = called.borrow();
                    cell.replace(true);
                }));
                let callback = Box::into_raw(callback);
                let handler = make_new_handler("default", callback);

                let _: id =
                    msg_send![handler, userContentController:nil didReceiveScriptMessage:nil];

                let was_called: &RefCell<bool> = local_called.borrow();
                assert!(was_called.clone().into_inner());
            }
        }
    }
}
