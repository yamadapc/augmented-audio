use cocoa::appkit::NSBackingStoreType::NSBackingStoreBuffered;
use cocoa::appkit::{NSWindow, NSWindowStyleMask};
use cocoa::base::{nil, NO};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString};
use raw_window_handle::macos::MacOSHandle;
use raw_window_handle::RawWindowHandle;
use std::ffi::c_void;
use vst::editor::Editor;

pub struct PluginWindowHandle {
    pub editor: Box<dyn Editor>,
    pub raw_window_handle: RawWindowHandle,
}

pub fn open_plugin_window(mut editor: Box<dyn Editor>, size: (i32, i32)) -> PluginWindowHandle {
    let _pool = unsafe { NSAutoreleasePool::new(nil) };
    let (width, height) = size;
    let rect = NSRect::new(
        NSPoint::new(0.0, 0.0),
        NSSize::new(width as f64, height as f64),
    );
    let ns_window = unsafe {
        let ns_window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
            rect,
            NSWindowStyleMask::NSTitledWindowMask | NSWindowStyleMask::NSClosableWindowMask,
            NSBackingStoreBuffered,
            NO,
        );
        // .autorelease();
        ns_window.center();

        let title = NSString::alloc(nil).init_str("plugin-window"); // .autorelease();
        ns_window.setTitle_(title);

        ns_window.makeKeyAndOrderFront_(nil);

        ns_window
    };
    let ns_view = unsafe { ns_window.contentView() };
    let raw_window_handle = RawWindowHandle::MacOS(MacOSHandle {
        ns_window: ns_window as *mut c_void,
        ns_view: ns_view as *mut c_void,
        ..MacOSHandle::empty()
    });
    editor.open(ns_view as *mut c_void);

    PluginWindowHandle {
        editor,
        raw_window_handle,
    }
}
