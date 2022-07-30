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

//! Batteries included solution to using reference counted values on the audio-thread.
//!
//! Wraps `basedrop` so that smart pointers are dropped on a background thread. Exposes a default
//! global GC thread and helpers to create pointers attached to it.
//!
//! # Collection frequency
//! Collection is based on polling the queue. If references are created and dropped very frequently
//! this will not be adequate.

use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

use basedrop::Collector;
pub use basedrop::{Handle, Owned, Shared, SharedCell};
use lazy_static::lazy_static;
use thiserror::Error;

lazy_static! {
    static ref GARBAGE_COLLECTOR: GarbageCollector = GarbageCollector::default();
}

/// Return a reference to a global GC instance
pub fn current() -> &'static GarbageCollector {
    &GARBAGE_COLLECTOR
}

/// Return a handle to a global GC instance
pub fn handle() -> &'static Handle {
    GARBAGE_COLLECTOR.handle()
}

/// Create a new [`basedrop::SharedCell`] value using the default global [`GarbageCollector`]
/// instance.
pub fn make_shared_cell<T: Send + 'static>(value: T) -> SharedCell<T> {
    SharedCell::new(make_shared(value))
}

/// Create a new [`basedrop::Shared`] value using the default global [`GarbageCollector`]
/// instance.
pub fn make_shared<T: Send + 'static>(value: T) -> Shared<T> {
    Shared::new(handle(), value)
}

/// Errors that may be emitted when stopping the GC
#[derive(Debug, Error)]
pub enum GarbageCollectorError {
    /// Emitted if the GC thread panicked during a GC run
    #[error("Failed to acquire lock")]
    LockError,
    /// Emitted if the GC thread panicked at another point
    #[error("Failed to join the GC thread")]
    JoinError,
}

struct GarbageCollectorState {
    running: bool,
    collect_interval: Duration,
}

/// Wraps [`basedrop::Collector`] with a polling GC thread.
///
/// This drops reference counted variables on a dedicated thread to avoid deallocating from the
/// audio thread.
pub struct GarbageCollector {
    collector: Arc<Mutex<Collector>>,
    state: Arc<Mutex<GarbageCollectorState>>,
    thread: Option<JoinHandle<()>>,
    handle: Handle,
}

impl Default for GarbageCollector {
    fn default() -> Self {
        Self::new(Duration::from_millis(100))
    }
}

impl GarbageCollector {
    /// Create the collector and start the garbage collector thread
    pub fn new(collect_interval: Duration) -> Self {
        let collector = Collector::new();
        let handle = collector.handle();
        let collector = Arc::new(Mutex::new(collector));

        let state = Arc::new(Mutex::new(GarbageCollectorState {
            running: true,
            collect_interval,
        }));

        let thread = {
            let collector = collector.clone();
            let state = state.clone();
            std::thread::Builder::new()
                .name(String::from("gc-thread"))
                .spawn(move || run_collector_loop(collector, state))
                .unwrap()
        };

        GarbageCollector {
            collector,
            thread: Some(thread),
            handle,
            state,
        }
    }

    /// Stop & join the collector thread.
    pub fn stop(&mut self) -> Result<(), GarbageCollectorError> {
        self.state
            .lock()
            .map(|mut state| {
                state.running = false;
            })
            .map_err(|_| GarbageCollectorError::LockError)?;
        if let Some(thread) = self.thread.take() {
            thread
                .join()
                .map_err(|_| GarbageCollectorError::JoinError)?;
        }
        Ok(())
    }

    /// Get a handle to the collector. Does not lock.
    pub fn handle(&self) -> &Handle {
        &self.handle
    }

    /// Force GC on the current thread & return whether it was successful.
    /// Tries to acquire a lock on the collector.
    #[allow(dead_code)]
    pub fn blocking_collect(&self) -> bool {
        self.collector
            .lock()
            .map(|mut collector| {
                collector.collect();
                true
            })
            .unwrap_or(false)
    }

    /// Gets the number of live allocations associated with the `Collector`.
    /// Tries to acquire a lock on the collector.
    #[allow(dead_code)]
    pub fn blocking_alloc_count(&self) -> usize {
        self.collector
            .lock()
            .map(|collector| collector.alloc_count())
            .unwrap_or(0)
    }
}

fn run_collector_loop(collector: Arc<Mutex<Collector>>, state: Arc<Mutex<GarbageCollectorState>>) {
    log::info!("Garbage collector thread started");
    loop {
        let (collect_interval, is_running) = state
            .lock()
            .map(|state| (state.collect_interval, state.running))
            .unwrap_or((Duration::default(), false));
        if !is_running {
            log::info!("Garbage collector thread stopping");
            return;
        }

        let collector = collector.lock().map(|mut collector| {
            collector.collect();
        });
        if collector.is_err() {
            log::warn!("Garbage collector thread failing due to lock error");
            return;
        }

        std::thread::sleep(collect_interval);
    }
}

#[cfg(test)]
mod test {
    use basedrop::*;

    use super::*;

    #[test]
    fn test_create_stop_collector() {
        let _ = wisual_logger::init_from_env();
        let mut gc = GarbageCollector::new(Duration::from_secs(1));
        gc.stop().unwrap();
    }

    #[test]
    fn test_gc_will_run_after_period() {
        let _ = wisual_logger::init_from_env();
        let mut gc = GarbageCollector::new(Duration::from_millis(10));

        assert_eq!(gc.blocking_alloc_count(), 0);
        {
            let _s1 = Shared::new(gc.handle(), 10);
            let _s2 = Shared::new(gc.handle(), 10);
            assert_eq!(gc.blocking_alloc_count(), 2);
        }
        std::thread::sleep(Duration::from_millis(50));
        assert_eq!(gc.blocking_alloc_count(), 0);

        gc.stop().unwrap();
    }

    #[test]
    fn test_gc_will_run_with_blocking_collect() {
        let _ = wisual_logger::init_from_env();
        let mut gc = GarbageCollector::new(Duration::from_millis(100));

        assert_eq!(gc.blocking_alloc_count(), 0);
        {
            let _s1 = Shared::new(gc.handle(), 10);
            let _s2 = Shared::new(gc.handle(), 10);
            assert_eq!(gc.blocking_alloc_count(), 2);
        }
        assert_eq!(gc.blocking_collect(), true);
        assert_eq!(gc.blocking_alloc_count(), 0);

        gc.stop().unwrap();
    }
}
