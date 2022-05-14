use std::cell::UnsafeCell;
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
use std::ops::{Deref, DerefMut};

use basedrop::{Handle, Shared};

pub struct ProcessorCell<T>(pub UnsafeCell<T>);
unsafe impl<T> Send for ProcessorCell<T> {}
unsafe impl<T> Sync for ProcessorCell<T> {}

/// Hack around rust interior mutability for Shared processor pointers.
/// Processors need to be thread-safe internally.
///
/// We should deprecate this and use the 'handle' pattern instead.
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

#[cfg(test)]
mod test {
    use std::cell::UnsafeCell;

    use audio_processor_traits::NoopAudioProcessor;

    use super::*;

    #[test]
    fn test_create_cell() {
        let _cell = ProcessorCell(UnsafeCell::new(NoopAudioProcessor::<f32>::default()));
    }

    #[test]
    fn test_create_shared_processor() {
        let gc_handle = audio_garbage_collector::handle();
        let processor = NoopAudioProcessor::<f32>::default();
        let mut processor = SharedProcessor::new(gc_handle, processor);
        let _shared = processor.shared();
        let _processor = processor.deref();
        let _processor = processor.deref_mut();
        let _processor = processor.clone();
    }
}
