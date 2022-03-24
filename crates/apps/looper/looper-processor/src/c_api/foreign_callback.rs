use std::ffi::c_void;
use std::time::{Duration, SystemTime};

use crate::multi_track_looper::midi_store::MidiEvent;

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
