use std::time::{Duration, SystemTime};

pub enum MIDIButtonEvent {
    ButtonDown,
    ButtonUp,
    DoubleTap,
    Hold,
}

pub struct MIDIButton {
    is_pressed: bool,
    tap_count: usize,
    last_button_down_time: SystemTime,
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
            if time_since_last_press.as_millis() > 500 {
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
                > 2
            {
                self.last_button_down_time = now;
                return Some(MIDIButtonEvent::Hold);
            }
        }

        None
    }
}
