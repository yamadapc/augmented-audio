use std::time::Duration;

/// Similar to `crossbeam::utils::Backoff`, but for infrequent producers. Sleeps at exponentially
/// increasing timeouts when no elements appear. `LongBackoff::snooze` should be called **if there
/// was no work on this iteration**.
pub struct LongBackoff {
    backoff: crossbeam::utils::Backoff,
    iterations: usize,
}

impl LongBackoff {
    pub fn new() -> Self {
        Self {
            backoff: crossbeam::utils::Backoff::new(),
            iterations: 0,
        }
    }

    /// Snoozes with backoff for the first 100 iterations.
    ///
    /// Then starts sleeping from 1ms to 127ms with delays growing exponentially.
    ///
    /// Sleeps for at most 127ms
    pub fn snooze(&mut self) {
        if self.iterations < 100 {
            self.iterations += 1;
            self.backoff.snooze();
        } else {
            self.iterations += 1;
            let iteration = (self.iterations - 100).min(7); // 0-7
            let sleep_time = 2_u64.pow(iteration as u32); // 2^i
            std::thread::sleep(Duration::from_millis(sleep_time));
        }
    }
}
