use std::time::{Duration, SystemTime};

/// Time the button must be held down in order to trigger a hold event
const HOLD_THRESHOLD_SECS: u64 = 2;
/// Maximum time between taps for the button to trigger a double-tap event
const DOUBLE_TAP_THRESHOLD_MS: u128 = 500;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum MIDIButtonEvent {
    ButtonDown,
    ButtonUp,
    DoubleTap,
    Hold,
}

/// Implements a button with double tap and hold detection
///
/// There are two methods in the API:
///
/// * `MIDIButton::accept` should be called with MIDI `u8` value for this button, whenever an event
///   is fired
/// * `MIDIButton::tick` should be constantly called on every frame
///
/// Both methods return `Option<MIDIButtonEvent>`.
///
/// Button-down is considered any value above 0.5 * 127, so an expression pedal would be considered
/// down past half-way.
///
/// By default `MIDIButton` will forward `ButtonDown` and `ButtonUp` immediately when they happen.
///
/// This means a "double tap" interaction will first trigger down/up events for the first tap and
/// then trigger double-tap. An alternative is to wait until the threshold has passed before firing
/// down/up events. This is not implemented. Immediate forwarding works better for looper buttons.
///
/// If the button is pressed twice within `DOUBLE_TAP_THRESHOLD_MS` a double-tap event will fire.
///
/// `MIDIButton::tick` checks if the button has been held down for `HOLD_THRESHOLD_SECS` and fires a
/// hold event.
pub struct MIDIButton {
    is_pressed: bool,
    tap_count: usize,
    last_button_down_time: SystemTime,
}

impl Default for MIDIButton {
    fn default() -> Self {
        Self::new()
    }
}

impl MIDIButton {
    pub fn new() -> Self {
        Self {
            is_pressed: false,
            tap_count: 0,
            last_button_down_time: SystemTime::now(),
        }
    }

    pub fn accept(&mut self, value: u8) -> Option<MIDIButtonEvent> {
        let value = value as f32 / 127.0;
        let was_pressed = self.is_pressed;

        self.is_pressed = value > 0.5;

        if was_pressed && !self.is_pressed {
            Some(MIDIButtonEvent::ButtonUp)
        } else if !was_pressed && self.is_pressed {
            let time_since_last_press = self
                .last_button_down_time
                .elapsed()
                .unwrap_or(Duration::from_millis(0));
            if time_since_last_press.as_millis() > DOUBLE_TAP_THRESHOLD_MS {
                self.tap_count = 0;
            }

            self.tap_count += 1;
            if self.tap_count >= 2 {
                return Some(MIDIButtonEvent::DoubleTap);
            }

            self.last_button_down_time = SystemTime::now();
            Some(MIDIButtonEvent::ButtonDown)
        } else {
            None
        }
    }

    pub fn tick(&mut self) -> Option<MIDIButtonEvent> {
        if self.is_pressed {
            let now = SystemTime::now();
            if now
                .duration_since(self.last_button_down_time)
                .unwrap_or(Duration::from_millis(0))
                .as_secs()
                > HOLD_THRESHOLD_SECS
            {
                self.last_button_down_time = now;
                return Some(MIDIButtonEvent::Hold);
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_button() {
        let _button = MIDIButton::default();
    }

    #[test]
    fn test_detect_button_down() {
        let mut button = MIDIButton::default();
        let event = button.accept(127); // button down
        assert_eq!(event.unwrap(), MIDIButtonEvent::ButtonDown);
    }

    #[test]
    fn test_detect_button_up() {
        let mut button = MIDIButton::default();
        let _ = button.accept(127); // button down
        let event = button.accept(0); // button up
        assert_eq!(event.unwrap(), MIDIButtonEvent::ButtonUp);
    }

    #[test]
    fn test_detect_button_double_tap() {
        let mut button = MIDIButton::default();
        let _ = button.accept(127); // button down
        let _ = button.accept(0); // button up
        let event = button.accept(127); // button down
        assert_eq!(event.unwrap(), MIDIButtonEvent::DoubleTap);
    }

    #[test]
    fn test_detect_button_hold() {
        let mut button = MIDIButton::default();
        let event = button.accept(127);
        assert_eq!(event.unwrap(), MIDIButtonEvent::ButtonDown);
        // The button is pressed so accepting pressed state won't emit events
        let event = button.accept(127);
        assert_eq!(event, None);
        let event = button.tick();
        assert_eq!(event, None);

        // Wait until hold can fire...
        std::thread::sleep(Duration::from_secs(HOLD_THRESHOLD_SECS + 1));
        let event = button.tick();
        assert_eq!(event.unwrap(), MIDIButtonEvent::Hold);
    }
}
