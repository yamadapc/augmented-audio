// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
use crate::Oscillator;

/// Calculate the cursor step increment between samples.
///
/// This is the fraction of a length which will go through each sample tick
pub fn get_cursor_step(frequency: f32, sample_rate: f32, table_len: f32) -> f32 {
    table_len * frequency / sample_rate
}

/// Fetches a sample from the wave-table by performing linear interpolation
pub fn get_interpolated(cursor: f32, table: &[f32]) -> f32 {
    let c1 = cursor;
    let diff = c1 - c1.floor();
    let c1 = c1 as usize;
    let mut c2 = c1 + 1;
    if c2 >= table.len() {
        c2 = 0;
    }

    let v1 = table[c1];
    let v2 = table[c2];
    v1 + diff * (v2 - v1)
}

pub struct WaveTableOscillator {
    cursor: f32,
    cursor_step: f32,
    table: Vec<f32>,
    table_len: f32,
    sample_rate: f32,
    frequency: f32,
}

impl WaveTableOscillator {
    pub fn from_oscillator(mut oscillator: Oscillator<f32>, table_len: usize) -> Self {
        let frequency = oscillator.get_frequency();
        let sample_rate = oscillator.sample_rate;

        oscillator.set_frequency(sample_rate / table_len as f32);

        let table: Vec<f32> = (0..table_len).map(|_| oscillator.next_sample()).collect();

        let mut result = Self::new(table);
        result.set_sample_rate(sample_rate);
        result.set_frequency(frequency);
        result
    }

    pub fn new(table: Vec<f32>) -> Self {
        let frequency = 440.0;
        let sample_rate = 44100.0;
        Self {
            cursor: 0.0,
            cursor_step: get_cursor_step(frequency, sample_rate, table.len() as f32),
            sample_rate,
            frequency,
            table_len: table.len() as f32,
            table,
        }
    }

    pub fn frequency(&self) -> f32 {
        self.frequency
    }

    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    pub fn table(&self) -> &[f32] {
        &self.table
    }

    pub fn table_mut(&mut self) -> &mut [f32] {
        &mut self.table
    }

    pub fn set_sample_rate(&mut self, value: f32) {
        self.sample_rate = value;
        self.update();
    }

    pub fn set_frequency(&mut self, value: f32) {
        self.frequency = value;
        self.update();
    }

    fn update(&mut self) {
        let frequency = self.frequency;
        let sample_rate = self.sample_rate;
        let table_len = self.table_len;
        let cursor_step = get_cursor_step(frequency, sample_rate, table_len);
        self.cursor_step = cursor_step
    }

    pub fn tick(&mut self) {
        let cursor = &mut self.cursor;
        *cursor += self.cursor_step;
        while *cursor >= self.table_len {
            *cursor -= self.table_len;
        }
    }

    pub fn tick_n(&mut self, samples: f32) {
        let cursor = &mut self.cursor;
        *cursor += self.cursor_step * samples;
        while *cursor >= self.table_len {
            *cursor -= self.table_len;
        }
    }

    pub fn get(&self) -> f32 {
        let cursor = self.cursor;
        let table = &self.table;

        get_interpolated(cursor, table)
    }

    pub fn next_sample(&mut self) -> f32 {
        let result = self.get();
        self.tick();
        result
    }
}

#[cfg(test)]
mod test {
    use crate::test_utils::generate_plot;

    use super::*;

    #[test]
    fn test_get_cursor_step() {
        let step = get_cursor_step(220.0, 44100.0, 512.0);
        assert!((step - 2.55419501).abs() < f32::EPSILON);
    }

    #[test]
    fn test_generate_plots() {
        let root_path = format!("{}/src/wavetable.rs", env!("CARGO_MANIFEST_DIR"));
        let mut oscillator = Oscillator::sine(44100.0);
        oscillator.set_frequency(440.0);
        let mut wave_table = WaveTableOscillator::from_oscillator(oscillator.clone(), 1000);

        generate_plot(&root_path, || wave_table.next_sample(), "wave_table");
        generate_plot(&root_path, || oscillator.next_sample(), "sine_oscillator");
    }

    #[test]
    fn test_cursor_step_is_properly_set() {
        let mut oscillator = Oscillator::sine(44100.0);
        oscillator.set_frequency(440.0);
        let mut wave_table = WaveTableOscillator::from_oscillator(oscillator.clone(), 1000);
        wave_table.set_sample_rate(44100.0);
        wave_table.set_frequency(440.0);
        assert!((wave_table.cursor - 0.0).abs() < f32::EPSILON);
        assert!((wave_table.frequency - 440.0).abs() < f32::EPSILON);
        assert!((wave_table.sample_rate - 44100.0).abs() < f32::EPSILON);

        let cursor_step = wave_table.cursor_step;
        assert!(
            (cursor_step - 9.97732426_f32).abs() < f32::EPSILON,
            "{}",
            cursor_step
        );
    }

    #[test]
    fn test_smoke_test_wave_table_error() {
        let mut oscillator = Oscillator::sine(44100.0);
        oscillator.set_frequency(440.0);
        let mut wave_table = WaveTableOscillator::from_oscillator(oscillator.clone(), 4000);
        wave_table.set_sample_rate(44100.0);
        wave_table.set_frequency(440.0);
        let oscillator_result: Vec<f32> = (0..44100).map(|_| oscillator.next_sample()).collect();
        let wave_table_result: Vec<f32> = (0..44100).map(|_| wave_table.next_sample()).collect();
        for (o, w) in oscillator_result.iter().zip(wave_table_result.iter()) {
            assert!((o - w).abs() < 0.01)
        }
    }
}
