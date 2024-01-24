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
//! [`atomic_queue`] is a port of C++'s [max0x7ba/atomic_queue](https://github.com/max0x7ba/atomic_queue)
//! implementation to rust.
//!
//! This is part of [augmented-audio](https://github.com/yamadapc/augmented-audio).
//!
//! It provides a bounded multi-producer, multi-consumer lock-free queue that is real-time safe.
//!
//! # Usage
//! ```rust
//! let queue: atomic_queue::Queue<usize> = atomic_queue::bounded(10);
//!
//! queue.push(10);
//! if let Some(v) = queue.pop() {
//!     assert_eq!(v, 10);
//! }
//! ```
//!
//! # Safety
//! This queue implementation uses unsafe internally.
//!
//! # Performance
//! When benchmarked on a 2017 i7, this was a lot slower than `ringbuf` (~2x).
//!
//! I'd think this is fine since this queue supporting multiple consumers and
//! multiple producers while `ringbuf` is single producer single consumer.
//!
//! Testing again on a M1 Pro, it is 30% faster.
use std::cell::UnsafeCell;
use std::cmp::max;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicI8, AtomicUsize, Ordering};

#[warn(missing_docs)]

/// State a slot in the Queue's circular buffer can be in.
enum CellState {
    Empty = 0,
    Storing = 1,
    Stored = 2,
    Loading = 3,
}

impl From<CellState> for i8 {
    fn from(value: CellState) -> Self {
        match value {
            CellState::Empty => 0,
            CellState::Storing => 1,
            CellState::Stored => 2,
            CellState::Loading => 3,
        }
    }
}

/// Atomic queue cloned from https://github.com/max0x7ba/atomic_queue
///
/// Should be:
/// * Lock-free
///
/// Any type can be pushed into the queue, but it's recommended to use some sort of smart pointer
/// that can be free-ed outside of the critical path.
///
/// Uses unsafe internally.
pub struct Queue<T> {
    head: AtomicUsize,
    tail: AtomicUsize,
    elements: Vec<UnsafeCell<MaybeUninit<T>>>,
    states: Vec<AtomicI8>,
}

unsafe impl<T: Send> Send for Queue<T> {}
unsafe impl<T: Send> Sync for Queue<T> {}

/// Alias for `Queue::new`, creates a new bounded `MPMC` queue with the given capacity.
///
/// Writes will fail if the queue is full.
pub fn bounded<T>(capacity: usize) -> Queue<T> {
    Queue::new(capacity)
}

impl<T> Queue<T> {
    /// Create a queue with a certain capacity. Writes will fail when the queue is full.
    pub fn new(capacity: usize) -> Self {
        let mut elements = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            elements.push(UnsafeCell::new(MaybeUninit::uninit()));
        }
        let mut states = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            states.push(AtomicI8::new(CellState::Empty.into()));
        }
        let head = AtomicUsize::new(0);
        let tail = AtomicUsize::new(0);
        Queue {
            head,
            tail,
            elements,
            states,
        }
    }

    /// Push an element into the queue and return `true` on success.
    ///
    /// `false` will be returned if the queue is full. If there's contention this operation will
    /// wait until it's able to claim a slot in the queue.
    ///
    /// This is a CAS loop to increment the head of the queue, then another to push this element in.
    pub fn push(&self, element: T) -> bool {
        let mut head = self.head.load(Ordering::Relaxed);
        let elements_len = self.elements.len();
        loop {
            let length = head as i64 - self.tail.load(Ordering::Relaxed) as i64;
            if length >= elements_len as i64 {
                return false;
            }

            if self
                .head
                .compare_exchange(head, head + 1, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                self.do_push(element, head);
                return true;
            }

            head = self.head.load(Ordering::Relaxed);
        }
    }

    /// Pop an element from the queue and return `true` on success.
    ///
    /// `false` will be returned if the queue is empty. If there's contention this operation will
    /// wait until it's able to claim a slot in the queue.
    ///
    /// This is a CAS loop to increment the tail of the queue then another CAS loop to pop the
    /// element at this index out.
    pub fn pop(&self) -> Option<T> {
        let mut tail = self.tail.load(Ordering::Relaxed);
        loop {
            let length = self.head.load(Ordering::Relaxed) as i64 - tail as i64;
            if length <= 0 {
                return None;
            }

            if self
                .tail
                .compare_exchange(tail, tail + 1, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }

            tail = self.tail.load(Ordering::Relaxed);
        }
        Some(self.do_pop(tail))
    }

    /// Pop an element from the queue without checking if it's empty.
    ///
    /// # Safety
    /// There's nothing safe about this.
    pub unsafe fn force_pop(&self) -> T {
        let tail = self.tail.fetch_add(1, Ordering::Acquire);
        self.do_pop(tail)
    }

    /// Push an element into the queue without checking if it's full.
    ///
    /// # Safety
    /// There's nothing safe about this.
    pub unsafe fn force_push(&self, element: T) {
        let head = self.head.fetch_add(1, Ordering::Acquire);
        self.do_push(element, head);
    }

    /// True if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the length of the queue.
    pub fn len(&self) -> usize {
        max(
            self.head.load(Ordering::Relaxed) - self.tail.load(Ordering::Relaxed),
            0,
        )
    }
}

