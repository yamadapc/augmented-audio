use std::marker::PhantomData;
use std::sync::atomic::{AtomicPtr, Ordering};

use crate::collector::GarbageCollectorRef;
use crate::node::NodeTrait;

pub struct SharedGeneric<T, Handle: GarbageCollectorRef, Node: NodeTrait<T>> {
    inner: AtomicPtr<Node>,
    handle: *mut Handle,
    phantom: PhantomData<T>,
}

impl<T, Handle: GarbageCollectorRef, Node: NodeTrait<T>> SharedGeneric<T, Handle, Node> {
    pub fn new(handle: *mut Handle, value: T) -> Self {
        let inner = Box::into_raw(Box::new(Node::with_value(value)));
        SharedGeneric {
            inner: AtomicPtr::new(inner),
            handle,
            phantom: PhantomData::default(),
        }
    }
}

impl<T, Handle: GarbageCollectorRef, Node: NodeTrait<T>> Drop for SharedGeneric<T, Handle, Node> {
    fn drop(&mut self) {
        unsafe {
            let inner = self.inner.load(Ordering::Acquire);
            let count = (*inner).ref_count().fetch_sub(1, Ordering::Release);
            if count <= 1 {
                (*self.handle).enqueue_drop((*inner).drop_command());
                let _ = Box::from_raw(inner);
            }
        }
    }
}

