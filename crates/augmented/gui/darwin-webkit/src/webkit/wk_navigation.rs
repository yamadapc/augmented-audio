use cocoa::base::id;

#[link(name = "WebKit", kind = "framework")]
extern "C" {
    pub static WKNavigation: id;
}

pub trait WKNavigation: Sized {
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(WKNavigation), alloc]
    }
}