impl<T> Queue<T> {
    fn do_pop(&self, tail: usize) -> T {
        let state = &self.states[tail % self.states.len()];
        loop {
            let expected = CellState::Stored;
            if state
                .compare_exchange(
                    expected.into(),
                    CellState::Loading.into(),
                    Ordering::Acquire,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                let element = unsafe {
                    self.elements[tail % self.elements.len()]
                        .get()
                        .replace(MaybeUninit::uninit())
                        .assume_init()
                };

                state.store(CellState::Empty.into(), Ordering::Release);

                return element;
            }
        }
    }

    fn do_push(&self, element: T, head: usize) {
        self.do_push_any(element, head);
    }

    fn do_push_any(&self, element: T, head: usize) {
        let state = &self.states[head % self.states.len()];
        loop {
            let expected = CellState::Empty;
            if state
                .compare_exchange(
                    expected.into(),
                    CellState::Storing.into(),
                    Ordering::Acquire,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                unsafe {
                    // There's a potential small % optimisation from removing bounds checking here &
                    // using mem::replace.
                    self.elements[head % self.elements.len()]
                        .get()
                        .write(MaybeUninit::new(element));
                }
                state.store(CellState::Stored.into(), Ordering::Release);
                return;
            }
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        if std::mem::needs_drop::<T>() {
            // Could probably be made more efficient by using [std::ptr::drop_in_place()]
            // as the &mut self here guarantees that we are the only remaining user of this Queue
            while let Some(element) = self.pop() {
                drop(element);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::ffi::c_void;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::thread::JoinHandle;
    use std::time::Duration;

    use super::*;

    #[derive(Eq, PartialEq, Debug, Copy, Clone)]
    struct MockPtr(*mut c_void);

    unsafe impl Send for MockPtr {}

    fn mock_ptr(value: i32) -> MockPtr {
        MockPtr(value as *mut c_void)
    }

    #[test]
    fn test_create_bounded_queue() {
        let _queue = Queue::<MockPtr>::new(10);
    }

    #[test]
    fn test_get_empty_queue_len() {
        let queue = Queue::<MockPtr>::new(10);
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_queue_drops_items() {
        struct Item {
            drop_count: Arc<AtomicUsize>,
        }
        impl Drop for Item {
            fn drop(&mut self) {
                self.drop_count.fetch_add(1, Ordering::Relaxed);
            }
        }
        let drop_count = Arc::new(AtomicUsize::new(0));
        let queue: Queue<Item> = Queue::new(10);
        queue.push(Item {
            drop_count: drop_count.clone(),
        });
        queue.push(Item {
            drop_count: drop_count.clone(),
        });
        queue.push(Item {
            drop_count: drop_count.clone(),
        });
        drop(queue);

        assert_eq!(drop_count.load(Ordering::Relaxed), 3);
    }

    #[test]
    fn test_push_element_to_queue_increments_length() {
        let queue = Queue::<MockPtr>::new(10);
        assert_eq!(queue.len(), 0);
        let ptr = mock_ptr(1);
        assert!(queue.push(ptr));
        assert_eq!(queue.len(), 1);
        let value = queue.pop();
        assert_eq!(value.unwrap(), ptr);
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_push_pop_push_pop() {
        let queue = Queue::<MockPtr>::new(10);
        assert_eq!(queue.len(), 0);
        {
            let ptr = mock_ptr(1);
            assert!(queue.push(ptr));
            assert_eq!(queue.len(), 1);
            let value = queue.pop();
            assert_eq!(value.unwrap(), ptr);
            assert_eq!(queue.len(), 0);
        }
        {
            let ptr = mock_ptr(2);
            assert!(queue.push(ptr));
            assert_eq!(queue.len(), 1);
            let value = queue.pop();
            assert_eq!(value.unwrap(), ptr);
            assert_eq!(queue.len(), 0);
        }
    }

    #[test]
    fn test_overflow_will_not_break_things() {
        let queue = Queue::<MockPtr>::new(3);
        assert_eq!(queue.len(), 0);

        // ENTRY 1 - HEAD, ENTRY, TAIL, EMPTY, EMPTY
        assert!(queue.push(mock_ptr(1)));
        assert_eq!(queue.len(), 1);

        // ENTRY 2 - HEAD, ENTRY, ENTRY, TAIL, EMPTY
        assert!(queue.push(mock_ptr(2)));
        assert_eq!(queue.len(), 2);

        // ENTRY 3 - HEAD, ENTRY, ENTRY, ENTRY, TAIL
        assert!(queue.push(mock_ptr(3)));
        assert_eq!(queue.len(), 3);

        // ENTRY 4 - Will fail
        assert_eq!(queue.len(), 3);
        let result = queue.push(mock_ptr(4));
        assert!(!result);
        assert_eq!(queue.len(), 3);
    }

    #[test]
    fn test_multithread_push() {
        wisual_logger::init_from_env();

        let queue = Arc::new(Queue::new(50000));

        let writer_thread_1 = spawn_writer_thread(
            10,
            queue.clone(),
            Duration::from_millis((0.0 * rand::random::<f64>()) as u64),
        );
        let writer_thread_2 = spawn_writer_thread(
            10,
            queue.clone(),
            Duration::from_millis((0.0 * rand::random::<f64>()) as u64),
        );
        let writer_thread_3 = spawn_writer_thread(
            10,
            queue.clone(),
            Duration::from_millis((0.0 * rand::random::<f64>()) as u64),
        );

        writer_thread_1.join().unwrap();
        writer_thread_2.join().unwrap();
        writer_thread_3.join().unwrap();
        assert_eq!(queue.len(), 30);
    }

    #[test]
    fn test_multithread_push_pop() {
        wisual_logger::init_from_env();

        let size = 10000;
        let num_threads = 5;

        let queue: Arc<Queue<MockPtr>> = Arc::new(Queue::new(size * num_threads / 3));
        let output_queue: Arc<Queue<MockPtr>> = Arc::new(Queue::new(size * num_threads));

        let is_running = Arc::new(Mutex::new(true));
        let reader_thread = {
            let is_running = is_running.clone();
            let queue = queue.clone();
            let output_queue = output_queue.clone();
            thread::spawn(move || {
                while *is_running.lock().unwrap() || queue.len() > 0 {
                    loop {
                        match queue.pop() {
                            None => break,
                            Some(value) => {
                                output_queue.push(value);
                            }
                        }
                    }
                }
                log::info!("Reader thread done reading");
            })
        };

        let threads: Vec<JoinHandle<()>> = (0..num_threads)
            .into_iter()
            .map(|_| {
                spawn_writer_thread(
                    size,
                    queue.clone(),
                    Duration::from_millis((rand::random::<f64>()) as u64),
                )
            })
            .collect();

        for thread in threads {
            thread.join().unwrap();
        }

        {
            let mut is_running = is_running.lock().unwrap();
            *is_running = false;
        }
        reader_thread.join().unwrap();

        assert_eq!(queue.len(), 0);
        assert_eq!(output_queue.len(), size * num_threads);
    }

    fn spawn_writer_thread(
        size: usize,
        queue: Arc<Queue<MockPtr>>,
        duration: Duration,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            for i in 0..size {
                loop {
                    let pushed = queue.push(mock_ptr(i as i32));
                    if pushed {
                        break;
                    }
                }
                thread::sleep(duration);
            }
            log::info!("Thread done writing");
        })
    }
}
