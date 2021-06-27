use std::ops::{Deref, DerefMut};

use basedrop::{Handle, Shared};
use std::cell::UnsafeCell;

pub struct ProcessorCell<T>(pub UnsafeCell<T>);
unsafe impl<T> Send for ProcessorCell<T> {}
unsafe impl<T> Sync for ProcessorCell<T> {}

/// Hack around rust interior mutability for Shared processor pointers.
/// Processors need to be thread-safe internally.
pub struct SharedProcessor<T: Send + 'static> {
    inner: Shared<ProcessorCell<T>>,
}

unsafe impl<T: Send + 'static> Send for SharedProcessor<T> {}
unsafe impl<T: Send + 'static> Sync for SharedProcessor<T> {}

impl<T: Send + 'static> SharedProcessor<T> {
    pub fn new(handle: &Handle, value: T) -> Self {
        SharedProcessor {
            inner: Shared::new(handle, ProcessorCell(UnsafeCell::new(value))),
        }
    }

    pub fn shared(&self) -> Shared<ProcessorCell<T>> {
        self.inner.clone()
    }
}

impl<T: Send + 'static> Deref for SharedProcessor<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.inner.deref().0.get()) }
    }
}

impl<T: Send + 'static> DerefMut for SharedProcessor<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut (*(self.inner.deref()).0.get()) }
    }
}

impl<T: Send + 'static> Clone for SharedProcessor<T> {
    fn clone(&self) -> Self {
        SharedProcessor {
            inner: self.inner.clone(),
        }
    }
}