impl<T, Handle: GarbageCollectorRef, Node: NodeTrait<T>> Clone for SharedGeneric<T, Handle, Node> {
    fn clone(&self) -> Self {
        unsafe {
            let inner = self.inner.load(Ordering::Acquire);
            let _ = (*inner).ref_count().fetch_add(1, Ordering::Release);
        }

        SharedGeneric {
            inner: AtomicPtr::new(self.inner.load(Ordering::Relaxed)),
            handle: self.handle,
            phantom: self.phantom,
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::atomic::AtomicUsize;
    use std::sync::Arc;

    use atomic_queue;

    use crate::node::DropCommand;

    use super::*;

    struct MockGarbageCollector {
        queue: atomic_queue::Queue<*mut DropCommand>,
    }

    impl Default for MockGarbageCollector {
        fn default() -> Self {
            MockGarbageCollector {
                queue: atomic_queue::Queue::new(10),
            }
        }
    }

    impl GarbageCollectorRef for MockGarbageCollector {
        fn enqueue_drop(&self, drop_command: *mut DropCommand) {
            self.queue.push(drop_command);
        }
    }

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

    struct MockNode<T> {
        value: Option<*mut T>,
        drop_command: *mut DropCommand,
        ref_count: AtomicUsize,
        // Testing counter when this struct is dropped
        drop_counter: Arc<AtomicUsize>,
    }

    impl<T> MockNode<T> {
        fn set_drop_counter(&mut self, counter: Arc<AtomicUsize>) {
            self.drop_counter = counter;
        }

        fn set_drop_command(&mut self, drop_command: *mut DropCommand) {
            self.drop_command = drop_command;
        }
    }

    impl<T> Drop for MockNode<T> {
        fn drop(&mut self) {
            self.drop_counter.fetch_sub(1, Ordering::Relaxed);
        }
    }

    impl<T> NodeTrait<T> for MockNode<T> {
        fn with_value(value: T) -> Self {
            MockNode {
                value: Some(Box::into_raw(Box::new(value))),
                drop_command: mock_ptr(66),
                ref_count: AtomicUsize::new(1),
                drop_counter: Arc::new(AtomicUsize::new(1)),
            }
        }

        fn drop_command(&self) -> *mut DropCommand {
            self.drop_command
        }

        fn value(&self) -> *mut T {
            let value_ref = self.value.unwrap();
            value_ref as *const T as *mut T
        }

        fn ref_count(&self) -> &AtomicUsize {
            &self.ref_count
        }
    }

    #[test]
    fn test_create_shared_does_not_drop_its_contents() {
        let mock_handle = Box::into_raw(Box::new(MockGarbageCollector::default()));
        let count = Arc::new(AtomicUsize::new(1));
        let value = RefCounter::new(count.clone());
        let _shared = SharedGeneric::<RefCounter, MockGarbageCollector, MockNode<RefCounter>>::new(
            mock_handle,
            value,
        );
        assert_eq!(
            count.load(Ordering::Relaxed),
            1,
            "Shared dropped its 'RefCounter' node type"
        );
    }

    #[test]
    fn test_drop_shared_ref() {
        let mock_handle = Box::into_raw(Box::new(MockGarbageCollector::default()));
        let mock_node_count = Arc::new(AtomicUsize::new(1));
        let value_count = Arc::new(AtomicUsize::new(1));
        let value = RefCounter::new(value_count.clone());

        let shared = SharedGeneric::<RefCounter, MockGarbageCollector, MockNode<RefCounter>>::new(
            mock_handle,
            value,
        );
        set_drop_command(&shared, mock_ptr(1));
        {
            let inner = shared.inner.load(Ordering::Relaxed);
            unsafe {
                (*inner).set_drop_counter(mock_node_count.clone());
            }
        }

        let current_count = get_node_ref_count(&shared);
        assert_eq!(
            current_count, 1,
            "Shared ref count wasn't 1 though there's only 1 ref"
        );
        // Assert node was not dropped
        assert_eq!(
            mock_node_count.load(Ordering::Relaxed),
            1,
            "Shared dropped node memory while using it"
        );

        std::mem::drop(shared);
        // Assert node was dropped
        assert_eq!(
            mock_node_count.load(Ordering::Relaxed),
            0,
            "Shared leaked node memory"
        );
        // Assert value was NOT dropped
        assert_eq!(
            value_count.load(Ordering::Relaxed),
            1,
            "Shared dropped the value on the current thread"
        );

        // Drop command should have been pushed into drop queue
        let queue_len = unsafe { (*mock_handle).queue.len() };
        assert_eq!(queue_len, 1, "Drop command was not enqueued");
        // It was our drop command that was pushed
        let drop_command = unsafe { (*mock_handle).queue.pop().unwrap() };
        assert_eq!(drop_command, mock_ptr(1), "Drop command is invalid");
    }

    #[test]
    fn test_clone() {
        let mock_handle = Box::into_raw(Box::new(MockGarbageCollector::default()));
        let count = Arc::new(AtomicUsize::new(1));
        let value = RefCounter::new(count);

        let shared1 = SharedGeneric::<RefCounter, MockGarbageCollector, MockNode<RefCounter>>::new(
            mock_handle,
            value,
        );
        set_drop_command(&shared1, mock_ptr(1));
        let current_count = get_node_ref_count(&shared1);
        assert_eq!(
            current_count, 1,
            "Ref count is not 1, but there's only 1 ref"
        );

        #[allow(clippy::redundant_clone)]
        let shared2 = shared1.clone();
        let current_count = get_node_ref_count(&shared2);
        assert_eq!(current_count, 2, "Ref count is not 2, but there're 2 refs");
    }

    #[test]
    fn test_clone_shared_does_not_drop_its_contents_when_1_ref_is_dropped() {
        let mock_handle = Box::into_raw(Box::new(MockGarbageCollector::default()));
        let count = Arc::new(AtomicUsize::new(1));
        let value = RefCounter::new(count.clone());
        let shared = SharedGeneric::<RefCounter, MockGarbageCollector, MockNode<RefCounter>>::new(
            mock_handle,
            value,
        );
        let _shared2 = shared.clone();
        assert_eq!(
            count.load(Ordering::Relaxed),
            1,
            "contents were dropped but there're still refs"
        );
        std::mem::drop(shared);
        assert_eq!(
            count.load(Ordering::Relaxed),
            1,
            "contents were dropped but there're still refs"
        );
    }

    #[test]
    fn test_clone_shared_drops_when_all_refs_are_gone() {
        let mock_handle = Box::into_raw(Box::new(MockGarbageCollector::default()));
        let count = Arc::new(AtomicUsize::new(1));
        let value = RefCounter::new(count.clone());
        {
            let shared1 =
                SharedGeneric::<RefCounter, MockGarbageCollector, MockNode<RefCounter>>::new(
                    mock_handle,
                    value,
                );
            set_drop_command(&shared1, mock_ptr(2));
            let _shared2 = shared1;
        }
        assert_eq!(
            count.load(Ordering::Relaxed),
            1,
            "contents were dropped on the main thread"
        );

        // Drop command should have been pushed into drop queue
        let queue_len = unsafe { (*mock_handle).queue.len() };
        assert_eq!(queue_len, 1, "Drop command was not enqueued");
        // It was our drop command that was pushed
        let drop_command = unsafe { (*mock_handle).queue.pop().unwrap() };
        assert_eq!(drop_command, mock_ptr(2), "Drop command is invalid");
    }

    fn get_node_ref_count(
        shared: &SharedGeneric<RefCounter, MockGarbageCollector, MockNode<RefCounter>>,
    ) -> usize {
        let inner = shared.inner.load(Ordering::Relaxed);

        unsafe { (*inner).ref_count.load(Ordering::Relaxed) }
    }

    fn set_drop_command(
        shared: &SharedGeneric<RefCounter, MockGarbageCollector, MockNode<RefCounter>>,
        drop_command: *mut DropCommand,
    ) {
        let inner = shared.inner.load(Ordering::Relaxed);
        unsafe {
            (*inner).set_drop_command(drop_command);
        }
    }

    fn mock_ptr<T>(value: usize) -> *mut T {
        value as *mut T
    }
}
