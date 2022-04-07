// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
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
