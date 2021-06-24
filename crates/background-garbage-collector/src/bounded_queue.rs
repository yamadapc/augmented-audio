use std::fmt::{Debug, Formatter};
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

use circular_data_structures::CircularVec;

struct BoundedQueue<T> {
    buffer: CircularVec<AtomicPtr<T>>,
    write_index: AtomicUsize,
    read_index: AtomicUsize,
    head_ptr: *mut T,
    tail_ptr: *mut T,
}

unsafe impl<T> Sync for BoundedQueue<T> {}
unsafe impl<T> Send for BoundedQueue<T> {}

impl<T> Debug for BoundedQueue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_str("BoundedQueue {\n")?;
        let _ = f.write_fmt(format_args!(
            "  write_index: {},\n",
            self.write_index.load(Ordering::Relaxed) % self.buffer.len()
        ));
        let _ = f.write_fmt(format_args!(
            "  read_index: {},\n",
            self.read_index.load(Ordering::Relaxed) % self.buffer.len()
        ));
        let _ = f.write_str("  buffer: CircularVec {\n")?;
        let _ = f.write_str("    inner: [\n");
        for i in 0..self.buffer.len() {
            f.write_str("      ")?;
            let ptr = self.buffer[i].load(Ordering::Relaxed);
            let entry_str = if i == self.read_index.load(Ordering::Relaxed) % self.buffer.len() {
                "HEAD"
            } else if i == self.write_index.load(Ordering::Relaxed) % self.buffer.len() {
                "TAIL"
            } else if ptr == std::ptr::null_mut() {
                "EMPTY"
            } else {
                "DATA"
            };
            f.write_str(entry_str)?;
            f.write_str(",\n")?;
        }
        f.write_str("    ]\n  }\n")?;
        f.write_str("}\n")?;
        Ok(())
    }
}

impl<T: Clone> BoundedQueue<T> {
    fn new(capacity: usize) -> Self {
        let tail_ptr = Box::into_raw(Box::new(0)) as *mut T;
        let head_ptr = Box::into_raw(Box::new(0)) as *mut T;
        let mut buffer = Vec::with_capacity(capacity + 2);
        buffer.push(AtomicPtr::new(head_ptr));
        buffer.push(AtomicPtr::new(tail_ptr));
        for _ in 0..capacity {
            buffer.push(AtomicPtr::default());
        }

        let buffer = CircularVec::with_vec(buffer);

        BoundedQueue {
            buffer,
            write_index: AtomicUsize::new(1),
            read_index: AtomicUsize::new(0),
            tail_ptr,
            head_ptr,
        }
    }

    pub fn len(&self) -> usize {
        let head_index = self.read_index.load(Ordering::Relaxed) % self.buffer.len();
        let tail_index = self.write_index.load(Ordering::Relaxed) % self.buffer.len();
        (tail_index as i32 - head_index as i32 - 1).abs() as usize
    }

