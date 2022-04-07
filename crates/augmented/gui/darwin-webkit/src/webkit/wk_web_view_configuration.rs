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
