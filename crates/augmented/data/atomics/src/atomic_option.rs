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
use std::sync::atomic::{AtomicBool, Ordering};

use crate::AtomicValue;

pub struct AtomicOption<T: AtomicValue + Default> {
    is_present: AtomicBool,
    value: T,
}

impl<T: AtomicValue + Default> AtomicOption<T> {
    pub fn new(value: T) -> Self {
        Self {
            is_present: AtomicBool::new(true),
            value,
        }
    }

    pub fn empty() -> Self {
        Self {
            is_present: AtomicBool::new(false),
            value: Default::default(),
        }
    }

    #[inline]
    pub fn set(&self, value: Option<T::Inner>) {
        if let Some(value) = value {
            self.value.set(value);
            self.is_present.store(true, Ordering::Relaxed);
        } else {
            self.is_present.store(false, Ordering::Relaxed);
        }
    }

    #[inline]
    pub fn inner(&self) -> Option<T::Inner> {
        let is_present = self.is_present.load(Ordering::Relaxed);
        if is_present {
            Some(self.value.get())
        } else {
            None
        }
    }
}

impl<T: AtomicValue + Default + From<T::Inner>> From<Option<T::Inner>> for AtomicOption<T> {
    fn from(value: Option<T::Inner>) -> Self {
        if let Some(value) = value {
            Self::new(value.into())
        } else {
            Self::empty()
        }
    }
}
