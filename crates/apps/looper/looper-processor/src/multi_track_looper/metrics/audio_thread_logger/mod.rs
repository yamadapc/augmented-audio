use std::sync::atomic::{AtomicBool, Ordering};

use basedrop::Shared;

use atomic_queue::Queue;
use audio_garbage_collector::make_shared;

pub struct LogMessage {
    #[allow(unused)]
    message: &'static str,
}

pub struct LoggerHandle {
    #[allow(unused)]
    queue: Queue<LogMessage>,
}

impl LoggerHandle {
    #[allow(unused)]
    pub fn info(&self, message: &'static str) {
        self.queue.push(LogMessage { message });
    }
}

pub struct Logger {
    is_running: Shared<AtomicBool>,
    #[allow(unused)]
    handle: Shared<LoggerHandle>,
}

impl Logger {
    #[allow(unused)]
    pub fn new() -> Self {
        let is_running = make_shared(AtomicBool::new(true));
        let handle = make_shared(LoggerHandle {
            queue: Queue::new(100),
        });
        {
            let is_running = is_running.clone();
            let handle = handle.clone();
            std::thread::spawn(move || {
                while is_running.load(Ordering::Relaxed) {
                    if let Some(message) = handle.queue.pop() {
                        log::info!("{}", message.message);
                    }
                }
            });
        }

        Self { is_running, handle }
    }

    #[allow(unused)]
    pub fn handle(&self) -> &Shared<LoggerHandle> {
        &self.handle
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
    }
}
