use crate::callbacks::CompletedCallback;
use std::thread;
use std::time::Duration;

#[no_mangle]
pub extern "C" fn async_operation(callback: CompletedCallback) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        callback.succeeded()
    });
}
