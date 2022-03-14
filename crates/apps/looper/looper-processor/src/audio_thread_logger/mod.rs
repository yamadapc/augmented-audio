use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;

use basedrop::Shared;

use atomic_queue::Queue;
use audio_garbage_collector::make_shared;

struct LogMessage {
    message: &'static str,
}

pub struct LoggerHandle {
    queue: Queue<LogMessage>,
}

impl LoggerHandle {
    pub fn info(&self, message: &'static str) {
        self.queue.push(LogMessage { message });
    }
}

pub struct Logger {
    logger_thread: JoinHandle<()>,
    is_running: Shared<AtomicBool>,
    handle: Shared<LoggerHandle>,
}

impl Logger {
    pub fn new() -> Self {
        let is_running = make_shared(AtomicBool::new(true));
        let handle = make_shared(LoggerHandle {
            queue: Queue::new(100),
        });
        let logger_thread = {
            let is_running = is_running.clone();
            let handle = handle.clone();
            std::thread::spawn(move || {
                while is_running.load(Ordering::Relaxed) {
                    if let Some(message) = handle.queue.pop() {
                        log::info!("{}", message.message);
                    }
                }
            })
        };

        Self {
            logger_thread,
            is_running,
            handle,
        }
    }

    pub fn handle(&self) -> &Shared<LoggerHandle> {
        &self.handle
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
    }
}
