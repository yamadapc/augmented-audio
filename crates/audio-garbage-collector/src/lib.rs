use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

use basedrop::{Collector, Handle};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GarbageCollectorError {
    #[error("Failed to acquire lock")]
    LockError,
    #[error("Failed to join the GC thread")]
    JoinError,
}

struct GarbageCollectorState {
    running: bool,
    collect_interval: Duration,
}

/// Wraps `basedrop::Collector` with a polling GC thread.
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
        let collector = basedrop::Collector::new();
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

fn run_collector_loop(
    collector: Arc<Mutex<basedrop::Collector>>,
    state: Arc<Mutex<GarbageCollectorState>>,
) {
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
    fn test_gc_will_run() {
        let _ = wisual_logger::init_from_env();
        let mut gc = GarbageCollector::new(Duration::from_millis(100));

        assert_eq!(gc.blocking_alloc_count(), 0);
        {
            let _s1 = Shared::new(gc.handle(), 10);
            let _s2 = Shared::new(gc.handle(), 10);
            assert_eq!(gc.blocking_alloc_count(), 2);
        }
        std::thread::sleep(Duration::from_millis(200));
        assert_eq!(gc.blocking_alloc_count(), 0);

        gc.stop().unwrap();
    }
}
