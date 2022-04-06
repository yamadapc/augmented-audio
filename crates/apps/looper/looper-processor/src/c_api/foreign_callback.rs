use std::ffi::c_void;

use crate::common::Consumer;

#[repr(C)]
pub struct ForeignCallback<T> {
    pub context: *mut c_void,
    pub callback: extern "C" fn(*mut c_void, T),
}

unsafe impl<T> Send for ForeignCallback<T> {}

impl<T> ForeignCallback<T> {
    pub fn call(&self, value: T) {
        (self.callback)(self.context, value);
    }
}

impl<T> Consumer<T> for ForeignCallback<T> {
    fn accept(&self, value: T) {
        self.call(value);
    }
}

#[cfg(test)]
mod test {
    use std::ffi::c_void;
    use std::sync::atomic::AtomicUsize;
    use std::sync::Arc;

    use augmented_atomics::AtomicValue;

    use crate::ForeignCallback;

    extern "C" fn closure_forwarder(context: *mut c_void, value: usize) {
        let context: &mut &mut dyn Fn(usize) -> () = unsafe { std::mem::transmute(context) };
        context(value);
    }

    /// This test passes a rust closure into ForeignCallback, as if it was a C ffi compliant
    /// function it then tests that this closure is properly called.
    #[test]
    pub fn test_foreign_callback() {
        let holder = Arc::new(AtomicUsize::new(0));
        let closure = {
            // This leaks memory as `holder` will never get released, foreign callback has no way
            // of dropping its context (at least not for real-world FFI where there's a foreign
            // language)
            let holder = holder.clone();
            move |value| holder.set(value)
        };
        let context: Box<Box<dyn Fn(usize) -> ()>> = Box::new(Box::new(closure));
        let context = Box::into_raw(context) as *mut c_void;
        let foreign_callback = ForeignCallback {
            context,
            callback: closure_forwarder,
        };
        foreign_callback.call(10);
        assert_eq!(holder.get(), 10);
    }
}
