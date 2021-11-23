// Taken from - https://www.nickwilcox.com/blog/recipe_swift_rust_callback/
use std::ffi::c_void;

#[repr(C)]
pub struct CompletedCallback {
    userdata: *mut c_void,
    callback: extern "C" fn(*mut c_void, bool),
}

unsafe impl Send for CompletedCallback {}

impl CompletedCallback {
    pub fn succeeded(self) {
        (self.callback)(self.userdata, true);
        std::mem::forget(self)
    }

    #[allow(dead_code)]
    pub fn failed(self) {
        (self.callback)(self.userdata, false);
        std::mem::forget(self)
    }
}

impl Drop for CompletedCallback {
    fn drop(&mut self) {
        panic!("CompletedCallback must have explicit succeeded or failed call")
    }
}
