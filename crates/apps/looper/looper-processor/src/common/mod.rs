//! This module is odd, but since `Fn` traits are unstable this is required to have
//! a uniform API between rust callbacks & FFI callbacks (`ForeignCallback`).
#[cfg(test)]
use self::closure_consumer::*;

pub trait Consumer<T> {
    fn accept(&self, value: T);
}

impl<T> Consumer<T> for dyn Fn(T) -> () {
    fn accept(&self, value: T) {
        self(value)
    }
}

#[cfg(test)]
mod closure_consumer {
    use std::marker::PhantomData;

    use super::*;

    pub struct ClosureConsumer<F, T> {
        f: F,
        t: PhantomData<T>,
    }

    impl<T, F: Fn(T) -> ()> ClosureConsumer<F, T> {
        pub fn new(f: F) -> Self {
            Self {
                f,
                t: Default::default(),
            }
        }
    }

    impl<T, F: Fn(T) -> ()> Consumer<T> for ClosureConsumer<F, T> {
        fn accept(&self, value: T) {
            (self.f)(value);
        }
    }
}
