use cocoa::base::id;

#[link(name = "WebKit", kind = "framework")]
extern "C" {
    pub static WKUserContentController: id;
}

pub trait WKUserContentController: Sized {
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(WKUserContentController), alloc]
    }

    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn init(self) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// `message_handler` should be a `WKScriptMessageHandler` and `name` an `NSString`.
    ///
    /// `make_new_handler` is provided to create `WKScriptMessageHandler`s from closures, but it's
    /// also not safe.
    unsafe fn addScriptMessageHandler(self, message_handler: id, name: id) -> id;
}

impl WKUserContentController for id {
    unsafe fn init(self) -> id {
        msg_send![self, init]
    }

    unsafe fn addScriptMessageHandler(self, message_handler: id, name: id) -> id {
        msg_send![self, addScriptMessageHandler: message_handler name: name]
    }
}

#[link(name = "WebKit", kind = "framework")]
extern "C" {
    pub static WKWebViewConfiguration: id;
}

pub trait WKWebViewConfiguration: Sized {
    /// # Safety
    /// All the FFI functions are unsafe
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(WKWebViewConfiguration), alloc]
    }
    /// # Safety
    /// All the FFI functions are unsafe
    unsafe fn init(self) -> id;

    /// # Safety
    /// All the FFI functions are unsafe.
    /// `id` should point to a `WKUserContentController`
    unsafe fn setUserContentController(self, controller: id);
}

impl WKWebViewConfiguration for id {
    unsafe fn init(self) -> id {
        msg_send![self, init]
    }

    unsafe fn setUserContentController(self, controller: id) {
        msg_send![self, setUserContentController: controller]
    }
}
