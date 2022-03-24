use super::utils::undenormalize;
use crate::reverb::utils::make_vec;

pub struct AllPass {
    feedback: f32,
    buffer: Vec<f32>,
    buffer_cursor: usize,
}

impl AllPass {
    pub fn new(size: usize) -> Self {
        Self {
            feedback: 0.0,
            buffer: make_vec(size),
            buffer_cursor: 0,
        }
    }

    pub fn set_feedback(&mut self, value: f32) {
        self.feedback = value;
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let buffer_output = undenormalize(self.buffer[self.buffer_cursor]);
        let output = -input + buffer_output;
        self.buffer[self.buffer_cursor] = input + buffer_output * self.feedback;

        self.buffer_cursor += 1;
        if self.buffer_cursor >= self.buffer.len() {
            self.buffer_cursor = 0;
        }

        output
    }
}
