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

use crate::audio::multi_track_looper::long_backoff::LongBackoff;
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
            std::thread::Builder::new()
                .name(String::from("audio_thread_logger"))
                .spawn(move || {
                    let mut long_backoff = LongBackoff::new();
                    while is_running.load(Ordering::Relaxed) {
                        if let Some(message) = handle.queue.pop() {
                            log::info!("{}", message.message);
                        } else {
                            long_backoff.snooze();
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
