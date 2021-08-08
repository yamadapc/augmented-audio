#[macro_use]
extern crate objc;

use cocoa::appkit::NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular;
use cocoa::base::Class;
use cocoa::{
    appkit::{
        NSApp, NSApplication, NSBackingStoreType::NSBackingStoreBuffered, NSEventModifierFlags,
        NSWindow, NSWindowStyleMask,
    },
    base::{id, nil, NO},
    foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString},
};
use lazy_static::lazy_static;
use objc::{
    declare::ClassDecl,
    msg_send,
    runtime::{Object, Sel},
    sel,
};

struct AppDelegate {
    id: *mut Object,
}

impl AppDelegate {
    fn new() -> Self {
        let id = unsafe {
            let instance: id = msg_send![APP_DELEGATE_CLASS.0, alloc];
            msg_send![instance, init]
        };

        Self { id }
    }

    fn id(&self) -> *mut Object {
        self.id
    }
}

pub struct AppDelegateClass(pub *const objc::runtime::Class);
unsafe impl Send for AppDelegateClass {}
unsafe impl Sync for AppDelegateClass {}

lazy_static! {
    pub static ref APP_DELEGATE_CLASS: AppDelegateClass = unsafe {
        let superclass = class!(NSResponder);
        let mut decl = ClassDecl::new("ExampleAppDelegate", superclass).unwrap();
        decl.add_method(
            sel!(applicationWillFinishLaunching:),
            will_finish_launching as extern "C" fn(&Object, Sel, id),
        );
        decl.add_method(
            sel!(applicationDidFinishLaunching:),
            did_finish_launching as extern "C" fn(&Object, Sel, id),
        );
        AppDelegateClass(decl.register())
    };
}

extern "C" fn will_finish_launching(this: &Object, _: Sel, _: id) {
    log::info!("Running `applicationWillFinishLaunching`");
}

extern "C" fn did_finish_launching(this: &Object, _: Sel, _: id) {
    log::info!("Running `applicationDidFinishLaunching`");
}

struct KeyEquivalent<'a> {
    key: &'a str,
    masks: Option<NSEventModifierFlags>,
}

fn menu_item(
    title: *mut Object,
    selector: Sel,
    key_equivalent: Option<KeyEquivalent<'_>>,
) -> *mut Object {
    use cocoa::appkit::NSMenuItem;
    unsafe {
        let (key, masks) = match key_equivalent {
            Some(ke) => (NSString::alloc(nil).init_str(ke.key), ke.masks),
            None => (NSString::alloc(nil).init_str(""), None),
        };
        let item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(title, selector, key);
        if let Some(masks) = masks {
            item.setKeyEquivalentModifierMask_(masks)
        }

        item
    }
}

fn initialize_menu() -> *mut Object {
    use cocoa::appkit::{NSMenu, NSMenuItem};
    use cocoa::base::selector;
    use cocoa::foundation::NSProcessInfo;

    unsafe {
        let menubar = NSMenu::new(nil);
        let app_menu_item = NSMenuItem::new(nil);
        menubar.addItem_(app_menu_item);

        let app_menu = NSMenu::new(nil);
        let process_name = NSProcessInfo::processInfo(nil).processName();

        // About menu item
        let about_item_prefix = NSString::alloc(nil).init_str("About ");
        let about_item_title = about_item_prefix.stringByAppendingString_(process_name);
        let about_item = menu_item(
            about_item_title,
            selector("orderFrontStandardAboutPanel:"),
            None,
        );

        // Seperator menu item
        let sep_first = NSMenuItem::separatorItem(nil);

        // Hide application menu item
        let hide_item_prefix = NSString::alloc(nil).init_str("Hide ");
        let hide_item_title = hide_item_prefix.stringByAppendingString_(process_name);
        let hide_item = menu_item(
            hide_item_title,
            selector("hide:"),
            Some(KeyEquivalent {
                key: "h",
                masks: None,
            }),
        );

        // Hide other applications menu item
        let hide_others_item_title = NSString::alloc(nil).init_str("Hide Others");
        let hide_others_item = menu_item(
            hide_others_item_title,
            selector("hideOtherApplications:"),
            Some(KeyEquivalent {
                key: "h",
                masks: Some(
                    NSEventModifierFlags::NSAlternateKeyMask
                        | NSEventModifierFlags::NSCommandKeyMask,
                ),
            }),
        );

        // Show applications menu item
        let show_all_item_title = NSString::alloc(nil).init_str("Show All");
        let show_all_item = menu_item(
            show_all_item_title,
            selector("unhideAllApplications:"),
            None,
        );

        // Seperator menu item
        let sep = NSMenuItem::separatorItem(nil);

        // Quit application menu item
        let quit_item_prefix = NSString::alloc(nil).init_str("Quit ");
        let quit_item_title = quit_item_prefix.stringByAppendingString_(process_name);
        let quit_item = menu_item(
            quit_item_title,
            selector("terminate:"),
            Some(KeyEquivalent {
                key: "q",
                masks: None,
            }),
        );

        app_menu.addItem_(about_item);
        app_menu.addItem_(sep_first);
        app_menu.addItem_(hide_item);
        app_menu.addItem_(hide_others_item);
        app_menu.addItem_(show_all_item);
        app_menu.addItem_(sep);
        app_menu.addItem_(quit_item);
        app_menu_item.setSubmenu_(app_menu);

        menubar
    }
}

fn main() {
    wisual_logger::init_from_env();

    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let app = NSApp();
        // Make the app show-up as a proper app with menu-bar & dock item
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        log::info!("Creating main window");
        let rect = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(500.0, 500.0));
        let ns_window = {
            let ns_window = NSWindow::alloc(nil)
                .initWithContentRect_styleMask_backing_defer_(
                    rect,
                    NSWindowStyleMask::NSTitledWindowMask | NSWindowStyleMask::NSClosableWindowMask,
                    NSBackingStoreBuffered,
                    NO,
                )
                .autorelease();
            ns_window.center();

            let title = NSString::alloc(nil).init_str(&"Main").autorelease();
            ns_window.setTitle_(title);
            ns_window.makeKeyAndOrderFront_(nil);

            ns_window
        };

        let menu = initialize_menu();
        app.setMainMenu_(menu);
        let app_delegate = AppDelegate::new();
        app.setDelegate_(app_delegate.id());

        app.run();
    }
}
