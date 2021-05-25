use std::time::Duration;

struct InterpolationState {
    /// The start value of this change
    start: f32,
    /// The value we're moving towards
    target: f32,
    /// The increment over current value per tick
    tick_increment: f32,
}

/// Wraps a certain numeric value with linear interpolation.
///
/// Whenever a value change is requested, the change will be smoothed over a time window.
pub struct InterpolatedValue {
    /// Sample rate
    sample_rate: f32,
    /// Duration of interpolation
    smoothing_samples: f32,
    /// Current value
    current_value: f32,
    /// Interpolation parameters if an interpolation is running
    interpolation_state: Option<InterpolationState>,
    /// The duration of interpolation
    smoothing_duration: Duration,
}

impl InterpolatedValue {
    /// Create a new interpolated value
    pub fn new(sample_rate: f32, smoothing_duration: Duration, initial_value: f32) -> Self {
        let smoothing_samples =
            InterpolatedValue::calculate_smoothing_samples(sample_rate, smoothing_duration);

        InterpolatedValue {
            sample_rate,
            smoothing_samples,
            current_value: initial_value,
            interpolation_state: None,
            smoothing_duration,
        }
    }

    /// Modify the sample rate
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.on_parameter_update(false);
    }

    /// Modify the interpolation window
    ///
    /// If a movement is running and duration changes, the transition may continue with the new
    /// speed or be 'restarted'.
    ///
    /// For example, if a 1s transition was mid-way and the duration changes to 200ms, the transition
    /// may continue for 100ms or 200ms, depending on whether reset is set to false/true respectively.
    pub fn set_duration(&mut self, duration: Duration, reset: bool) {
        self.smoothing_duration = duration;
        self.on_parameter_update(reset);
    }

    /// Modify the target value
    pub fn set(&mut self, target: f32) {
        let delta = target - self.current_value;
        let tick_increment = delta / self.smoothing_samples;
        self.interpolation_state = Some(InterpolationState {
            start: self.current_value,
            target,
            tick_increment,
        })
    }

    /// Get the current value and tick the internal state
    pub fn next(&mut self) -> f32 {
        let value = self.get();
        self.tick();
        value
    }

    /// Return the current value
    pub fn get(&self) -> f32 {
        self.current_value
    }

    /// Interpolates current value towards the target value
    pub fn tick(&mut self) {
        if let Some(interpolation_state) = &self.interpolation_state {
            self.current_value += interpolation_state.tick_increment;

            // Reset internal state & don't let the value exceed the target.
            if self.current_value >= interpolation_state.target {
                self.current_value = interpolation_state.target;
                self.interpolation_state = None;
            }
        }
    }
}

impl InterpolatedValue {
    /// Calculates n# of samples in a duration
    fn calculate_smoothing_samples(sample_rate: f32, smoothing_duration: Duration) -> f32 {
        smoothing_duration.as_secs_f32() * sample_rate
    }

    /// Update internal state when sample rate or duration changes.
    fn on_parameter_update(&mut self, reset: bool) {
        self.smoothing_samples = InterpolatedValue::calculate_smoothing_samples(
            self.sample_rate,
            self.smoothing_duration,
        );

        // Reset currently running interpolation
        if reset {
            if let Some(target) = self.interpolation_state.as_ref().map(|s| s.target) {
                self.set(target);
            }
            return;
        }

        // Update currently running interpolation
        if let Some(state) = &mut self.interpolation_state {
            let delta = state.target - state.start;
            let tick_increment = delta / self.smoothing_samples;
            state.tick_increment = tick_increment;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::EPSILON;

    #[test]
    fn test_smoothing_samples() {
        let samples =
            InterpolatedValue::calculate_smoothing_samples(44100.0, Duration::from_secs(1));
        assert_eq!(samples, 44100.0);
    }

    #[test]
    fn test_create_smooth_value() {
        let value = InterpolatedValue::new(44100.0, Duration::from_secs(1), 1234.0);
        assert_eq!(value.get(), 1234.0);
    }

    #[test]
    fn test_interpolation_works() {
        let sample_rate = 44100.0; // 44.1kHz
        let duration = Duration::from_secs(1);
        let initial_value = 0.0;
        let mut value = InterpolatedValue::new(sample_rate, duration, initial_value);
        assert_eq!(value.get(), 0.0);

        // Value starts at 0.0, it'll take 1s (or 44.1k samples) to reach the target
        value.set(100.0);

        // Tick the value 22.05k times, the current value should be 50.
        for _ in 0..22050 {
            value.tick();
        }
        assert_approx_equals(value.get(), 50.0);

        // Tick the value another 22.05k times, the current value should be 100.
        for _ in 0..22050 {
            value.tick();
        }
        assert_approx_equals(value.get(), 100.0);
    }

    #[test]
    fn test_interpolation_will_not_exceed_target() {
        let sample_rate = 44100.0; // 44.1kHz
        let duration = Duration::from_secs(1);
        let initial_value = 0.0;
        let mut value = InterpolatedValue::new(sample_rate, duration, initial_value);
        assert_eq!(value.get(), 0.0);

        // Value starts at 0.0, it'll take 1s (or 44.1k samples) to reach the target
        value.set(100.0);

        for _ in 0..50000 {
            let new_value = value.next();
            assert!(new_value <= 100.0)
        }
        assert_approx_equals(value.get(), 100.0);
    }

    #[test]
    fn test_interpolation_can_be_interrupted() {
        let sample_rate = 44100.0; // 44.1kHz
        let duration = Duration::from_secs(1);
        let initial_value = 0.0;
        let mut value = InterpolatedValue::new(sample_rate, duration, initial_value);
        assert_eq!(value.get(), 0.0);

        // Value starts at 0.0, it'll take 1s (or 44.1k samples) to reach the target
        value.set(100.0);

        // Tick the value 22.05k times, the current value should be 50.
        for _ in 0..22050 {
            value.tick();
        }
        assert_approx_equals(value.get(), 50.0);

        // Go back to 0
        value.set(0.0);

        // Tick the value another 22.05k times, the current value should be 0.
        for _ in 0..22050 {
            value.tick();
        }
        assert_approx_equals(value.get(), 0.0);
    }

    fn assert_approx_equals(value: f32, target: f32) {
        assert!(value - target < EPSILON);
    }
}
