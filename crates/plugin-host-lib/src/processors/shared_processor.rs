use basedrop::{Handle, Shared};
use std::ops::{Deref, DerefMut};

/// Hack around rust interior mutability for Shared processor pointers.
/// Processors need to be thread-safe internally.
pub struct SharedProcessor<T: Send + 'static> {
    inner: Shared<T>,
}

impl<T: Send + 'static> SharedProcessor<T> {
    pub fn new(handle: &Handle, value: T) -> Self {
        SharedProcessor {
            inner: Shared::new(handle, value),
        }
    }
}

impl<T: Send + 'static> Deref for SharedProcessor<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<T: Send + 'static> DerefMut for SharedProcessor<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut (*(self.inner.deref() as *const Self::Target as *mut Self::Target)) }
    }
}

impl<T: Send + 'static> Clone for SharedProcessor<T> {
    fn clone(&self) -> Self {
        SharedProcessor {
            inner: self.inner.clone(),
        }
    }
}
