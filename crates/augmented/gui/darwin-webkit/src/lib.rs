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
