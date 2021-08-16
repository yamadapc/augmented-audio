use cocoa::appkit::NSApp;
use cocoa::base::nil;
use cocoa::foundation::NSAutoreleasePool;
use webview_holder::WebviewHolder;

fn main() {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let _app = NSApp();
        let holder = WebviewHolder::new((100, 100));
        holder
            .webview()
            .evaluate_javascript("console.log('hello world')");
    }
}
