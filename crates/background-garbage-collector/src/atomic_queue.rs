use std::cmp::max;
use std::sync::atomic::{AtomicI8, AtomicUsize, Ordering};

use circular_data_structures::CircularVec;

enum CellState {
    Empty = 0,
    Storing = 1,
    Stored = 2,
    Loading = 3,
}

impl Into<i8> for CellState {
    fn into(self) -> i8 {
        match self {
            CellState::Empty => 0,
            CellState::Storing => 1,
            CellState::Stored => 2,
            CellState::Loading => 3,
        }
    }
}

/// Atomic queue cloned from
/// https://github.com/max0x7ba/atomic_queue
pub struct Queue<T> {
    head: AtomicUsize,
    tail: AtomicUsize,
    elements: CircularVec<*mut T>,
    states: CircularVec<AtomicI8>,
}

unsafe impl<T> Send for Queue<T> {}
unsafe impl<T> Sync for Queue<T> {}

impl<T> Queue<T> {
    pub fn new(capacity: usize) -> Self {
        let mut elements = CircularVec::with_capacity(capacity);
        elements.resize(capacity, std::ptr::null_mut());
        let mut states = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            states.push(AtomicI8::new(CellState::Empty.into()));
        }
        let states = CircularVec::with_vec(states);
        let head = AtomicUsize::new(0);
        let tail = AtomicUsize::new(0);
        Queue {
            head,
            tail,
            elements,
            states,
        }
    }

    pub fn push(&self, element: *mut T) -> bool {
        let mut head = self.head.load(Ordering::Relaxed);
        loop {
            let length = head as i64 - self.tail.load(Ordering::Relaxed) as i64;
            if length >= self.elements.len() as i64 {
                return false;
            }

            if self
                .head
                .compare_exchange(head, head + 1, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }

            head = self.head.load(Ordering::Relaxed);
        }
        self.do_push(element, head);
        return true;
    }

    pub fn pop(&self) -> Option<*mut T> {
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

    pub fn force_pop(&self) -> *mut T {
        let tail = self.tail.fetch_add(1, Ordering::Acquire);
        self.do_pop(tail)
    }

    fn do_pop(&self, tail: usize) -> *mut T {
        let state = &self.states[tail];
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
                let element = self.elements[tail].clone();
                state.store(CellState::Empty.into(), Ordering::Release);
                return element;
            }
        }
    }

    pub fn force_push(&self, element: *mut T) {
        let head = self.head.fetch_add(1, Ordering::Acquire);
        self.do_push(element, head);
    }

    fn do_push(&self, element: *mut T, head: usize) {
        self.do_push_any(element, head);
    }

    fn do_push_any(&self, element: *mut T, head: usize) {
        let state = &self.states[head];
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
                    let self_ptr = self as *const Self as *mut Self;
                    (*self_ptr).elements[head] = element;
                }
                state.store(CellState::Stored.into(), Ordering::Release);
                return;
            }
        }
    }

    pub fn len(&self) -> usize {
        max(
            self.head.load(Ordering::Relaxed) - self.tail.load(Ordering::Relaxed),
            0,
        )
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::thread::JoinHandle;
    use std::time::Duration;

    use super::*;

    fn mock_ptr(value: i32) -> *mut i32 {
        value as *mut i32
    }

    #[test]
    fn test_create_bounded_queue() {
        let _queue = Queue::<i32>::new(10);
        // assert_eq!(queue.read_index(), 0);
        // assert_eq!(queue.write_index(), 1);
    }

    #[test]
    fn test_get_empty_queue_len() {
        let queue = Queue::<i32>::new(10);
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_push_element_to_queue_increments_length() {
        let queue = Queue::<i32>::new(10);
        assert_eq!(queue.len(), 0);
        let ptr = mock_ptr(1);
        assert!(queue.push(ptr.clone()));
        assert_eq!(queue.len(), 1);
        let value = queue.pop();
        assert_eq!(value.unwrap(), ptr);
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_push_pop_push_pop() {
        let queue = Queue::<i32>::new(10);
        assert_eq!(queue.len(), 0);
        {
            let ptr = mock_ptr(1);
            assert!(queue.push(ptr.clone()));
            assert_eq!(queue.len(), 1);
            let value = queue.pop();
            assert_eq!(value.unwrap(), ptr);
            assert_eq!(queue.len(), 0);
        }
        {
            let ptr = mock_ptr(2);
            assert!(queue.push(ptr.clone()));
            assert_eq!(queue.len(), 1);
            let value = queue.pop();
            assert_eq!(value.unwrap(), ptr);
            assert_eq!(queue.len(), 0);
        }
    }

    #[test]
    fn test_overflow_will_not_break_things() {
        let queue = Queue::<i32>::new(3);
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

        let queue: Arc<Queue<i32>> = Arc::new(Queue::new(size * num_threads / 3));
        let output_queue: Arc<Queue<i32>> = Arc::new(Queue::new(size * num_threads));

        let is_running = Arc::new(Mutex::new(true));
        let reader_thread = {
            let is_running = is_running.clone();
            let queue = queue.clone();
            let output_queue = output_queue.clone();
            thread::spawn(move || {
                while *is_running.lock().unwrap() {
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
                    Duration::from_millis((2.0 * rand::random::<f64>()) as u64),
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
        queue: Arc<Queue<i32>>,
        duration: Duration,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            for i in 0..size {
                loop {
                    let pushed = queue.push(mock_ptr(i as i32));
                    if !pushed {
                        log::error!("Failed to write entry {}", i);
                    } else {
                        break;
                    }
                }
                std::thread::sleep(duration);
            }
            log::info!("Thread done writing");
        })
    }
}
