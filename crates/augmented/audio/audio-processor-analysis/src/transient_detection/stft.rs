use rustfft::num_complex::Complex;

use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, SimpleAudioProcessor,
};

use crate::fft_processor::{FftDirection, FftProcessor};
use crate::transient_detection::stft::dynamic_thresholds::DynamicThresholdsParams;
use crate::transient_detection::stft::f_func::FFuncParams;

/// Implements transient detection:
///
/// * https://www.researchgate.net/profile/Balaji-Thoshkahna/publication/220723752_A_Transient_Detection_Algorithm_for_Audio_Using_Iterative_Analysis_of_STFT/links/0deec52e6331412aed000000/A-Transient-Detection-Algorithm-for-Audio-Using-Iterative-Analysis-of-STFT.pdf
fn find_transients<BufferType: AudioBuffer<SampleType = f32>>(data: &mut BufferType) -> Vec<f32> {
    // normalize(data);

    log::info!("Performing FFT...");
    let fft_frames = get_fft_frames(data);

    log::info!("Finding base function values");
    let mut magnitudes: Vec<Vec<f32>> = fft_frames
        .iter()
        .map(|frame| {
            frame
                .iter()
                .enumerate()
                .map(|(bin, frequency_bin)| frequency_bin.norm())
                .collect()
        })
        .collect();

    let mut transient_spectogram_frames: Vec<Vec<f32>> = magnitudes
        .clone()
        .iter()
        .map(|frame| frame.iter().map(|_| 0.0).collect())
        .collect();
    let delta_magnitude = 0.1;

    let num_iterations = 20;
    for iteration_m in 0..num_iterations {
        log::info!("Running iteration {}", iteration_m);
        let t_results = t_func::get_t_func(&magnitudes);
        let f_frames = f_func::get_f_func(
            FFuncParams {
                spectral_spread_bins: 3,
            },
            &t_results,
        );
        let threshold_frames = dynamic_thresholds::calculate_dynamic_thresholds(
            DynamicThresholdsParams {
                time_spread: 3,
                beta: 0.1,
            },
            &f_frames,
        );
        // log::info!("Sample - threshold_frames={:?}", threshold_frames.buffer[0]);
        // log::info!(
        //     "Sample - threshold_frames={:?}",
        //     threshold_frames.buffer[300]
        // );

        let threshold_changed_bins = 4096 / 4;
        let num_changed_bins_frames: Vec<usize> = threshold_frames
            .buffer
            .iter()
            .zip(f_frames.buffer)
            .map(|(threshold_frame, f_frame)| {
                // Γ
                threshold_frame
                    .iter()
                    .zip(f_frame)
                    .map(|(threshold, f)| if f > *threshold { 1 } else { 0 })
                    // end Γ
                    .sum()
            })
            .collect();
        // log::info!("Sample - changed_bins={:?}", num_changed_bins_frames);

        for i in 0..transient_spectogram_frames.len() {
            for j in 0..transient_spectogram_frames[i].len() {
                if iteration_m == 0 || num_changed_bins_frames[i] >= threshold_changed_bins {
                    transient_spectogram_frames[i][j] += delta_magnitude * magnitudes[i][j];
                    magnitudes[i][j] -= (1.0 - delta_magnitude) * magnitudes[i][j];
                }
            }
        }
    }

    let mut planner = rustfft::FftPlanner::new();
    let fft = planner.plan_fft(4096, FftDirection::Inverse);
    let scratch_size = fft.get_inplace_scratch_len();
    let mut scratch = Vec::with_capacity(scratch_size);
    scratch.resize(scratch_size, 0.0.into());

    let mut output = vec![];
    output.resize(data.num_samples(), 0.0);

    let mut cursor = 0;

    for i in 0..fft_frames.len() {
        let frame = &fft_frames[i];
        let mut buffer: Vec<Complex<f32>> = frame
            .iter()
            .zip(&transient_spectogram_frames[i])
            .map(|(input_signal_complex, transient_magnitude)| {
                Complex::from_polar(*transient_magnitude, input_signal_complex.arg())
            })
            .collect();

        fft.process_with_scratch(&mut buffer, &mut scratch);
        for j in 0..buffer.len() {
            if cursor + j < output.len() {
                output[cursor + j] += buffer[j].norm();
            }
        }

        cursor += (frame.len() as f32 * 0.25) as usize;
    }

    output
}

fn get_fft_frames<BufferType: AudioBuffer<SampleType = f32>>(
    data: &mut BufferType,
) -> Vec<Vec<Complex<f32>>> {
    let mut fft = FftProcessor::new(4096, FftDirection::Forward, 0.25);
    let mut fft_frames = vec![];

    for frame in data.frames_mut() {
        fft.s_process_frame(frame);
        if fft.has_changed() {
            fft_frames.push(fft.buffer().clone());
        }
    }

    fft_frames
}

