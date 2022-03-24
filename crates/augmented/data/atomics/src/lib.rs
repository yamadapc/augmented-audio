use std::sync::atomic::*;

use num_traits::{FromPrimitive, ToPrimitive};

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

            fn get(&self) -> Self::Inner {
                self.load(Ordering::Relaxed)
            }

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

macro_rules! atomic_float {
    ($name: ident, $backing: ident, $inner: ident) => {
        /// Simple atomic floating point variable with relaxed ordering.
        ///
        /// Fork of atomic float from rust-vst.
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

            fn get(&self) -> Self::Inner {
                $name::get(self)
            }

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

    fn get(&self) -> Self::Inner {
        Self::get(self)
    }

    fn set(&self, value: Self::Inner) {
        Self::set(self, value)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_sample() {}
}
