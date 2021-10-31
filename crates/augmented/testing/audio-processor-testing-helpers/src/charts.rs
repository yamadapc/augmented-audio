use std::path::Path;
use std::time::Duration;

use crate::generators::sine_buffer;
use crate::util::rms_level;
use audio_processor_traits::audio_buffer::VecAudioBuffer;
use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};
use plotters::prelude::*;

struct FrequencyResponseResult {
    frequency: f32,
    relative_output_level: f32,
    input_rms: f32,
    output_rms: f32,
}

fn test_frequency_response<Processor>(
    sample_rate: f32,
    frequency: f32,
    audio_processor: &mut Processor,
) -> FrequencyResponseResult
where
    Processor: AudioProcessor<SampleType = f32>,
{
    let input_buffer = sine_buffer(sample_rate, frequency, Duration::from_millis(50));
    let mut input_buffer = VecAudioBuffer::from(input_buffer);
    let input_rms = rms_level(input_buffer.slice());

    audio_processor.process(&mut input_buffer);

    let output_rms = rms_level(input_buffer.slice());

    let relative_output_level = output_rms / input_rms;
    FrequencyResponseResult {
        frequency,
        relative_output_level,
        input_rms,
        output_rms,
    }
}

fn get_test_frequencies() -> Vec<f32> {
    let mut freqs = vec![];
    let mut start_freq = 20.0;
    for i in 0..200 {
        freqs.push(start_freq);
        start_freq += 20.0;
    }
    freqs
}

struct FrequencyResponseChartModel {
    x_range: (f64, f64),
    y_range: (f64, f64),
    values: Vec<(f64, f64)>,
}

fn build_frequency_response_chart_model(
    responses: Vec<FrequencyResponseResult>,
) -> FrequencyResponseChartModel {
    let min_x = 0.0;
    let max_x = responses
        .iter()
        .map(|r| r.frequency as f64)
        .fold(-1. / 0., f64::max);
    let min_y = 0.0;
    let max_y = responses
        .iter()
        .map(|r| r.relative_output_level as f64)
        .fold(-1. / 0., f64::max);

    let values = responses
        .iter()
        .map(|r| (r.frequency as f64, r.relative_output_level as f64))
        .collect();

    FrequencyResponseChartModel {
        x_range: (min_x, max_x),
        y_range: (min_y, max_y),
        values,
    }
}

/// Generates a frequency response plot for a given audio processor
pub fn generate_frequency_response_plot<Processor>(
    filename: &str,
    plot_name: &str,
    audio_processor: &mut Processor,
) where
    Processor: AudioProcessor<SampleType = f32>,
{
    let mut settings = AudioProcessorSettings::default();
    settings.sample_rate = 22050.0;
    settings.input_channels = 1;
    settings.output_channels = 1;
    audio_processor.prepare(settings);
    let sample_rate = settings.sample_rate;

    let frequencies = get_test_frequencies();
    let responses = frequencies
        .iter()
        .map(|frequency| test_frequency_response(sample_rate, *frequency, audio_processor))
        .collect();
    let chart_model = build_frequency_response_chart_model(responses);

    let filename = Path::new(filename);
    let chart_filename = filename.with_file_name(format!(
        "{}--{}.svg",
        filename.file_name().unwrap().to_str().unwrap(),
        plot_name
    ));

    let svg_backend = SVGBackend::new(&chart_filename, (1000, 1000));
    let drawing_area = svg_backend.into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption(plot_name, ("sans-serif", 20))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(
            chart_model.x_range.0..chart_model.x_range.1,
            chart_model.y_range.0..chart_model.y_range.1,
        )
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(chart_model.values, &RED))
        .unwrap();
    println!(">>> Wrote {} chart to {:?}", plot_name, chart_filename);
}

#[cfg(test)]
mod test {
    #[test]
    fn it_compiles() {}
}
