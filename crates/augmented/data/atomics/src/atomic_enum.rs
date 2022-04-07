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
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};

use num_traits::{FromPrimitive, ToPrimitive};

/// Given an enum value deriving `FromPrimitive`/`ToPrimitive`, handles storing the value as an
/// atomic usize.
#[derive(Default, Debug)]
pub struct AtomicEnum<Inner: FromPrimitive + ToPrimitive> {
    value: AtomicUsize,
    inner: PhantomData<Inner>,
}

impl<Inner: FromPrimitive + ToPrimitive> AtomicEnum<Inner> {
    pub fn new(value: Inner) -> Self {
        let value = value.to_usize().unwrap();
        AtomicEnum {
            value: AtomicUsize::new(value),
            inner: PhantomData::default(),
        }
    }

    #[inline]
    pub fn set(&self, value: Inner) {
        let value = value.to_usize().unwrap();
        self.value.store(value, Ordering::Relaxed);
    }

    #[inline]
    pub fn get(&self) -> Inner {
        let value = self.value.load(Ordering::Relaxed);
        Inner::from_usize(value).unwrap()
    }
}

impl<Inner: FromPrimitive + ToPrimitive> From<Inner> for AtomicEnum<Inner> {
    fn from(inner: Inner) -> Self {
        Self::new(inner)
    }
}

#[cfg(test)]
mod test {
    use num_derive::{FromPrimitive, ToPrimitive};

    use super::*;

    #[derive(FromPrimitive, ToPrimitive, Debug, PartialEq)]
    enum TestEnum {
        First,
        Second,
        Third,
    }

    #[test]
    fn test_get_set_enum() {
        let value = TestEnum::First;
        let atomic_enum = AtomicEnum::new(value);
        assert_eq!(atomic_enum.get(), TestEnum::First);
        atomic_enum.set(TestEnum::Second);
        assert_eq!(atomic_enum.get(), TestEnum::Second);
    }
}
