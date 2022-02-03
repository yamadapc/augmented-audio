pub mod generators;

/// Calculate the phase step increment between samples.
///
/// This is the fraction of a period which will go through each sample tick
fn get_phase_step(sample_rate: f32, frequency: f32) -> f32 {
    frequency / sample_rate
}

#[derive(Clone)]
pub struct Oscillator<T> {
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
    pub fn phase(&self) -> f32 {
        self.phase
    }

    /// Process a single sample & update the oscillator phase.
    pub fn next_sample(&mut self) -> T {
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

    pub fn value_for_phase(&self, phase: T) -> T {
        (self.generator_fn)(phase)
    }

    pub fn get(&self) -> T {
        // User provided function from phase to a sample
        (self.generator_fn)(T::from(self.phase))
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use plotters::prelude::*;

    use super::*;

    static DEFAULT_SAMPLE_RATE: f32 = 44100.0;

    #[test]
    fn test_generate_plots() {
        let mut oscillator = Oscillator::sine(DEFAULT_SAMPLE_RATE);
        generate_plot(&mut oscillator, "sine-wave");
        let mut oscillator =
            Oscillator::new_with_sample_rate(DEFAULT_SAMPLE_RATE, generators::square_generator);
        generate_plot(&mut oscillator, "square-wave");
        let mut oscillator =
            Oscillator::new_with_sample_rate(DEFAULT_SAMPLE_RATE, generators::saw_generator);
        generate_plot(&mut oscillator, "saw-wave");
    }

    fn generate_plot(oscillator: &mut Oscillator<f32>, plot_name: &str) {
        let filename = Path::new(file!());
        let sine_wave_filename = filename.with_file_name(format!(
            "{}--{}.svg",
            filename.file_name().unwrap().to_str().unwrap(),
            plot_name
        ));
        let sine_wave_filename = sine_wave_filename.as_path();
        oscillator.set_frequency(440.0);

        let mut output_buffer = Vec::new();
        let mut current_seconds = 0.0;
        for _i in 0..440 {
            let sample = oscillator.next_sample();
            current_seconds += 1.0 / 44100.0; // increment time past since last sample
            output_buffer.push((current_seconds, sample));
        }

        let svg_backend = SVGBackend::new(sine_wave_filename, (1000, 1000));
        let drawing_area = svg_backend.into_drawing_area();
        drawing_area.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&drawing_area)
            .caption("Sine oscillator", ("sans-serif", 20))
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(0.0..current_seconds, -1.2..1.2)
            .unwrap();
        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                output_buffer.iter().map(|(x, y)| (*x, *y as f64)),
                &RED,
            ))
            .unwrap();
    }
}
