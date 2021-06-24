use std::sync::atomic::{AtomicU32, Ordering};

pub struct Barrier {
    counter: AtomicU32,
}

unsafe impl Send for Barrier {}
unsafe impl Sync for Barrier {}

impl Barrier {
    pub fn new() -> Self {
        Barrier {
            counter: AtomicU32::new(0),
        }
    }

    pub unsafe fn wait(&self) {
        self.counter.fetch_add(1, Ordering::Acquire);
        while self.counter.load(Ordering::Relaxed) != 0 {
            std::hint::spin_loop();
        }
    }

    pub unsafe fn release(&self, expected: u32) {
        while expected != self.counter.load(Ordering::Relaxed) {
            std::hint::spin_loop();
        }
        self.counter.store(0, Ordering::Release);
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_single_barrier_wait_and_release() {
        let has_finished = Arc::new(Mutex::new(false));
        let barrier = Arc::new(Barrier::new());
        let thread1 = {
            let has_finished = has_finished.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || unsafe {
                barrier.wait();
                let mut v = has_finished.lock().unwrap();
                *v = true;
            })
        };
        assert_eq!(*has_finished.lock().unwrap(), false);
        unsafe {
            barrier.release(1);
        }
        std::thread::sleep(Duration::from_millis(10));
        assert_eq!(*has_finished.lock().unwrap(), true);
        thread1.join().unwrap();
    }
}
