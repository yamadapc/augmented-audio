extern crate cocoa;
extern crate darwin_webkit;

use cocoa::base::id;
use cocoa::foundation::NSString;
use darwin_webkit::helpers::dwk_app::DarwinWKApp;
use darwin_webkit::webkit::wk_script_message_handler::WKScriptMessage;
use std::rc::Rc;

fn main() {
    unsafe {
        let app = DarwinWKApp::new("Host an app");
        let webview = Rc::new(app.create_webview());

        let mut callback = |_: id, _: id| {
            println!("JavaScript called rust!");
            webview
                .evaluate_javascript("document.body.innerHTML += ' -> response from rust<br />';");
        };
        webview.add_message_handler("hello", &mut callback);

        let mut callback = |_: id, payload: id| {
            println!("JavaScript interval called rust!");
            let body = payload.body();
            let str = Box::new(String::from_utf8_unchecked(Vec::from_raw_parts(
                body.UTF8String() as *mut u8,
                body.len(),
                body.len(),
            )));
            let str = Box::into_raw(str);
            webview.evaluate_javascript(
                format!(
                    "document.body.innerHTML += 'interval tick - received: {}<br />';",
                    *str
                )
                .as_str(),
            );
            println!("  Message {}", *str);
        };
        webview.add_message_handler("interval", &mut callback);

        webview.load_html_string(
            "
            <h1>Hello there</h1>

            <script>
                document.body.innerHTML += 'start';
                window.webkit.messageHandlers.hello.postMessage('hello');
                document.body.innerHTML += ' -> success';

                setInterval(() => {
                    window.webkit.messageHandlers.interval.postMessage('' + Math.random());
                }, 100);
            </script>
            ",
            "",
        );

        app.set_webview(&webview);
        app.run();
    }
}
