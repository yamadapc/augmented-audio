//! Small helpers for augmented applications.
//!
//! * [`audio_processor_metrics`] exposes structs/functions for tracking CPU usage on the
//!   audio-thread
use std::time::Instant;

pub mod audio_processor_metrics;

/// Log duration of a function
pub fn time<T>(label: &str, body: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let result = body();
    log::info!("{} duration={}ms", label, start.elapsed().as_millis());
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_time() {
        let result = time("test_time", || 10_i32.pow(2));
        assert_eq!(result, 100);
    }
}
