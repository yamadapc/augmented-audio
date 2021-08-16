//! Wraps a `WKWebView`, `WKWebViewConfiguration` & `WKUserContentController`
use cocoa::appkit::{NSView, NSViewHeightSizable, NSViewWidthSizable};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSRect, NSString};

use block::ConcreteBlock;
use foundation::*;
use webkit::wk_script_message_handler::make_new_handler;
use webkit::*;

/// Wraps a `WKWebView`, `WKWebViewConfiguration` & `WKUserContentController`
///
/// Can be used from a cocoa application with `get_native_handle` or with `DWKApp`.
///
/// # Example
/// ```no_run
/// extern crate cocoa;
/// use cocoa::base::id;
/// use darwin_webkit::helpers::dwk_app::DarwinWKApp;
/// use std::sync::Arc;
///
/// unsafe {
///     let app = DarwinWKApp::new("Host an app");
///     let webview = Arc::new(app.create_webview());
///     
///     let callback_webview = webview.clone();
///     let callback = Box::into_raw(Box::new(Box::new(|_: id, _: id| {
///         println!("JavaScript called rust!");
///         callback_webview.evaluate_javascript(
///             "document.body.innerHTML += ' -> response from rust<br />';"
///         );
///     })));
///
///     webview.add_message_handler("hello", callback);
///     webview.load_html_string("
///         <script>
///             document.body.innerHTML += 'start';
///             window.webkit.messageHandlers.hello.postMessage('hello');
///         </script>
///     ", "", );
/// }
/// ```
pub struct DarwinWKWebView {
    webview: id,
    configuration: id,
    content_controller: id,
}

impl DarwinWKWebView {
    /// Create a webview with the given frame rect. Also creates the supporting configuration and
    /// content controller.
    ///
    /// Thee view is resizable other options are empty.
    ///
    /// # Safety
    /// All the FFI functions are unsafe.
    pub unsafe fn new(frame: NSRect) -> DarwinWKWebView {
        let configuration = WKWebViewConfiguration::init(WKWebViewConfiguration::alloc(nil));
        let content_controller = WKUserContentController::init(WKUserContentController::alloc(nil));
        configuration.setUserContentController(content_controller);
        let webview = WKWebView::alloc(nil).initWithFrame_configuration_(frame, configuration);

        NSView::setAutoresizingMask_(webview, NSViewWidthSizable | NSViewHeightSizable);

        DarwinWKWebView {
            webview,
            configuration,
            content_controller,
        }
    }

    /// Get the `WKWebView` instance
    pub fn get_native_handle(&self) -> id {
        self.webview
    }

    /// Get the `WKUserContentController` instance
    pub fn get_user_content_controller_handle(&self) -> id {
        self.content_controller
    }

    /// Get the `WKWebViewConfiguration` instance
    pub fn get_configuration_handle(&self) -> id {
        self.configuration
    }

    /// Load an URL onto the WebView.
    ///
    /// # Safety
    /// All the FFI functions are unsafe.
    pub unsafe fn load_url(&self, url: &str) {
        let url = NSString::alloc(nil).init_str(url);
        let url = NSURL::alloc(nil).initWithString_(url);
        let req = NSURLRequest::alloc(nil).initWithURL_(url);
        self.webview.loadRequest_(req);
    }

    /// Load an HTML string onto the WebView.
    ///
    /// # Safety
    /// All the FFI functions are unsafe.
    pub unsafe fn load_html_string(&self, html: &str, base_url: &str) {
        let html = NSString::alloc(nil).init_str(html);
        let base_url = NSString::alloc(nil).init_str(base_url);
        let base_url = NSURL::alloc(nil).initWithString_(base_url);
        self.webview.loadHTMLString_baseURL_(html, base_url);
    }

    /// Evaluate a JavaScript string on the WebView
    ///
    /// # Safety
    /// All the FFI functions are unsafe.
    pub unsafe fn evaluate_javascript(&self, javascript: &str) {
        let javascript = NSString::alloc(nil).init_str(javascript);
        let b = |_: id, error: id| {
            if error != nil {
                let str = msg_send![error, localizedDescription];
                let str = string_from_nsstring(str);
                println!("Error {}", str.as_ref().unwrap().as_str());
                return;
            }
        };
        let b = ConcreteBlock::new(b);
        let b = b.copy();
        self.webview.evaluateJavaScript_(javascript, &b);
    }

    /// Register a callback into the WebView.
    ///
    /// Calls `make_new_handler` under the hood. The callback should have form:
    ///
    /// ```compile_fail
    /// FnMut(id /* WKUserContentController */, id /* WKScriptMessage */)
    /// ```
    ///
    /// The handler will be available from JavaScript through:
    ///
    /// ```javascript
    /// window.webkit.messageHandlers.name.postMessage('some message');
    /// ```
    ///
    /// # Safety
    /// All the FFI functions are unsafe.
    ///
    /// Your callback will be called from WebKit. If the WebView outlives it: 💥.
    pub unsafe fn add_message_handler<Func>(&self, name: &str, callback: *mut Func)
    where
        Func: FnMut(id, id),
    {
        let handler = make_new_handler(format!("DWKHandler_{}", name).as_str(), callback);
        let name = NSString::alloc(nil).init_str(name);
        self.content_controller
            .addScriptMessageHandler(handler, name);
    }
}

pub extern "C" fn javascript_callback(_: id, _: id) {}

unsafe impl Send for DarwinWKWebView {}
unsafe impl Sync for DarwinWKWebView {}

/// Create a `String` pointer from a `NSString`.
///
/// # Safety
/// All the FFI functions are unsafe.
pub unsafe fn string_from_nsstring(nsstring: id) -> *mut String {
    let len = nsstring.len();
    let str = Box::new(String::from_utf8_unchecked(Vec::from_raw_parts(
        nsstring.UTF8String() as *mut u8,
        len,
        len,
    )));
    Box::into_raw(str)
}
