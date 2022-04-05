pub mod generators;
pub mod wavetable;

#[cfg(test)]
mod test_utils;

/// Calculate the phase step increment between samples.
///
/// This is the fraction of a period which will go through each sample tick
pub fn get_phase_step(sample_rate: f32, frequency: f32) -> f32 {
    frequency / sample_rate
}

#[derive(Clone)]
pub struct Oscillator<T> {
    /// Current phase of the oscillator
    phase: f32,
    phase_step: f32,
    /// A function from `phase` to `sample`
    generator_fn: fn(T) -> T,
    /// The sample rate to output with
    sample_rate: f32,
    /// The oscillator frequency
    frequency: f32,
}

impl Oscillator<f32> {
    /// Construct a sine generator
    pub fn sine(sample_rate: f32) -> Oscillator<f32> {
        Oscillator::new_with_sample_rate(sample_rate, generators::sine_generator)
    }
}

impl<T> Oscillator<T> {
    /// Construct a new oscillator with a given sample rate
    pub fn new_with_sample_rate(sample_rate: f32, generator_fn: fn(T) -> T) -> Self {
        let frequency = 440.;
        let phase_step = get_phase_step(sample_rate, frequency);
        Oscillator {
            sample_rate,
            generator_fn,
            frequency,
            phase: 0.,
            phase_step,
        }
    }

    /// Construct an oscillator with 44.1Hz default sample rate
    pub fn new(generator_fn: fn(T) -> T) -> Self {
        Oscillator::new_with_sample_rate(44100., generator_fn)
    }
}

impl<T> Oscillator<T> {
    /// Set the sample rate
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.phase_step = get_phase_step(self.sample_rate, self.frequency);
    }
}

impl<T> Oscillator<T> {
    /// Get the oscillator frequency
    pub fn get_frequency(&self) -> f32 {
        self.frequency
    }

    /// Set the oscillator frequency
    pub fn set_frequency(&mut self, frequency: f32) {
        if (frequency - self.frequency).abs() < f32::EPSILON {
            return;
        }

        self.frequency = frequency;
        self.phase_step = get_phase_step(self.sample_rate, self.frequency);
    }
}

impl<T> Oscillator<T>
where
    T: From<f32>,
{
    /// Process a single sample & update the oscillator phase.
    pub fn next_sample(&mut self) -> T {
        let result = self.get();
        self.tick();
        result
    }

    pub fn tick_n(&mut self, n: f32) {
        self.phase = self.phase + n * self.phase_step;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }
    }

    pub fn tick(&mut self) {
        self.phase = self.phase + self.phase_step;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }
    }

    pub fn value_for_phase(&self, phase: T) -> T {
        (self.generator_fn)(phase)
    }

    pub fn get(&self) -> T {
        // User provided function from phase to a sample
        (self.generator_fn)(T::from(self.phase))
    }

    /// Return the current phase as a number between 0-1
    pub fn phase(&self) -> f32 {
        self.phase
    }
}

#[cfg(test)]
mod test {
    use crate::test_utils::generate_plot;

    use super::*;

    static DEFAULT_SAMPLE_RATE: f32 = 44100.0;

    #[test]
    fn test_generate_plots() {
        let root_path = format!("{}/src/lib.rs", env!("CARGO_MANIFEST_DIR"));
        let mut oscillator = Oscillator::sine(DEFAULT_SAMPLE_RATE);
        oscillator.set_frequency(440.0);
        generate_plot(&root_path, || oscillator.next_sample(), "sine-wave");
        let mut oscillator =
            Oscillator::new_with_sample_rate(DEFAULT_SAMPLE_RATE, generators::square_generator);
        oscillator.set_frequency(440.0);
        generate_plot(&root_path, || oscillator.next_sample(), "square-wave");
        let mut oscillator =
            Oscillator::new_with_sample_rate(DEFAULT_SAMPLE_RATE, generators::saw_generator);
        oscillator.set_frequency(440.0);
        generate_plot(&root_path, || oscillator.next_sample(), "saw-wave");
    }
}
