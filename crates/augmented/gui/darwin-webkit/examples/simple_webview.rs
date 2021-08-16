extern crate cocoa;
extern crate darwin_webkit;

use darwin_webkit::helpers::dwk_app::DarwinWKApp;

fn main() {
    unsafe {
        let app = DarwinWKApp::new("Simple WebView");
        let webview = app.create_webview();

        webview.load_url("https://www.google.com.br");

        app.set_webview(&webview);
        app.run();
    }
}
