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
//! This module is odd, but since `Fn` traits are unstable this is required to have
//! a uniform API between rust callbacks & FFI callbacks (`ForeignCallback`).
#[cfg(test)]
pub use self::closure_consumer::*;

pub trait Consumer<T> {
    fn accept(&self, value: T);
}

impl<T> Consumer<T> for dyn Fn(T) {
    fn accept(&self, value: T) {
        self(value)
    }
}

#[cfg(test)]
mod test {
    use std::sync::mpsc::channel;

    use super::*;

    #[test]
    fn test_fn_consumer() {
        let (tx, rx) = channel();

        fn sample_consumer(tx: std::sync::mpsc::Sender<bool>) {
            tx.send(true).unwrap();
        }

        let consumer: Box<dyn Fn(std::sync::mpsc::Sender<bool>)> = Box::new(sample_consumer);
        consumer.accept(tx);
        assert!(rx.recv().unwrap());
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

    impl<T, F: Fn(T)> ClosureConsumer<F, T> {
        pub fn new(f: F) -> Self {
            Self {
                f,
                t: Default::default(),
            }
        }
    }

    impl<T, F: Fn(T)> Consumer<T> for ClosureConsumer<F, T> {
        fn accept(&self, value: T) {
            (self.f)(value);
        }
    }
}
