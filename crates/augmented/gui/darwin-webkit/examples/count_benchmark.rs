extern crate cocoa;
extern crate criterion;
extern crate darwin_webkit;

use cocoa::base::id;
use darwin_webkit::helpers::dwk_app::DarwinWKApp;
use darwin_webkit::helpers::dwk_webview::DarwinWKWebView;
use std::sync::mpsc::{channel, Receiver};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

unsafe fn setup_count_bench(webview: Arc<DarwinWKWebView>, n: u64) -> Receiver<()> {
    println!("Starting webview");
    let (sender, receiver) = channel();

    let mut i = 0;
    let message_sender = sender.clone();
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

fn main() {
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