mod dynamic_thresholds {
    use super::f_func::FFuncResults;

    #[derive(Debug)]
    pub struct DynamicThresholds {
        pub buffer: Vec<Vec<f32>>,
    }

    pub struct DynamicThresholdsParams {
        // This is `tau` on the paper
        pub time_spread: usize,
        // "Controls strength of transients to be extracted"
        pub beta: f32,
    }

    pub fn calculate_dynamic_thresholds(
        params: DynamicThresholdsParams,
        f_func_results: &FFuncResults,
    ) -> DynamicThresholds {
        let DynamicThresholdsParams { time_spread, beta } = params;
        let mut result = {
            let mut result = Vec::with_capacity(f_func_results.len());
            for _i in 0..f_func_results.len() {
                result.push({
                    let mut v = Vec::with_capacity(f_func_results.bins());
                    v.resize(f_func_results.bins(), 0.0);
                    v
                });
            }
            result
        };

        for i in 0..f_func_results.len() {
            for j in 0..f_func_results.bins() {
                let mut sum = 0.0;

                {
                    let i = i as i32;
                    let time_spread = time_spread as i32;

                    for l in i - time_spread..i + time_spread {
                        if l >= 0 && l < f_func_results.len() as i32 {
                            sum += f_func_results.buffer[l as usize][j];
                        }
                    }
                }

                result[i][j] = beta * (sum / (2.0 * time_spread as f32 + 1.0));
            }
        }

        DynamicThresholds { buffer: result }
    }
}

mod f_func {
    use super::t_func::TFuncResults;

    pub struct FFuncResults {
        pub buffer: Vec<Vec<f32>>,
    }

    impl FFuncResults {
        pub fn len(&self) -> usize {
            self.buffer.len()
        }

        pub fn bins(&self) -> usize {
            if self.buffer.is_empty() {
                0
            } else {
                self.buffer[0].len()
            }
        }
    }

    pub struct FFuncParams {
        // This is `v` on the paper
        pub spectral_spread_bins: usize,
    }

    fn signal(f: f32) -> f32 {
        if f >= 0.0 {
            1.0
        } else {
            -1.0
        }
    }

    pub fn get_f_func(params: FFuncParams, t_results: &TFuncResults) -> FFuncResults {
        // pre-allocate result
        let mut result = {
            let mut result = Vec::with_capacity(t_results.len());
            for _i in 0..t_results.len() {
                result.push({
                    let mut v = Vec::with_capacity(t_results.bins());
                    v.resize(t_results.bins(), 0.0);
                    v
                });
            }
            result
        };

        let spectral_spread = params.spectral_spread_bins;

        for i in 0..t_results.len() {
            result.push({
                let mut v = Vec::with_capacity(t_results.bins());
                v.resize(t_results.bins(), 0.0);
                v
            });

            for j in 0..t_results.bins() {
                let mut sum = 0.0;
                let spectral_spread = spectral_spread as i32;

                {
                    let j = j as i32;
                    for k in j - spectral_spread..j + spectral_spread {
                        if k >= 0 && k < t_results.bins() as i32 {
                            let minus: f32 = t_results.minus[i][k as usize];
                            let plus: f32 = t_results.plus[i][k as usize];

                            sum += (1.0 + signal(minus)) * minus + (1.0 + signal(plus) * plus);
                        }
                    }
                }

                result[i][j] = 0.5 * sum;
            }
        }

        FFuncResults { buffer: result }
    }
}

mod t_func {
    use rustfft::num_complex::Complex;

    pub struct TFuncResults {
        pub minus: Vec<Vec<f32>>,
        pub plus: Vec<Vec<f32>>,
    }

    impl TFuncResults {
        pub fn len(&self) -> usize {
            self.minus.len()
        }

        pub fn bins(&self) -> usize {
            if self.minus.is_empty() {
                0
            } else {
                self.minus[0].len()
            }
        }
    }