    pub fn push(&self, value: *mut T) -> bool {
        // Increment the write index
        let tail_index = self.write_index.fetch_add(1, Ordering::SeqCst);

        // Move the tail one entry forward
        let tail_entry = &self.buffer[tail_index + 1];
        // This can fail if:
        //
        // * There's no more space in the QUEUE
        // * Something else moved the tail forward one more time
        //
        // In case there's no more space left in the QUEUE, this means the 'write_index' is now
        // invalid both the entry & tail swaps will fail until head moves.
        //
        // In case something else moved the tail forward, we can ignore this issue.
        let _ = tail_entry.compare_exchange(
            std::ptr::null_mut(),
            self.tail_ptr,
            Ordering::SeqCst,
            Ordering::SeqCst,
        );
        if self.read_index.load(Ordering::SeqCst) % self.buffer.len()
            == tail_index % self.buffer.len()
        {
            return false;
        }

        // Replace the current tail position with the entry.
        // The write goes forward if the current tail position is empty or if it's occupied by TAIL.
        let entry = &self.buffer[tail_index];
        let entry_insert =
            entry.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |existing_entry| {
                if existing_entry == std::ptr::null_mut() || existing_entry == self.tail_ptr {
                    Some(value)
                } else {
                    None
                }
            });
        return if entry_insert.is_ok() {
            true
        } else {
            log::error!(
                "THE IMPOSSIBLE HAS HAPPENED: Entry failed to insert because insert index is occupied {}",
                tail_index,
            );
            false
        };
    }

    pub fn pop(&self) -> Option<*mut T> {
        let head_index = self.read_index.fetch_add(1, Ordering::SeqCst);
        let pop_index = head_index + 1;
        let pop_entry = &self.buffer[pop_index];
        let result = pop_entry.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |entry| {
            if entry == self.head_ptr || entry == self.tail_ptr || entry == std::ptr::null_mut() {
                None
            } else {
                Some(std::ptr::null_mut())
            }
        });
        let head_entry = &self.buffer[head_index];
        let _ = head_entry.compare_exchange(
            std::ptr::null_mut(),
            self.head_ptr,
            Ordering::SeqCst,
            Ordering::SeqCst,
        );
        result.ok()
    }

    pub fn read_index(&self) -> usize {
        self.read_index.load(Ordering::Relaxed)
    }

    pub fn write_index(&self) -> usize {
        self.write_index.load(Ordering::Relaxed)
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
        let queue = BoundedQueue::<i32>::new(10);
        assert_eq!(queue.read_index(), 0);
        assert_eq!(queue.write_index(), 1);
    }

    #[test]
    fn test_get_empty_queue_len() {
        let queue = BoundedQueue::<i32>::new(10);
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_push_element_to_queue_increments_length() {
        let queue = BoundedQueue::<i32>::new(10);
        assert_eq!(queue.len(), 0);
        let ptr = mock_ptr(1);
        assert!(queue.push(ptr.clone()));
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.read_index(), 0);
        assert_eq!(queue.write_index(), 2);
        let value = queue.pop();
        assert_eq!(value.unwrap(), ptr);
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.read_index(), 1);
        assert_eq!(queue.write_index(), 2);
    }

    #[test]
    fn test_push_pop_push_pop() {
        let queue = BoundedQueue::<i32>::new(10);
        assert_eq!(queue.len(), 0);
        {
            let ptr = mock_ptr(1);
            assert!(queue.push(ptr.clone()));
            assert_eq!(queue.len(), 1);
            assert_eq!(queue.read_index(), 0);
            assert_eq!(queue.write_index(), 2);
            let value = queue.pop();
            assert_eq!(value.unwrap(), ptr);
            assert_eq!(queue.len(), 0);
            assert_eq!(queue.read_index(), 1);
            assert_eq!(queue.write_index(), 2);
        }
        {
            let ptr = mock_ptr(2);
            assert!(queue.push(ptr.clone()));
            assert_eq!(queue.len(), 1);
            assert_eq!(queue.read_index(), 1);
            assert_eq!(queue.write_index(), 3);
            let value = queue.pop();
            assert_eq!(value.unwrap(), ptr);
            assert_eq!(queue.len(), 0);
            assert_eq!(queue.read_index(), 2);
            assert_eq!(queue.write_index(), 3);
        }
    }

    #[test]
    fn test_overflow_will_not_break_things() {
        let queue = BoundedQueue::<i32>::new(3);
        assert_eq!(queue.read_index(), 0);
        assert_eq!(queue.write_index(), 1);
        assert_eq!(queue.len(), 0);

        // ENTRY 1 - HEAD, ENTRY, TAIL, EMPTY, EMPTY
        assert!(queue.push(mock_ptr(1)));
        assert_eq!(queue.read_index(), 0);
        assert_eq!(queue.write_index(), 2);
        assert_eq!(queue.len(), 1);
        println!("{:?}", queue);

        // ENTRY 2 - HEAD, ENTRY, ENTRY, TAIL, EMPTY
        assert!(queue.push(mock_ptr(2)));
        assert_eq!(queue.read_index(), 0);
        assert_eq!(queue.write_index(), 3);
        assert_eq!(queue.len(), 2);
        println!("{:?}", queue);

        // ENTRY 3 - HEAD, ENTRY, ENTRY, ENTRY, TAIL
        assert!(queue.push(mock_ptr(3)));
        assert_eq!(queue.read_index(), 0);
        assert_eq!(queue.write_index(), 4);
        assert_eq!(queue.len(), 3);
        println!("{:?}", queue);

        // ENTRY 4 - WILL still go through
        assert_eq!(queue.len(), 3);
        let result = queue.push(mock_ptr(4));
        println!("{:?}", queue);
        assert!(!result);
        assert_eq!(queue.len(), 4);
    }

    #[test]
    fn test_multithread_push() {
        wisual_logger::init_from_env();

        let queue = Arc::new(BoundedQueue::new(50000));

        let writer_thread_1 = spawn_reader_thread(
            queue.clone(),
            Duration::from_millis((0.0 * rand::random::<f64>()) as u64),
        );
        let writer_thread_2 = spawn_reader_thread(
            queue.clone(),
            Duration::from_millis((0.0 * rand::random::<f64>()) as u64),
        );
        let writer_thread_3 = spawn_reader_thread(
            queue.clone(),
            Duration::from_millis((0.0 * rand::random::<f64>()) as u64),
        );

        writer_thread_1.join().unwrap();
        writer_thread_2.join().unwrap();
        writer_thread_3.join().unwrap();
        assert_eq!(queue.len(), 30000);
    }

    #[test]
    fn test_multithread_push_pop() {
        wisual_logger::init_from_env();

        let queue: Arc<BoundedQueue<i32>> = Arc::new(BoundedQueue::new(30));

        let is_running = Arc::new(Mutex::new(true));
        let output_queue: Arc<BoundedQueue<i32>> = Arc::new(BoundedQueue::new(30));
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
                                log::info!("pop");
                                output_queue.push(value);
                            }
                        }
                    }
                }
                log::info!("Reader thread done reading");
            })
        };

        let writer_thread_1 = spawn_reader_thread(
            queue.clone(),
            Duration::from_millis((0.0 * rand::random::<f64>()) as u64),
        );
        // let writer_thread_2 = spawn_reader_thread(
        //     queue.clone(),
        //     Duration::from_millis((0.0 * rand::random::<f64>()) as u64),
        // );
        // let writer_thread_3 = spawn_reader_thread(
        //     queue.clone(),
        //     Duration::from_millis((0.0 * rand::random::<f64>()) as u64),
        // );

        writer_thread_1.join().unwrap();
        // writer_thread_2.join().unwrap();
        // writer_thread_3.join().unwrap();
        {
            let mut is_running = is_running.lock().unwrap();
            *is_running = false;
        }
        reader_thread.join().unwrap();

        println!("{:?}", queue);
        println!("{:?}", output_queue);
        assert_eq!(queue.len(), 0);
        assert_eq!(output_queue.len(), 30000);
    }

    fn spawn_reader_thread(queue: Arc<BoundedQueue<i32>>, duration: Duration) -> JoinHandle<()> {
        thread::spawn(move || {
            for i in 0..10 {
                let pushed = queue.push(mock_ptr(i));
                if !pushed {
                    log::error!("Failed to write entry {}", i);
                }
                std::thread::sleep(duration);
            }
            log::info!("Thread done writing");
        })
    }
}
