pub fn auto_correlation(signal: &[f32]) -> Vec<Vec<f32>> {
    // let tau = 5; // lag in samples
    let window_size = 15;

    let mut result = vec![];

    for tau in (0..signal.len()) {
        let mut r = vec![];

        for i in 0..signal.len() {
            let mut s = 0.0;
            for j in (0..window_size) {
                let index = i + j;
                let xj = signal.get(index).cloned().unwrap_or(0.0);
                let xj_tau = signal.get(index + tau).cloned().unwrap_or(0.0);
                s += xj * xj_tau;
            }
            r.push(s);
        }
        result.push(r);
    }

    result
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use audio_processor_testing_helpers::charts::{draw_vec_chart, SVGBackend};
    use audio_processor_testing_helpers::{oscillator_buffer, relative_path};
    use nannou::winit::window::CursorIcon::Default;

    use audio_processor_traits::{AudioBuffer, AudioProcessorSettings, VecAudioBuffer};

    use crate::pitch_detection::yin_estimator::auto_correlation::auto_correlation;

    #[test]
    fn test_draw_acf() {
        let oscillator = oscillator_buffer(
            44100.0,
            440.0,
            Duration::from_millis(20),
            augmented_oscillator::generators::sine_generator,
        );
        draw_vec_chart(
            &*relative_path!("pitch-detection-auto-correlation"),
            "input-signal",
            oscillator.clone(),
        );
        // let output_signal = auto_correlation(&oscillator);
        // draw_vec_chart(
        //     &*relative_path!("pitch-detection-auto-correlation"),
        //     "output-signal",
        //     output_signal,
        // );
    }

    #[test]
    fn test_draw_acf_piano() {
        use plotters::prelude::*;

        let audio_input_path = relative_path!("../../../../input-files/piano-a440.wav");
        let mut input_processor = audio_processor_file::AudioFileProcessor::from_path(
            audio_garbage_collector::handle(),
            AudioProcessorSettings::default(),
            &*audio_input_path,
        )
        .unwrap();
        input_processor.prepare(AudioProcessorSettings::default());
        let mut input_signal = VecAudioBuffer::empty_with(
            2,
            (Duration::from_millis(10).as_secs_f32() * 44100.0) as usize,
            0.0,
        );
        input_processor.process(&mut input_signal);
        let input_signal: Vec<f32> = input_signal.frames().map(|frame| frame[0]).collect();
        let output_path = relative_path!("pitch-detection-auto-correlation-piano.svg");
        let output_signal = auto_correlation(&input_signal);

        let area = SVGBackend::new(&output_path, (800, 800)).into_drawing_area();
        area.fill(&WHITE).unwrap();

        let x_axis = (-0.5..0.5).step(0.01);
        let z_axis = (-0.5..0.5).step(0.01);

        let mut chart = ChartBuilder::on(&area)
            .caption(format!("3D Plot Test"), ("sans", 20))
            .build_cartesian_3d(x_axis.clone(), -3.0..3.0, z_axis.clone())
            .unwrap();

        chart.with_projection(|mut pb| {
            pb.yaw = 0.5;
            pb.scale = 0.3;
            pb.into_matrix()
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .draw()
            .unwrap();

        chart
            .draw_series(
                SurfaceSeries::xoz(
                    output_signal
                        .iter()
                        .enumerate()
                        .map(|(i, _)| i as f64)
                        .collect::<Vec<f64>>()
                        .iter()
                        .cloned(),
                    input_signal
                        .iter()
                        .enumerate()
                        .map(|(i, _)| i as f64)
                        .collect::<Vec<f64>>()
                        .iter()
                        .cloned(),
                    |x, y| output_signal[x as usize][y as usize] as f64,
                )
                .style(BLUE.mix(0.2).filled()),
            )
            .unwrap()
            .label("Surface")
            .legend(|(x, y)| {
                Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled())
            });

        chart
            .configure_series_labels()
            .border_style(&BLACK)
            .draw()
            .unwrap();

        // To avoid the IO failure being ignored silently, we manually call the present function
        area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    }
}
