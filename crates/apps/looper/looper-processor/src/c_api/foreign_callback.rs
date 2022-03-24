use std::ffi::c_void;
use std::time::{Duration, SystemTime};

#[repr(C)]
pub struct ForeignCallback<T> {
    context: *mut c_void,
    callback: extern "C" fn(*mut c_void, T),
}

unsafe impl<T> Send for ForeignCallback<T> {}

impl<T> ForeignCallback<T> {
    pub fn call(&self, value: T) {
        (self.callback)(self.context, value);
    }
}

#[no_mangle]
pub extern "C" fn get_current_time(id: usize) -> f32 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f32()
}

#[no_mangle]
pub extern "C" fn test_foreign_callback(callback: ForeignCallback<usize>) {
    std::thread::spawn(move || loop {
        loop {
            log::info!("Calling foreign callback");

            callback.call(1);

            std::thread::sleep(Duration::from_secs(1))
        }
    });
}
