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
//! **darwin_webkit** exposes bindings to some of the WebKit's API on MacOS for
//! Rust.
//!
//! Modules follow the `cocoa` crate convention for naming bindings.
//!
//! It uses the `objc` and `cocoa` crates to bind with Objective-C.
//!
//! The `darwin_webkit::foundation` module exposes some dependencies to using
//! the WKWebView APIs, like `NSURLRequest`.
//!
//! The `darwin_webkit::webkit` module exposes bindings to the `WKWebView` API.
//!
//! In `darwin_webkit::helpers` there's a very small higher level wrapper that
//! may turn into a higher level API.
//!
//! Callbacks from JavaScript to rust may be registered with:
//!
//! * `darwin_webkit::webkit::wk_script_message_handler::make_new_handler`
//! * or `darwin_webkit::helpers::DarwinWKWebView`
//!
//! Rust may evaluate JavaScript and HTML with:
//!
//! * `darwin_webkit::helpers::DarwinWKWebView::evaluate_javascript`
//! * `darwin_webkit::helpers::DarwinWKWebView::load_url`
//! * `darwin_webkit::helpers::DarwinWKWebView::load_html_string`
#![allow(non_snake_case)]

extern crate cocoa;
extern crate core_graphics;
extern crate libc;
#[macro_use]
extern crate objc;
extern crate block;

pub mod foundation;
pub mod helpers;
pub mod webkit;
