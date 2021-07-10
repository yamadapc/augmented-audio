use std::fmt::Debug;

pub trait Updatable {
    type Message: Clone + Debug;
    fn update(&mut self, _message: Self::Message);
}

impl Updatable for () {
    type Message = ();

    fn update(&mut self, _message: Self::Message) {}
}
