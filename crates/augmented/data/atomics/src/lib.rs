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
use std::sync::atomic::*;

use num_traits::{FromPrimitive, ToPrimitive};
use serde_derive::{Deserialize, Serialize};

pub use atomic_enum::AtomicEnum;
pub use atomic_option::AtomicOption;

mod atomic_enum;
mod atomic_option;

pub trait AtomicValue {
    type Inner;

    fn get(&self) -> Self::Inner;
    fn set(&self, value: Self::Inner);
}

macro_rules! std_atomic_impl {
    ($x: path, $inner: ident) => {
        impl AtomicValue for $x {
            type Inner = $inner;

            #[inline]
            fn get(&self) -> Self::Inner {
                self.load(Ordering::Relaxed)
            }

            #[inline]
            fn set(&self, value: Self::Inner) {
                self.store(value, Ordering::Relaxed)
            }
        }
    };
}

std_atomic_impl!(AtomicU8, u8);
std_atomic_impl!(AtomicU16, u16);
std_atomic_impl!(AtomicU32, u32);
std_atomic_impl!(AtomicU64, u64);
std_atomic_impl!(AtomicUsize, usize);
std_atomic_impl!(AtomicI8, i8);
std_atomic_impl!(AtomicI16, i16);
std_atomic_impl!(AtomicI32, i32);
std_atomic_impl!(AtomicI64, i64);
std_atomic_impl!(AtomicBool, bool);

macro_rules! atomic_float {
    ($name: ident, $backing: ident, $inner: ident) => {
        /// Simple atomic floating point variable with relaxed ordering.
        ///
        /// Fork of atomic float from rust-vst.
        #[derive(Serialize, Deserialize)]
        pub struct $name {
            atomic: $backing,
        }

        impl $name {
            /// New atomic float with initial value `value`.
            #[inline]
            pub fn new(value: $inner) -> Self {
                Self {
                    atomic: $backing::new(value.to_bits()),
                }
            }

            /// Get the current value of the atomic float with relaxed ordering.
            #[inline]
            pub fn get(&self) -> $inner {
                $inner::from_bits(self.atomic.load(Ordering::Relaxed))
            }

            /// Set the value of the atomic float to `value` with relaxed ordering.
            #[inline]
            pub fn set(&self, value: $inner) {
                self.atomic.store(value.to_bits(), Ordering::Relaxed)
            }

            /// Get the current value of the atomic float with `ordering`.
            #[inline]
            pub fn load(&self, ordering: Ordering) -> $inner {
                $inner::from_bits(self.atomic.load(ordering))
            }

            /// Set the value of the atomic float to `value` with `ordering`.
            #[inline]
            pub fn store(&self, value: $inner, ordering: Ordering) {
                self.atomic.store(value.to_bits(), ordering)
            }
        }

        impl Default for $name {
            #[inline]
            fn default() -> Self {
                Self::new(0.0)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(&self.get(), f)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.get(), f)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, rhs: &Self) -> bool {
                self.get() == rhs.get()
            }
        }

        impl Clone for $name {
            #[inline]
            fn clone(&self) -> Self {
                $name::from(self.get())
            }
        }

        impl From<$inner> for $name {
            #[inline]
            fn from(value: $inner) -> Self {
                $name::new(value)
            }
        }

        impl From<$name> for $inner {
            #[inline]
            fn from(value: $name) -> Self {
                value.get()
            }
        }

        impl AtomicValue for $name {
            type Inner = $inner;

            #[inline]
            fn get(&self) -> Self::Inner {
                $name::get(self)
            }

            #[inline]
            fn set(&self, value: Self::Inner) {
                $name::set(self, value)
            }
        }
    };
}

atomic_float!(AtomicF32, AtomicU32, f32);
atomic_float!(AtomicF64, AtomicU64, f64);

impl<T: ToPrimitive + FromPrimitive> AtomicValue for AtomicEnum<T> {
    type Inner = T;

    #[inline]
    fn get(&self) -> Self::Inner {
        Self::get(self)
    }

    #[inline]
    fn set(&self, value: Self::Inner) {
        Self::set(self, value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_f32_doesnt_lose_precision() {
        let value = 9.97732426_f32;
        let a_value = AtomicF32::new(value);
        assert!((a_value.get() - value) < f32::EPSILON);
    }
}
