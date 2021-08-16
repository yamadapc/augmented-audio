//! `DarwinWKApp` configures the `NSApplication` and opens a `NSWindow`.
use super::dwk_webview::*;

use cocoa::appkit::{
    NSApp, NSApplication, NSApplicationActivateIgnoringOtherApps,
    NSApplicationActivationPolicyRegular, NSBackingStoreBuffered, NSMenu, NSMenuItem,
    NSRunningApplication, NSWindow, NSWindowStyleMask,
};
use cocoa::base::{id, nil, selector, NO};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSProcessInfo, NSRect, NSSize, NSString};

/// Wraps an NSApplication instance with a main window that contains WebView.
///
/// See `DarwinWKWebView` as well.
///
/// # Example
///
/// ```no_run
/// use darwin_webkit::helpers::dwk_app::DarwinWKApp;
/// use std::rc::Rc;
///
/// unsafe {
///     let app = DarwinWKApp::new("Host an app");
///     let webview = Rc::new(app.create_webview());
///
///     // add handlers, load HTML, etc...
///
///     app.set_webview(&webview);
///     app.run();
/// }
/// ```
pub struct DarwinWKApp {
    /// The NSApplication instance
    nsapp: id,
    /// The NSWindow instance
    main_window: id,
}

impl DarwinWKApp {
    /// Create an app with the given window title
    ///
    /// # Safety
    /// All the FFI functions are unsafe.
    pub unsafe fn new(windowTitle: &str) -> DarwinWKApp {
        let _pool = NSAutoreleasePool::new(nil);

        let app = NSApp();

        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        // create Menu Bar
        let menubar = NSMenu::new(nil).autorelease();
        let app_menu_item = NSMenuItem::new(nil).autorelease();
        menubar.addItem_(app_menu_item);
        app.setMainMenu_(menubar);

        // create Application menu
        let app_menu = NSMenu::new(nil).autorelease();
        let quit_prefix = NSString::alloc(nil).init_str("Quit ");
        let quit_title =
            quit_prefix.stringByAppendingString_(NSProcessInfo::processInfo(nil).processName());
        let quit_action = selector("terminate:");
        let quit_key = NSString::alloc(nil).init_str("q");
        let quit_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(quit_title, quit_action, quit_key)
            .autorelease();
        app_menu.addItem_(quit_item);
        app_menu_item.setSubmenu_(app_menu);

        // create Window
        let styleMask = NSWindowStyleMask::NSTitledWindowMask
            | NSWindowStyleMask::NSClosableWindowMask
            | NSWindowStyleMask::NSResizableWindowMask;

        let window = NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(0., 0.), NSSize::new(800., 800.)),
                styleMask,
                NSBackingStoreBuffered,
                NO,
            )
            .autorelease();
        window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
        window.center();

        let title = NSString::alloc(nil).init_str(windowTitle);
        window.setTitle_(title);
        window.makeKeyAndOrderFront_(nil);

        DarwinWKApp {
            nsapp: app,
            main_window: window,
        }
    }

    /// Get the NSApplication handle
    pub fn get_app_native_handle(&self) -> id {
        self.nsapp
    }

    /// Get the NSWindow handle
    pub fn get_window_native_handle(&self) -> id {
        self.main_window
    }

    /// Start the NSApplication and activate it ignoring other apps so it comes to front.
    ///
    /// # Safety
    /// All the FFI functions are unsafe.
    pub unsafe fn run(&self) {
        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
        self.nsapp.run();
    }

    /// Stop the NSApplication run loop.
    ///
    /// # Safety
    /// All the FFI functions are unsafe.
    pub unsafe fn stop(&self) {
        msg_send![self.nsapp, stop: nil]
    }

    /// Create a webview that has this app window's frame.
    ///
    /// # Safety
    /// All the FFI functions are unsafe.
    pub unsafe fn create_webview(&self) -> DarwinWKWebView {
        let frame = NSWindow::frame(self.main_window);
        DarwinWKWebView::new(frame)
    }

    /// Set the content view of the main window to a certain webview
    ///
    /// # Safety
    /// All the FFI functions are unsafe.
    pub unsafe fn set_webview<'a>(&'a self, webview: &'a DarwinWKWebView) {
        self.main_window
            .setContentView_(webview.get_native_handle());
    }
}

unsafe impl Send for DarwinWKApp {}
unsafe impl Sync for DarwinWKApp {}
