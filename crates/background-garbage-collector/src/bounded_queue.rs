use std::fmt::{Debug, Formatter, Write};
use std::sync::atomic::{AtomicPtr, Ordering};

use circular_data_structures::CircularVec;

#[derive(Debug, PartialEq)]
enum Node<T> {
    Data(T),
    Head,
    Tail,
    Empty,
}

struct BoundedQueue<T> {
    buffer: CircularVec<AtomicPtr<Node<T>>>,
    head: *mut Node<T>,
    tail: *mut Node<T>,
    empty: *mut Node<T>,
}

impl<T> Debug for BoundedQueue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("BoundedQueue {\n")?;
        f.write_str("  buffer: CircularVec { inner: [\n")?;
        for i in 0..self.buffer.len() {
            f.write_str("    ")?;
            let ptr = self.buffer[i].load(Ordering::Relaxed);
            let entry_str = if ptr == self.head {
                "HEAD"
            } else if ptr == self.tail {
                "TAIL"
            } else if ptr == self.empty {
                "EMPTY"
            } else {
                "DATA"
            };
            f.write_str(entry_str)?;
            f.write_str(",\n")?;
        }
        f.write_str("  ] }\n")?;
        f.write_str("}\n")?;
        Ok(())
    }
}

impl<T: Clone> BoundedQueue<T> {
    fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity + 2);
        let head = Box::into_raw(Box::new(Node::Head));
        let tail = Box::into_raw(Box::new(Node::Tail));
        let empty = Box::into_raw(Box::new(Node::Empty));

        buffer.push(AtomicPtr::new(head));
        buffer.push(AtomicPtr::new(tail));
        for _ in 0..capacity {
            buffer.push(AtomicPtr::new(empty));
        }

        let buffer = CircularVec::with_vec(buffer);
        BoundedQueue {
            buffer,
            head,
            tail,
            empty,
        }
    }

    pub fn len(&self) -> usize {
        let head_index = self.head_index(Ordering::Relaxed);
        for i in 0..(self.buffer.len()) {
            let entry = &self.buffer[head_index + i];
            let value = entry.load(Ordering::Relaxed);
            if let Some(Node::Tail) = unsafe { value.as_ref() } {
                return i - 1;
            }
        }
        return 0;
    }

    pub fn push(&self, value: T) -> Result<(), String> {
        let data_node = Box::into_raw(Box::new(Node::Data(value)));

        for _i in 0..1000 {
            let tail_index = self.tail_index(Ordering::SeqCst);
            let next_entry = &self.buffer[tail_index + 1];
            let next_entry_result =
                next_entry.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |next_entry| {
                    if next_entry != self.empty {
                        None
                    } else {
                        Some(self.tail)
                    }
                });

            if let Ok(_) = next_entry_result {
                if let Ok(_) = self.new_entry_tail_swap(tail_index, data_node) {
                    return Ok(());
                } else {
                    return Err(String::from("Moved tail but not data entry"));
                }
            } else {
                return Err(String::from("The queue is full"));
            }
        }

        Err(String::from("Failed swap"))
    }

    fn new_entry_tail_swap(
        &self,
        index: usize,
        data: *mut Node<T>,
    ) -> Result<*mut Node<T>, *mut Node<T>> {
        self.buffer[index].fetch_update(Ordering::SeqCst, Ordering::SeqCst, |value| {
            if value == self.tail {
                Some(data)
            } else {
                None
            }
        })
    }

    fn head_index(&self, ordering: Ordering) -> usize {
        for i in 0..self.buffer.len() {
            let entry = &self.buffer[i];
            let value = entry.load(ordering);
            if let Some(Node::Head) = unsafe { value.as_ref() } {
                return i;
            }
        }
        return 0;
    }

    fn tail_index(&self, ordering: Ordering) -> usize {
        for i in 0..self.buffer.len() {
            let entry = &self.buffer[i];
            let value = entry.load(ordering);
            if let Some(Node::Tail) = unsafe { value.as_ref() } {
                return i;
            }
        }
        return 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_bounded_queue() {
        let _queue = BoundedQueue::<i32>::new(10);
    }

    #[test]
    fn test_get_empty_queue_len() {
        let queue = BoundedQueue::<i32>::new(10);
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_push_element_to_queue_increments_length() {
        let mut queue = BoundedQueue::<i32>::new(10);
        assert_eq!(queue.len(), 0);
        queue.push(10);
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn test_overflow_will_not_break_things() {
        let mut queue = BoundedQueue::<i32>::new(10);
        assert_eq!(queue.len(), 0);
        for i in 0..50 {
            queue
                .push(10)
                .unwrap_or_else(|err| println!("Error: {} Index: {}", err, i));
        }
        println!("{:?}", queue);
        assert_eq!(queue.len(), 10);
    }
}