    pub fn get_t_func(magnitudes: &Vec<Vec<f32>>) -> TFuncResults {
        if magnitudes.is_empty() {
            return TFuncResults {
                minus: vec![],
                plus: vec![],
            };
        }

        let empty_frame: Vec<f32> = magnitudes[0].iter().map(|_| 0.0).collect();
        let mut minus = vec![];
        let mut plus = vec![];

        for i in 0..magnitudes.len() {
            let frame = &magnitudes[i];
            let prev_frame = if i > 0 {
                &magnitudes[i - 1]
            } else {
                &empty_frame
            };
            let next_frame = if i < magnitudes.len() - 1 {
                &magnitudes[i + 1]
            } else {
                &empty_frame
            };

            let t_minus = frame.iter().zip(prev_frame).map(|(c, p)| c - p).collect();
            let t_plus = frame.iter().zip(next_frame).map(|(c, p)| c - p).collect();
            minus.push(t_minus);
            plus.push(t_plus)
        }

        TFuncResults { minus, plus }
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::relative_path;

    use audio_processor_file::AudioFileProcessor;
    use audio_processor_traits::{OwnedAudioBuffer, VecAudioBuffer};

    use super::*;

    fn read_input_file(input_file_path: &str) -> impl AudioBuffer<SampleType = f32> {
        log::info!("Reading input file input_file={}", input_file_path);
        let settings = AudioProcessorSettings::default();
        let mut input = AudioFileProcessor::from_path(
            audio_garbage_collector::handle(),
            settings,
            &input_file_path,
        )
        .unwrap();
        input.prepare(settings);
        let input_buffer = input.buffer();
        let mut buffer = VecAudioBuffer::new();
        buffer.resize(1, input_buffer[0].len(), 0.0);
        for (channel_index, channel) in input_buffer.iter().enumerate() {
            for (sample_index, sample) in channel.iter().enumerate() {
                buffer.set(0, sample_index, *sample + buffer.get(0, sample_index));
            }
        }
        buffer
    }

    #[test]
    fn test_transient_detector() {
        use visualization::draw;

        wisual_logger::init_from_env();

        let output_path = relative_path!("./src/transient_detection/stft.png");

        let input_path = relative_path!("./sample.mp3");
        // let input_path = relative_path!("../../../../input-files/C3-loop.mp3");
        let mut input = read_input_file(&input_path);
        let transients = find_transients(&mut input);

        let frames: Vec<f32> = input.frames().map(|frame| frame[0]).collect();

        assert_eq!(frames.len(), transients.len());

        draw(&output_path, &frames, &transients);
    }

    mod visualization {
        use std::cmp::Ordering;

        use audio_processor_testing_helpers::relative_path;
        use piet::kurbo::{PathEl, Point, Rect};
        use piet::{Color, RenderContext};
        use piet_common::{CoreGraphicsContext, Device};

        pub fn draw(output_file_path: &str, frames: &[f32], transients: &[f32]) {
            log::info!("Rendering image...");
            let width = 2000;
            let height = 1000;

            let mut device = Device::new().unwrap();
            let mut bitmap = device.bitmap_target(width, height, 1.0).unwrap();
            let mut render_context = bitmap.render_context();

            render_context.fill(
                Rect::new(0.0, 0.0, width as f64, height as f64),
                &Color::rgb(1.0, 1.0, 1.0),
            );
            let signal_color = Color::rgb(1.0, 0.0, 0.0);
            draw_line(&mut render_context, width, height, frames, &signal_color);
            let signal_color = Color::rgb(0.0, 1.0, 0.0);
            draw_line(
                &mut render_context,
                width,
                height,
                transients,
                &signal_color,
            );
            let signal_color = Color::rgb(0.0, 0.0, 1.0);

            let max_transient = transients
                .iter()
                .max_by(|f1, f2| f1.partial_cmp(f2).unwrap());
            let threshold = max_transient.unwrap() / 2.0;
            let gated_transients: Vec<f32> = transients
                .iter()
                .map(|transient| if *transient > threshold { 1.0 } else { 0.0 })
                .collect();
            draw_line(
                &mut render_context,
                width,
                height,
                &gated_transients,
                &signal_color,
            );

            render_context.finish().unwrap();
            std::mem::drop(render_context);

            bitmap
                .save_to_file(output_file_path)
                .expect("Failed to save image");
        }

        fn draw_line(
            render_context: &mut CoreGraphicsContext,
            width: usize,
            height: usize,
            frames: &[f32],
            signal_color: &Color,
        ) {
            let len = frames.len() as f64;
            let order = |f1: f32, f2: f32| f1.partial_cmp(&f2).unwrap_or(Ordering::Less);
            let min_sample = *frames.iter().min_by(|f1, f2| order(**f1, **f2)).unwrap() as f64;
            let max_sample = *frames.iter().max_by(|f1, f2| order(**f1, **f2)).unwrap() as f64;
            let fwidth = width as f64;
            let fheight = height as f64;
            let map_sample = |sig| {
                let r = (sig as f64 - min_sample) / (max_sample - min_sample);
                fheight - r * fheight
            };

            let mut path: Vec<PathEl> = frames
                .iter()
                .enumerate()
                .map(|(i, s)| ((i as f64 / len) * fwidth, map_sample(*s)))
                .filter(|(_i, x)| !(x.is_nan() || x.is_infinite()))
                .map(|(x, y)| Point::new(x, y))
                .map(|point| PathEl::LineTo(point))
                .collect();
            path.insert(0, PathEl::MoveTo(Point::new(0.0, fheight / 2.0)));
            path.push(PathEl::LineTo(Point::new(fwidth, fheight / 2.0)));
            render_context.stroke(&*path, signal_color, 0.5);
        }
    }
}
