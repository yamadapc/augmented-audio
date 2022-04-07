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
use std::path::Path;
use std::time::Duration;

use crate::generators::sine_buffer;
use crate::util::rms_level;
use audio_processor_traits::audio_buffer::VecAudioBuffer;
use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};
pub use plotters::prelude::*;

struct FrequencyResponseResult {
    frequency: f32,
    relative_output_level: f32,
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
    }
}

fn get_test_frequencies() -> Vec<f32> {
    let mut freqs = vec![];
    let mut start_freq = 20.0;
    for _ in 0..200 {
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

type SeriesDef = (RGBColor, Vec<f32>);
type MultiSeries = Vec<SeriesDef>;

pub fn draw_vec_chart(filename: &str, plot_name: &str, vec: Vec<f32>) {
    draw_multi_vec_charts(filename, plot_name, vec![(RED, vec)])
}

pub fn draw_multi_vec_charts(filename: &str, plot_name: &str, vecs: MultiSeries) {
    let (_, vec) = &vecs[0];
    let filename = Path::new(filename);
    let chart_filename = filename.with_file_name(format!(
        "{}--{}.png",
        filename.file_name().unwrap().to_str().unwrap(),
        plot_name
    ));

    let backend = BitMapBackend::new(&chart_filename, (1000, 200));
    let drawing_area = backend.into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let x_range = (0, vec.len());
    let y_range = (
        vec.iter().cloned().fold(-1. / 0., f32::max) as f64,
        vec.iter().cloned().fold(1. / 0., f32::min) as f64,
    );

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption(plot_name, ("sans-serif", 20))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(x_range.0..x_range.1, y_range.1..y_range.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    for (color, vec) in vecs {
        let values: Vec<(usize, f64)> = vec
            .iter()
            .enumerate()
            .map(|(i, s)| (i, *s as f64))
            .collect();
        chart.draw_series(LineSeries::new(values, color)).unwrap();
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_compiles() {}
}
