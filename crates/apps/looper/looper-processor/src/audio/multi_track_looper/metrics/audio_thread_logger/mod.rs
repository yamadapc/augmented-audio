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
use std::sync::atomic::{AtomicBool, Ordering};

use basedrop::Shared;
use lazy_static::lazy_static;

use atomic_queue::Queue;
use audio_garbage_collector::make_shared;

use crate::audio::multi_track_looper::long_backoff::LongBackoff;

lazy_static! {
    static ref AUDIO_THREAD_LOGGER: AudioThreadLogger = AudioThreadLogger::new();
}

#[allow(unused)]
struct LogMessage {
    level: log::Level,
    message: &'static str,
}

#[allow(unused)]
pub struct AudioThreadLoggerHandle {
    queue: Queue<LogMessage>,
}

impl AudioThreadLoggerHandle {
    #[allow(unused)]
    pub fn info(&self, message: &'static str) {
        self.queue.push(LogMessage {
            level: log::Level::Info,
            message,
        });
    }
}

pub struct AudioThreadLogger {
    is_running: Shared<AtomicBool>,
    #[allow(unused)]
    handle: Shared<AudioThreadLoggerHandle>,
}

impl AudioThreadLogger {
    #[allow(unused)]
    fn new() -> Self {
        let is_running = make_shared(AtomicBool::new(true));
        let handle = make_shared(AudioThreadLoggerHandle {
            queue: Queue::new(100),
        });
        {
            let is_running = is_running.clone();
            let handle = handle.clone();
            std::thread::Builder::new()
                .name(String::from("audio_thread_logger"))
                .spawn(move || Self::run(is_running, handle))
                .unwrap();
        }

        Self { is_running, handle }
    }

    #[allow(unused)]
    fn run(is_running: Shared<AtomicBool>, handle: Shared<AudioThreadLoggerHandle>) {
        log::info!("Starting audio-thread-logger");
        let mut long_backoff = LongBackoff::new();
        while is_running.load(Ordering::Relaxed) {
            if let Some(message) = handle.queue.pop() {
                log::log!(message.level, "{}", message.message);
                long_backoff.reset();
            } else {
                long_backoff.snooze();
            }
        }
    }

    #[allow(unused)]
    pub fn handle() -> &'static Shared<AudioThreadLoggerHandle> {
        &AUDIO_THREAD_LOGGER.handle
    }
}

impl Drop for AudioThreadLogger {
    fn drop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_create_drop_logger() {
        wisual_logger::init_from_env();
        let logger = AudioThreadLogger::new();
        std::mem::drop(logger);
    }

    #[test]
    fn test_log_a_message() {
        wisual_logger::init_from_env();
        let logger = AudioThreadLogger::new();
        let handle = logger.handle.clone();
        handle.info("Hello world");
        std::thread::sleep(Duration::from_millis(20));
    }

    #[test]
    fn test_log_a_message_with_global_logger() {
        wisual_logger::init_from_env();
        let logger = AudioThreadLogger::handle();
        logger.info("Global logger");
        std::thread::sleep(Duration::from_millis(20));
    }
}
