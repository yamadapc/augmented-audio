use std::any::Any;
use std::sync::atomic::{AtomicPtr, Ordering};

struct QueueNode<T> {
    head: T,
    tail: AtomicPtr<QueueNode<T>>,
}

impl<T> QueueNode<T> {
    fn new(head: T) -> QueueNode<T> {
        QueueNode {
            head,
            tail: AtomicPtr::default(),
        }
    }

    fn head(&self) -> &T {
        &self.head
    }
}

struct Queue<T> {
    head: AtomicPtr<QueueNode<T>>,
    tail: AtomicPtr<QueueNode<T>>,
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue {
            head: AtomicPtr::default(),
            tail: AtomicPtr::default(),
        }
    }

    fn push(&self, elem_ptr: T) {
        let elem_node = Box::into_raw(Box::new(QueueNode::new(elem_ptr)));
        let result = self.tail.compare_exchange(
            std::ptr::null_mut(),
            elem_node,
            Ordering::SeqCst,
            Ordering::SeqCst,
        );

        if result.is_ok() {
            let _ = self.head.compare_exchange(
                std::ptr::null_mut(),
                elem_node,
                Ordering::SeqCst,
                Ordering::SeqCst,
            );
            return;
        }
        let mut current_queue_ptr = result.err().unwrap();
        loop {
            // issue ; node being changed here may have been popped at this time
            let result = unsafe {
                (*current_queue_ptr).tail.compare_exchange(
                    std::ptr::null_mut(),
                    elem_node,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                )
            };

            match result {
                Ok(_) => break,
                Err(current_tail) => {
                    current_queue_ptr = current_tail;
                }
            }
        }
        // TODO - ?
        let _ = self.tail.compare_exchange(
            current_queue_ptr,
            elem_node,
            Ordering::SeqCst,
            Ordering::SeqCst,
        );
    }

    fn pop(&self) -> Option<*mut QueueNode<T>> {
        let mut current_queue_ptr = self.head.load(Ordering::SeqCst);
        loop {
            if current_queue_ptr == std::ptr::null_mut() {
                return None;
            }

            let result = self.head.compare_exchange(
                current_queue_ptr,
                unsafe { (*current_queue_ptr).tail.load(Ordering::SeqCst) },
                Ordering::SeqCst,
                Ordering::SeqCst,
            );

            match result {
                Err(other_head) => {
                    current_queue_ptr = other_head;
                }
                Ok(queue_ptr) => return Some(queue_ptr),
            }
        }
    }

    fn len(&self) -> usize {
        let mut current_queue_ptr = self.head.load(Ordering::SeqCst);
        let mut size = 0;
        while current_queue_ptr != std::ptr::null_mut() {
            size += 1;
            unsafe { current_queue_ptr = (*current_queue_ptr).tail.load(Ordering::SeqCst) }
        }
        size
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::thread::JoinHandle;
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_push_into_queue() {
        let collector = Queue::new();
        collector.push(Box::new(10));
        assert_eq!(collector.len(), 1);
        collector.push(Box::new(10));
        collector.push(Box::new(10));
        assert_eq!(collector.len(), 3);
    }

    #[test]
    fn test_pop_1_from_the_queue() {
        let queue: Queue<Box<dyn Any>> = Queue::new();
        queue.push(Box::new(10));
        assert_eq!(queue.len(), 1);
        let node = queue.pop();
        assert_eq!(queue.len(), 0);
        assert_eq!(node.is_some(), true);
        let node = node.unwrap();
        let any_box = unsafe { (*node).head() };
        let value: Option<&i32> = any_box.downcast_ref::<i32>();
        assert_eq!(*value.unwrap(), 10);
    }

    #[test]
    fn test_push_and_pop() {
        let queue: Queue<Box<dyn Any>> = Queue::new();
        queue.push(Box::new(10));
        queue.push(Box::new(11));
        queue.push(Box::new(12));
        assert_eq!(queue.len(), 3);
        let node = queue.pop();
        assert_node_value(node, 10);
        assert_eq!(queue.len(), 2);
        let node = queue.pop();
        assert_node_value(node, 11);
        assert_eq!(queue.len(), 1);
        let node = queue.pop();
        assert_node_value(node, 12);
        assert_eq!(queue.len(), 0);
        let node = queue.pop();
        assert_eq!(node.is_none(), true);
    }

    #[test]
    fn test_multithread_push() {
        wisual_logger::init_from_env();

        let queue = Arc::new(Queue::new());

        let writer_thread_1 = spawn_reader_thread(
            queue.clone(),
            Duration::from_millis((3.0 * rand::random::<f64>()) as u64),
        );
        let writer_thread_2 = spawn_reader_thread(
            queue.clone(),
            Duration::from_millis((2.0 * rand::random::<f64>()) as u64),
        );
        let writer_thread_3 = spawn_reader_thread(
            queue.clone(),
            Duration::from_millis((1.0 * rand::random::<f64>()) as u64),
        );

        writer_thread_1.join().unwrap();
        writer_thread_2.join().unwrap();
        writer_thread_3.join().unwrap();
        assert_eq!(queue.len(), 30000);
    }

    #[test]
    fn test_multithread_push_pop() {
        wisual_logger::init_from_env();

        let queue = Arc::new(Queue::new());

        let is_running = Arc::new(Mutex::new(true));
        let output_queue: Arc<Queue<*mut i32>> = Arc::new(Queue::new());
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
                                let value = unsafe { (*value).head() };
                                output_queue.push(*value);
                            }
                        }
                    }
                }
                log::info!("Reader thread done reading");
            })
        };

        let writer_thread_1 = spawn_reader_thread(
            queue.clone(),
            Duration::from_millis((3.0 * rand::random::<f64>()) as u64),
        );
        let writer_thread_2 = spawn_reader_thread(
            queue.clone(),
            Duration::from_millis((2.0 * rand::random::<f64>()) as u64),
        );
        let writer_thread_3 = spawn_reader_thread(
            queue.clone(),
            Duration::from_millis((1.0 * rand::random::<f64>()) as u64),
        );

        writer_thread_1.join().unwrap();
        writer_thread_2.join().unwrap();
        writer_thread_3.join().unwrap();
        std::thread::sleep(Duration::from_millis(10000));
        {
            let mut is_running = is_running.lock().unwrap();
            *is_running = false;
        }
        reader_thread.join().unwrap();

        assert_eq!(queue.len(), 0);
        assert_eq!(output_queue.len(), 30000);
    }

    fn spawn_reader_thread(queue: Arc<Queue<*mut i32>>, duration: Duration) -> JoinHandle<()> {
        thread::spawn(move || {
            for i in 0..10000 {
                queue.push(Box::into_raw(Box::new(i)));
                std::thread::sleep(duration);
            }
            log::info!("Thread 1 done writing");
        })
    }

    fn assert_node_value<T: 'static + PartialEq + Debug>(
        node: Option<*mut QueueNode<Box<dyn Any>>>,
        expected_value: T,
    ) {
        assert_eq!(node.is_some(), true);
        let node = node.unwrap();
        let any_box = unsafe { (*node).head() };
        let value: Option<&T> = any_box.downcast_ref::<T>();
        assert_eq!(*value.unwrap(), expected_value);
    }
}
