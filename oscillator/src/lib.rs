/// Calculate the phase step increment between samples.
fn get_phase_step<T>(sample_rate: f32) -> T
where
    T: HasPI
        + cpal::Sample
        + std::ops::Div<Output = T>
        + std::ops::AddAssign
        + std::ops::Mul<Output = T>,
{
    T::from(&1.0) / T::from(&sample_rate)
}

pub struct Oscillator<T>
where
    T: HasPI
        + cpal::Sample
        + std::ops::Div<Output = T>
        + std::ops::AddAssign
        + std::ops::Mul<Output = T>,
{
    /// The sample rate to output with
    sample_rate: f32,
    /// A function from `phase` to `sample`
    generator_fn: fn(T) -> T,
    /// The oscillator frequency
    frequency: T,
    /// Current phase of the oscillator
    phase: T,
    /// Calculated current phase step between samples
    phase_step_per_sample: T,
}

impl<T> Oscillator<T>
where
    T: HasPI
        + cpal::Sample
        + std::ops::Div<Output = T>
        + std::ops::AddAssign
        + std::ops::Mul<Output = T>,
{
    /// Construct a new oscillator with a given sample rate
    pub fn new_with_sample_rate(sample_rate: f32, generator_fn: fn(T) -> T) -> Self {
        let frequency = T::from(&440.);
        let phase_step_per_sample = get_phase_step(sample_rate);
        Oscillator {
            sample_rate,
            generator_fn,
            frequency,
            phase: T::from(&0.),
            phase_step_per_sample,
        }
    }

    /// Construct an oscillator with 44.1Hz default sample rate
    pub fn new(generator_fn: fn(T) -> T) -> Self {
        Oscillator::new_with_sample_rate(44100., generator_fn)
    }
}

impl<T> Oscillator<T>
where
    T: HasPI
        + cpal::Sample
        + std::ops::Div<Output = T>
        + std::ops::AddAssign
        + std::ops::Mul<Output = T>,
{
    /// Set the sample rate
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}

impl<T> Oscillator<T>
where
    T: HasPI
        + cpal::Sample
        + std::ops::Div<Output = T>
        + std::ops::AddAssign
        + std::ops::Mul<Output = T>,
{
    /// Get the oscillator frequency
    pub fn get_frequency(&self) -> T {
        self.frequency
    }

    /// Set the oscillator frequency
    pub fn set_frequency(&mut self, frequency: T) {
        self.frequency = frequency;
        self.phase_step_per_sample = get_phase_step(self.sample_rate);
    }
}

impl<T> Oscillator<T>
where
    T: HasPI
        + cpal::Sample
        + std::ops::Div<Output = T>
        + std::ops::AddAssign
        + std::ops::Mul<Output = T>,
{
    /// Process a single sample & update the oscillator phase.
    pub fn next(&mut self) -> T {
        let two_pi = T::get_pi() * T::from(&2.0);
        let radial_phase = self.phase * self.frequency * two_pi;
        let sample = (self.generator_fn)(radial_phase);
        self.phase += self.phase_step_per_sample;
        sample
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
