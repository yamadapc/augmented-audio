use std::ffi::c_void;
use std::sync::atomic::AtomicUsize;

pub trait NodeTrait<T> {
    fn with_value(value: T) -> Self;
    fn drop_command(&self) -> *mut DropCommand;
    fn value(&self) -> *mut T;
    fn ref_count(&self) -> &AtomicUsize;
}

pub struct Node<T: ?Sized> {
    value: *mut T,
    ref_count: AtomicUsize,
    drop_command: *mut DropCommand,
}

impl<T> Node<T> {
    pub fn new(value: *mut T) -> Self {
        Node {
            value,
            ref_count: AtomicUsize::new(0),
            drop_command: Box::into_raw(Box::new(DropCommand::new(value))),
        }
    }
}

impl<T> NodeTrait<T> for Node<T> {
    fn with_value(value: T) -> Self {
        let value_ptr = Box::into_raw(Box::new(value));
        Self::new(value_ptr)
    }

    fn drop_command(&self) -> *mut DropCommand {
        self.drop_command
    }

    fn value(&self) -> *mut T {
        self.value
    }

    fn ref_count(&self) -> &AtomicUsize {
        &self.ref_count
    }
}

pub struct DropCommand {
    value: *mut c_void,
    drop: unsafe fn(*mut c_void),
}

impl DropCommand {
    pub(crate) fn new<T>(value: *mut T) -> Self {
        DropCommand {
            value: value as *mut c_void,
            drop: drop_value::<T>,
        }
    }

    pub unsafe fn do_drop(&self) {
        (self.drop)(self.value);
    }
}

unsafe fn drop_value<T>(value: *mut c_void) {
    let _ = Box::<T>::from_raw(value as *mut T);
}

#[cfg(test)]
mod test {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    use super::*;

    struct RefCounter {
        count: Arc<AtomicUsize>,
    }

    impl RefCounter {
        fn new(count: Arc<AtomicUsize>) -> Self {
            RefCounter { count }
        }
    }

    impl Drop for RefCounter {
        fn drop(&mut self) {
            self.count.fetch_sub(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn test_drop_command_idea() {
        let count = Arc::new(AtomicUsize::new(1));
        let has_been_dropped = {
            let count = count.clone();
            move || count.load(Ordering::Relaxed) != 1
        };

        let ref_counter = RefCounter::new(count);
        let ref_counter = Box::into_raw(Box::new(ref_counter));
        let ref_counter_drop_command = DropCommand::new(ref_counter);

        assert!(!has_been_dropped(), "Value was dropped before expected");
        unsafe {
            ref_counter_drop_command.do_drop();
        }
        assert!(has_been_dropped(), "Value has been dropped properly");
    }

    #[test]
    fn test_node_create() {
        let count = Arc::new(AtomicUsize::new(1));
        let has_been_dropped = {
            let count = count.clone();
            move || count.load(Ordering::Relaxed) != 1
        };
        let ref_counter = RefCounter::new(count);
        let node = Node::with_value(ref_counter);

        assert!(!has_been_dropped(), "Value was dropped before expected");
        let drop_command = node.drop_command;
        unsafe {
            (*drop_command).do_drop();
        }
        assert!(has_been_dropped(), "Value has been dropped properly");
    }
}
