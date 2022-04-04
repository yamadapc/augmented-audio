/// Tracks a step sequencer running.
pub struct StepTracker {
    last_position_beats: f64,
}

impl Default for StepTracker {
    fn default() -> Self {
        Self {
            last_position_beats: -1.0,
        }
    }
}

impl StepTracker {
    /// Keeps track of the last position in beats, when a new position is pushed, checks if we've
    /// crossed a step and returns it.
    pub fn accept(&mut self, step_length_beats: f64, position_beats: f64) -> Option<usize> {
        let previous_step = (self.last_position_beats / step_length_beats) as i32;
        let current_step = (position_beats / step_length_beats) as i32;
        self.last_position_beats = position_beats;

        if previous_step != current_step {
            Some(current_step as usize)
        } else {
            None
        }
    }

    /// Reset the sequencer
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.last_position_beats = -1.0;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step_tracker_accept() {
        let mut tracker = StepTracker::default();
        let step_length = 0.25;
        // Step is 0.25 beats (e.g. 1/4 beat will be detected)
        let result = tracker.accept(step_length, 0.0);
        assert_eq!(result, Some(0)); // step 0 is returned
                                     // Nothing is returned in between 0 & 0.25
        let result = tracker.accept(step_length, 0.1);
        assert_eq!(result, None);
        let result = tracker.accept(step_length, 0.2);
        assert_eq!(result, None);
        let result = tracker.accept(step_length, 0.24);
        assert_eq!(result, None);
        // Here we get another step 1 event
        let result = tracker.accept(step_length, 0.25);
        assert_eq!(result, Some(1)); // step 1
                                     // Nothing
        let result = tracker.accept(step_length, 0.3);
        assert_eq!(result, None);
        // Step 3; it doesn't matter how much further we are from the previous call, since we've
        // crossed 0.5 by moving from 0.3 to 0.6, this returns step 2
        let result = tracker.accept(step_length, 0.6);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_step_tracker_reset() {
        let mut tracker = StepTracker::default();
        tracker.accept(0.25, 10.0);
        tracker.reset();
        assert_eq!(tracker.accept(0.25, 0.0), Some(0));
    }
}
