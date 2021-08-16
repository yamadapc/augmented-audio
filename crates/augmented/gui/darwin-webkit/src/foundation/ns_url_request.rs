//! # NSURLRequest
//!
//! Incomplete bindings to `NSURLRequest`
//!
//! See https://developer.apple.com/documentation/foundation/nsurlrequest
use cocoa::base::id;
use cocoa::foundation::NSTimeInterval;

extern "C" {
    pub static NSURLRequest: id;
}

pub trait NSURLRequest: Sized {
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSURLRequest), alloc]
    }

    // Creating Requests
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn requestWithURL_(_: Self, url: id) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initWithURL_(self, url: id) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn requestWithURL_cachePolicy_timeoutInterval(
        _: Self,
        url: id,
        cachePolicy: id,
        timeoutInterval: NSTimeInterval,
    ) -> id;
    /// # Safety
    /// All the FFI functions are unsafe.
    unsafe fn initWithURL_cachePolicy_timeoutInterval(
        self,
        url: id,
        cachePolicy: id,
        timeoutInterval: NSTimeInterval,
    ) -> id;

    // TODO:
    // Working with a Cache Policy
    // Accessing Request Components
    // Getting Header Fields
    // Controlling Request Behavior
    // Accessing the Service Type
    // Supporting Secure Coding
}

impl NSURLRequest for id {
    // Creating Requests
    unsafe fn requestWithURL_(_: Self, url: id) -> id {
        msg_send![class!(NSURLRequest), requestWithURL_: url]
    }

    unsafe fn initWithURL_(self, url: id) -> id {
        msg_send![self, initWithURL: url]
    }

    unsafe fn requestWithURL_cachePolicy_timeoutInterval(
        _: Self,
        url: id,
        cachePolicy: id,
        timeoutInterval: NSTimeInterval,
    ) -> id {
        msg_send![
            class!(NSURLRequest),
            requestWithURL:url
                cachePolicy:cachePolicy
                timeoutInterval:timeoutInterval
        ]
    }

    unsafe fn initWithURL_cachePolicy_timeoutInterval(
        self,
        url: id,
        cachePolicy: id,
        timeoutInterval: NSTimeInterval,
    ) -> id {
        msg_send![
            self,
            initWithURL:url
                cachePolicy:cachePolicy
                timeoutInterval:timeoutInterval
        ]
    }

    // TODO:
    // Working with a Cache Policy
    // Accessing Request Components
    // Getting Header Fields
    // Controlling Request Behavior
    // Accessing the Service Type
    // Supporting Secure Coding
}
