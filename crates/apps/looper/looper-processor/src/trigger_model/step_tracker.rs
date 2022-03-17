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

    pub fn reset(&mut self) {
        self.last_position_beats = -1.0;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step_tracker() {
        let mut tracker = StepTracker::default();
        let step_length = 0.25;
        let result = tracker.accept(step_length, 0.0);
        assert_eq!(result, Some(0));
        let result = tracker.accept(step_length, 0.1);
        assert_eq!(result, None);
        let result = tracker.accept(step_length, 0.2);
        assert_eq!(result, None);
        let result = tracker.accept(step_length, 0.24);
        assert_eq!(result, None);
        let result = tracker.accept(step_length, 0.25);
        assert_eq!(result, Some(1));
        let result = tracker.accept(step_length, 0.3);
        assert_eq!(result, None);
        let result = tracker.accept(step_length, 0.6);
        assert_eq!(result, Some(2));
    }
}
