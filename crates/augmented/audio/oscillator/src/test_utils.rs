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

use plotters::prelude::*;

pub fn generate_plot(file: &str, mut generator: impl FnMut() -> f32, plot_name: &str) {
    let filename = Path::new(file);
    let filename = filename.with_file_name(format!(
        "{}--{}.svg",
        filename.file_name().unwrap().to_str().unwrap(),
        plot_name
    ));
    let sine_wave_filename = filename.as_path();

    let mut output_buffer = Vec::new();
    let mut current_seconds = 0.0;
    for _i in 0..440 {
        let sample = generator();
        current_seconds += 1.0 / 44100.0; // increment time past since last sample
        output_buffer.push((current_seconds, sample));
    }

    let svg_backend = SVGBackend::new(sine_wave_filename, (1000, 1000));
    let drawing_area = svg_backend.into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption("oscillator", ("sans-serif", 20))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0.0..current_seconds, -1.2..1.2)
        .unwrap();
    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            output_buffer.iter().map(|(x, y)| (*x, *y as f64)),
            RED,
        ))
        .unwrap();
    drawing_area.present().unwrap();
}
