//! Higher-level wrapper.
//!
//! Exposes two structs:
//!
//! `DarwinWKApp`, which is not to be used seriously but
//! nice for testing. This configures the `NSApplication` and opens a
//! `NSWindow`.
//!
//! `DarwinWKWebView` wraps `WKWebView`, but unlike generic wrappers, allows
//! for it to be used in the context of an existing window (for example, in vst
//! plugins).
pub mod dwk_app;
pub mod dwk_webview;
