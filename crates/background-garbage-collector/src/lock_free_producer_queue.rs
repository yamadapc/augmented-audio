// Lock-free talk part 2
// https://www.youtube.com/watch?v=1obZeHnAwz4

#[cfg(test)]
mod test {
    use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

    type Value = u32;

    struct Queue {
        storage: Vec<AtomicPtr<Value>>,
        head: AtomicUsize,
        tail: AtomicUsize,
        len: AtomicUsize,
    }

    impl Queue {
        fn new() -> Self {
            let mut storage = Vec::with_capacity(10);
            for _ in 0..12 {
                storage.push(AtomicPtr::default());
            }
            Queue {
                storage,
                head: AtomicUsize::new(0),
                tail: AtomicUsize::new(0),
                len: AtomicUsize::new(0),
            }
        }

        fn push(&mut self, value: *mut Value) {
            // let current_index = self.tail.load(Ordering::Acquire);
            let current_index = self.tail.fetch_add(1, Ordering::Acquire);
            self.len.fetch_add(1, Ordering::Relaxed);
            self.storage[(current_index + 1) % 12].store(std::ptr::null_mut(), Ordering::Relaxed);
            self.storage[current_index % 12].store(value, Ordering::Release);

            // if let Ok(_) = self.tail.compare_exchange(
            //     current_index,
            //     current_index + 1,
            //     Ordering::Release,
            //     Ordering::Relaxed,
            // ) {
            //     return;
            // }
        }

        fn pop(&self) -> Option<*mut Value> {
            let head = self.head.fetch_add(1, Ordering::Acquire);
            let value = self.storage[head % 12].load(Ordering::Relaxed);
            if value != std::ptr::null_mut() {
                self.storage[head % 12].store(std::ptr::null_mut(), Ordering::Relaxed);
                Some(value)
            } else {
                None
            }
        }

        fn to_vec(&self) -> Vec<*mut Value> {
            let mut v = vec![];
            while let Some(value) = self.pop() {
                v.push(value);
            }
            v
        }
    }

    // Race conditions:
    // 1. Increment
    // 2. Read/write
    // 3. Write to the same record multiple times
    // 4. Consume unwritten record

    // * Atomic size 'counter'

    // Producers compete for the same slots
    // Consumers compete for the same slots

    // Producer:
    // * CAS - compare and swap
    //   - Update if current value matches the "old value"
    //   - Will loop in general (so if current value is changed try again) ; rust's `fetch_update`
    //   - Steps
    //     * Read current value
    //     * Compute
    //     * Try to atomically swap
    //     * If that fails, try again

    // Queue producer with compare and swap:
    //
    //     atomic<usize> n;
    //     usize old_n; record R;
    //     do {
    //       old_n = n;
    //       buildRecord(R, records[0]...);
    //     } while (cas(old_n, old_n + 1) != old_n)
    //

    fn into_raw<T>(v: T) -> *mut T {
        Box::into_raw(Box::new(v))
    }

    #[test]
    fn test_queue_push() {
        let mut q = Queue::new();
        let ptr1 = into_raw(10);
        let ptr2 = into_raw(10);
        let ptr3 = into_raw(10);
        q.push(ptr1);
        q.push(ptr2);
        q.push(ptr3);
        let v = q.to_vec();
        assert_eq!(v, vec![ptr1, ptr2, ptr3]);
    }

    #[test]
    fn test_queue_push_overflow() {
        let mut q = Queue::new();
        let ptr1 = into_raw(10);
        {
            assert_eq!(q.head.load(Ordering::Relaxed), 0);
            assert_eq!(q.tail.load(Ordering::Relaxed), 0);
            q.push(ptr1);
            assert_eq!(q.head.load(Ordering::Relaxed), 0);
            assert_eq!(q.tail.load(Ordering::Relaxed), 1);
            q.push(ptr1);
            assert_eq!(q.head.load(Ordering::Relaxed), 0);
            assert_eq!(q.tail.load(Ordering::Relaxed), 2);
            q.push(ptr1);
            assert_eq!(q.head.load(Ordering::Relaxed), 0);
            assert_eq!(q.tail.load(Ordering::Relaxed), 3);
            q.push(ptr1);
            assert_eq!(q.head.load(Ordering::Relaxed), 0);
            assert_eq!(q.tail.load(Ordering::Relaxed), 4);
            q.push(ptr1);
            q.push(ptr1);
            q.push(ptr1);
            q.push(ptr1);
            q.push(ptr1);
            assert_eq!(q.head.load(Ordering::Relaxed), 0);
            assert_eq!(q.tail.load(Ordering::Relaxed), 9);
            q.push(ptr1);
            assert_eq!(q.head.load(Ordering::Relaxed), 0);
            assert_eq!(q.tail.load(Ordering::Relaxed), 10);

            let mut v = Vec::new();
            v.push(q.pop().unwrap());
            assert_eq!(q.head.load(Ordering::Relaxed), 1);
            assert_eq!(q.tail.load(Ordering::Relaxed), 10);
            v.push(q.pop().unwrap());
            assert_eq!(q.head.load(Ordering::Relaxed), 2);
            assert_eq!(q.tail.load(Ordering::Relaxed), 10);
            for _ in 0..8 {
                assert!(q.head.load(Ordering::Relaxed) < q.tail.load(Ordering::Relaxed));
                v.push(q.pop().unwrap())
            }
            assert_eq!(q.head.load(Ordering::Relaxed), 10);
            assert_eq!(q.tail.load(Ordering::Relaxed), 10);

            assert_eq!(
                v,
                vec![ptr1, ptr1, ptr1, ptr1, ptr1, ptr1, ptr1, ptr1, ptr1, ptr1]
            );
        }

        {
            assert_eq!(q.head.load(Ordering::Relaxed), 10);
            assert_eq!(q.tail.load(Ordering::Relaxed), 10);
            q.push(ptr1);
            q.push(ptr1);
            q.push(ptr1);
            q.push(ptr1);
            q.push(ptr1);
            let v = q.to_vec();
            assert_eq!(v, vec![ptr1, ptr1, ptr1, ptr1, ptr1]);
        }
    }
}
