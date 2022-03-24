use std::ffi::c_void;

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
