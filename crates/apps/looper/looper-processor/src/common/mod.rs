use std::marker::PhantomData;

pub trait Consumer<T> {
    fn accept(&self, value: T);
}

impl<T> Consumer<T> for dyn Fn(T) -> () {
    fn accept(&self, value: T) {
        self(value)
    }
}

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
