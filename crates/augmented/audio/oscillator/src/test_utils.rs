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
            &RED,
        ))
        .unwrap();
    drawing_area.present().unwrap();
}
