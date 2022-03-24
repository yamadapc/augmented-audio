use super::utils::undenormalize;
use crate::reverb::utils::make_vec;

pub struct LowpassFeedbackCombFilter {
    feedback: f32,
    filter_store: f32,
    damp1: f32,
    damp2: f32,
    buffer: Vec<f32>,
    buffer_cursor: usize,
}

impl LowpassFeedbackCombFilter {
    pub fn new(size: usize) -> Self {
        Self {
            feedback: 0.0,
            filter_store: 0.0,
            damp1: 0.0,
            damp2: 0.0,
            buffer: make_vec(size),
            buffer_cursor: 0,
        }
    }

    pub fn set_damp(&mut self, value: f32) {
        self.damp1 = value;
        self.damp2 = 1.0 - value;
    }

    pub fn set_feedback(&mut self, value: f32) {
        self.feedback = value;
    }

    pub fn process(&mut self, sample: f32) -> f32 {
        let mut output = undenormalize(self.buffer[self.buffer_cursor]);

        self.filter_store = undenormalize(output * self.damp2 + self.filter_store * self.damp1);

        self.buffer[self.buffer_cursor] = sample + self.filter_store * self.feedback;
        self.buffer_cursor += 1;
        if self.buffer_cursor >= self.buffer.len() {
            self.buffer_cursor = 0;
        }

        output
    }
}
