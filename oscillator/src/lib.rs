pub mod generators;

/// Calculate the phase step increment between samples.
fn get_phase_step(sample_rate: f32, frequency: f32) -> f32 {
    let inverse_sample_rate = 1.0 / sample_rate;
    inverse_sample_rate * (frequency / 2.0)
}

pub struct Oscillator<T>
where
    T: cpal::Sample,
{
    /// The sample rate to output with
    sample_rate: f32,
    /// A function from `phase` to `sample`
    generator_fn: fn(T) -> T,
    /// The oscillator frequency
    frequency: f32,
    /// Current phase of the oscillator
    phase: f32,
    phase_step: f32,
}

impl Oscillator<f32> {
    /// Construct a sine generator
    pub fn sine(sample_rate: f32) -> Oscillator<f32> {
        Oscillator::new_with_sample_rate(sample_rate, generators::sine_generator)
    }
}

impl<T> Oscillator<T>
where
    T: cpal::Sample,
{
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

impl<T> Oscillator<T>
where
    T: cpal::Sample,
{
    /// Set the sample rate
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.phase_step = get_phase_step(self.sample_rate, self.frequency);
    }
}

impl<T> Oscillator<T>
where
    T: cpal::Sample,
{
    /// Get the oscillator frequency
    pub fn get_frequency(&self) -> f32 {
        self.frequency
    }

    /// Set the oscillator frequency
    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
        self.phase_step = get_phase_step(self.sample_rate, self.frequency);
    }
}

impl<T> Oscillator<T>
where
    T: cpal::Sample,
{
    /// Process a single sample & update the oscillator phase.
    pub fn next(&mut self) -> T {
        let result = self.get();
        self.tick();
        result
    }

    pub fn tick(&mut self) {
        self.phase += self.phase_step;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
    }

    pub fn get(&self) -> T {
        // User provided function from phase to a sample
        let sample = (self.generator_fn)(T::from(&self.phase));
        T::from(&sample)
    }
}

/// Trait for numeric types that have an associated PI constant
pub trait HasPI {
    fn get_pi() -> Self;
}

impl HasPI for f32 {
    fn get_pi() -> Self {
        std::f32::consts::PI
    }
}

impl HasPI for i32 {
    fn get_pi() -> Self {
        std::f32::consts::PI as i32
    }
}

impl HasPI for u32 {
    fn get_pi() -> Self {
        std::f32::consts::PI as u32
    }
}
