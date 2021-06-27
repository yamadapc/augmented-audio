use crate::atomic_queue;
use crate::node::DropCommand;

pub struct GarbageCollectorOptions {
    queue_capacity: usize,
}

impl GarbageCollectorOptions {
    pub fn new(queue_capacity: usize) -> Self {
        GarbageCollectorOptions { queue_capacity }
    }
}

impl Default for GarbageCollectorOptions {
    fn default() -> Self {
        GarbageCollectorOptions {
            queue_capacity: 500,
        }
    }
}

pub trait GarbageCollectorRef {
    fn enqueue_drop(&self, drop_command: *mut DropCommand);
}

pub struct GarbageCollector {
    queue: atomic_queue::Queue<DropCommand>,
}

impl Default for GarbageCollector {
    fn default() -> Self {
        Self::new(GarbageCollectorOptions::default())
    }
}

impl GarbageCollector {
    pub fn new(options: GarbageCollectorOptions) -> Self {
        GarbageCollector {
            queue: atomic_queue::Queue::new(options.queue_capacity),
        }
    }

    pub fn handle(&self) -> *mut Self {
        self as *const Self as *mut Self
    }
}

impl GarbageCollectorRef for GarbageCollector {
    fn enqueue_drop(&self, drop_command: *mut DropCommand) {
        self.queue.push(drop_command);
    }
}

impl GarbageCollector {
    /// # Safety
    /// Relies on drop commands pushed to its queue being valid.
    pub unsafe fn collect(&self) -> usize {
        let mut values_dropped = 0;
        while let Some(drop_command) = self.queue.pop() {
            (*drop_command).do_drop();
            values_dropped += 1;
        }
        values_dropped
    }
}

#[cfg(test)]
mod test {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    use crate::Shared;

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
    fn test_collect_when_empty() {
        let collector = GarbageCollector::default();
        unsafe {
            collector.collect();
        }
    }

    #[test]
    fn test_collect_with_one_entry() {
        let collector = GarbageCollector::default();
        let count = Arc::new(AtomicUsize::new(1));
        let has_been_dropped = {
            let count = count.clone();
            move || count.load(Ordering::Relaxed) != 1
        };
        let value_ptr = Box::into_raw(Box::new(RefCounter::new(count)));
        let drop_command = Box::into_raw(Box::new(DropCommand::new(value_ptr)));

        assert!(
            !has_been_dropped(),
            "Value has been dropped before collection"
        );
        unsafe {
            collector.collect();
        }
        assert!(
            !has_been_dropped(),
            "Value has been dropped before being added to queue"
        );
        collector.enqueue_drop(drop_command);
        assert!(
            !has_been_dropped(),
            "Value has been dropped before collection"
        );
        unsafe {
            collector.collect();
        }
        assert!(has_been_dropped(), "Value wasn't dropped when expected");
    }

    #[test]
    fn test_collect_list() {
        let collector = GarbageCollector::default();
        let num_entries = 10;
        let count = Arc::new(AtomicUsize::new(num_entries));
        let has_been_dropped = {
            let count = count.clone();
            move || count.load(Ordering::Relaxed) == 0
        };

        let make_command = |count| {
            let value_ptr = Box::into_raw(Box::new(RefCounter::new(count)));
            Box::into_raw(Box::new(DropCommand::new(value_ptr)))
        };

        for _i in 0..num_entries {
            collector.enqueue_drop(make_command(count.clone()));
        }

        assert!(
            !has_been_dropped(),
            "Value has been dropped before collection"
        );
        unsafe {
            collector.collect();
        }
        assert!(has_been_dropped(), "Value wasn't dropped when expected");
    }

    #[test]
    fn test_shared_integration_test() {
        let collector = GarbageCollector::default();
        let count = Arc::new(AtomicUsize::new(1));
        let has_been_dropped = {
            let count = count.clone();
            move || count.load(Ordering::Relaxed) != 1
        };
        let value = Shared::new(collector.handle(), RefCounter::new(count));

        assert!(
            !has_been_dropped(),
            "Value has been dropped before collection"
        );
        unsafe {
            collector.collect();
        }
        assert!(
            !has_been_dropped(),
            "Value has been dropped but variable is still around"
        );
        std::mem::drop(value);
        assert!(
            !has_been_dropped(),
            "Value has been dropped before collection"
        );
        unsafe {
            collector.collect();
        }
        assert!(has_been_dropped(), "Value wasn't dropped when expected");
    }
}
