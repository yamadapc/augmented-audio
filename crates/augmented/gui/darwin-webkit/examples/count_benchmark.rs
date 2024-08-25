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

#[cfg(target_os = "macos")]
extern crate cocoa;
extern crate criterion;
extern crate darwin_webkit;

#[cfg(target_os = "macos")]
mod implementation {
    pub use cocoa::base::id;
    pub use darwin_webkit::helpers::dwk_app::DarwinWKApp;
    pub use darwin_webkit::helpers::dwk_webview::DarwinWKWebView;
    pub use std::sync::mpsc::{channel, Receiver};
    pub use std::sync::Arc;
    pub use std::thread;
    pub use std::time::{Duration, Instant};

    pub unsafe fn setup_count_bench(webview: Arc<DarwinWKWebView>, n: u64) -> Receiver<()> {
        println!("Starting webview");
        let (sender, receiver) = channel();

        let mut i = 0;
        let message_sender = sender;
        let cb_webview = webview.clone();
        let callback = Box::into_raw(Box::new(Box::new(move |_: id, _message: id| {
            i += 1;
            let value = i;

            if value > n {
                message_sender.send(()).unwrap();
            } else {
                let main_cb_webview = cb_webview.clone();
                dispatch::Queue::main().exec_async(move || {
                    main_cb_webview.evaluate_javascript(format!("onMessage('{}')", value).as_str());
                });
            }
        })));

        webview.add_message_handler("general", callback);

        receiver
    }
}

#[cfg(target_os = "macos")]
fn main() {
    use implementation::*;

    println!("Measuring latency to send and receive messages from a WebView.");
    println!("Will:\n");
    println!("  1. Send a message to JavaScript");
    println!("  2. When a message is received in JavaScript, send a message back");
    println!("  3. When a message is received in Rust, send a message back");
    println!("  4. Measure the time to send N messages in this fashion");
    println!("\nThe result should indicate the round-trip latency of sending/receiving messages to/from JavaScript");
    unsafe {
        let app = Arc::new(DarwinWKApp::new("Host an app"));
        let webview = Arc::new(app.create_webview());
        webview.load_html_string(
            r"
                <script>
                window.onMessage = function onMessage(n) {
                    // n = +n;
                    window.webkit.messageHandlers.general.postMessage(null);
                };
                </script>
                ",
            "",
        );
        app.set_webview(&webview);

        let main_thread_app = app.clone();
        let main_thread = thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));

            println!("Setting-up webview for test");
            let max = 100000;
            let receiver = setup_count_bench(webview.clone(), max);

            println!("Starting to send {} messages", max);
            let start = Instant::now();
            let start_webview = webview.clone();
            dispatch::Queue::main().exec_async(move || {
                start_webview.evaluate_javascript("onMessage('1')");
            });
            receiver.recv().unwrap();
            let duration = start.elapsed();
            println!("Finished in {:?} - Sent {:?} messages", duration, max);
            let average_duration: f64 = (duration.as_millis() as f64) / (max as f64);
            println!("Average: {:?}ms", average_duration);

            main_thread_app.stop();
        });

        app.run();
        main_thread.join().unwrap();
    }
}

#[cfg(not(target_os = "macos"))]
fn main() {
    todo!()
}
