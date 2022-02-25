use rustfft::num_complex::Complex;

use audio_processor_traits::{AudioBuffer, AudioProcessorSettings, SimpleAudioProcessor};

use crate::fft_processor::{FftDirection, FftProcessor};
use crate::transient_detection::stft::dynamic_thresholds::{
    DynamicThresholds, DynamicThresholdsParams,
};
use crate::transient_detection::stft::power_change::{PowerOfChangeFrames, PowerOfChangeParams};

struct IterativeTransientDetectionParams {
    /// Size of the FFT windows, defaults to 2048; at 44.1kHz each frame should be ~40ms
    fft_size: usize,
    /// If 0.75 is provided, 3/4 of the windows will overlap. Defaults to 3/4
    fft_overlap_ratio: f32,
    /// `v` in the paper (equation `5`)
    ///
    /// Defaults to 3 frequency bins or roughly 60Hz at 44.1kHz sample rate
    power_of_change_spectral_spread: usize,
    /// `τ` in the paper (equation 7)
    ///
    /// * When calculating dynamic thresholds, controls how many neighbouring time frames are
    ///   considered
    /// * For example, if `threshold_time_spread_factor` is 2.0, a frequency bin and its
    ///   `spectral_spread` neighbours will have to be 2.0 the average of the `time_spread` time
    ///   frames' rate of change for this bin
    ///
    /// Defaults to 3
    threshold_time_spread: usize,
    /// `β` - `threshold_time_spread_factor` (equation 7)
    ///
    /// * Internal multiplier of dynamic thresholds
    /// * This nº affects by what factor a frequency bin needs to change in relation to what it has
    ///   changed in neighboring frames to be considered a transient
    /// * Higher nºs means sensitivity is decreased
    ///
    /// Defaults to 2.0
    threshold_time_spread_factor: f32,
    /// How many bins should change for a frame to be considered a transient
    ///
    /// Defaults to 1/4 of the fft_size
    frequency_bin_change_threshold: usize,
    /// `δ` - `iteration_magnitude_factor` (equation 10)
    ///
    /// * What factor of the magnitude is collected onto the output per iteration
    ///
    /// Defaults to 0.1
    iteration_magnitude_factor: f32,
    /// `N` - `iteration_count` (algorithm 1)
    ///
    /// Defaults to 20
    iteration_count: usize,
}

impl Default for IterativeTransientDetectionParams {
    fn default() -> Self {
        let fft_size = 2048;
        let frequency_bin_change_threshold = 2048 / 4;
        Self {
            fft_size,
            fft_overlap_ratio: 0.75,
            power_of_change_spectral_spread: 3,
            threshold_time_spread: 2,
            threshold_time_spread_factor: 1.5,
            iteration_magnitude_factor: 0.1,
            iteration_count: 20,
            frequency_bin_change_threshold,
        }
    }
}

/// Implements iterative STFT transient detection for polyphonic signals. Output is a monophonic
/// audio track of the transient signal. Not real-time safe.
///
/// Reference:
/// * https://www.researchgate.net/profile/Balaji-Thoshkahna/publication/220723752_A_Transient_Detection_Algorithm_for_Audio_Using_Iterative_Analysis_of_STFT/links/0deec52e6331412aed000000/A-Transient-Detection-Algorithm-for-Audio-Using-Iterative-Analysis-of-STFT.pdf
///
/// Inputs to the algorithm:
/// * `fft_size` - Size of the FFT window
/// * `fft_overlap` - Amount of overlap between windows
/// * `v` - `power_of_change_spectral_spread` (equation 5)
/// * `β` - `threshold_time_spread_factor` (equation 7)
///   * Internal multiplier of dynamic thresholds
///   * This nº affects by what factor a frequency bin's rate of change needs to be higher than its
///     time-domain neighbours
///   * Higher nºs means sensitivity is decreased
/// * `λThr` - `frequency_bin_change_threshold` (equation 10)
///   * If this amount of frequency bins have changed, this frame will be considered a transient
/// * `δ` - `iteration_magnitude_factor` (equation 10)
///   * What factor of the magnitude is collected onto the output per iteration
/// * `N` - `iteration_count` (algorithm 1)
///
/// The algorithm is as follows:
/// * Perform FFT with overlaping windows at 3/4's ratio (e.g. one 40ms window every 30ms)
/// * Calculate `M(frame, bin)` **magnitudes for each frame/bin**
/// * Let `P(frame, bin)` be the output `transient_magnitude_frames`
///   * These are the transient magnitude frames
///   * e.g. Magnitudes of the transients, per frequency bin, over time
/// * for iteration in 0..`N`
///   * For each frame/bin, calculate **power of change** value `F(frame, bin)`
///     * First we calculate `T-(frame, bin)` and `T+(frame, bin)`, respectively the deltas in
///       magnitude with the previous and next frames respectively
///     * `F(frame, bin)` (**power of change** represents how much its magnitude is higher compared
///       with next and previous frames, if it's higher than its next/previous frames (0.0
///       if its not higher than neighbouring time-domain frames)
///     * For each `bin` this `power_of_change` is summed with its `v` (`power_of_change_spectral_spread`)
///       neighbour frequency bins
///     * This is for of 'peak detection' in some way, it's finding frames higher than their
///       time-domain peers and quantifying how much they're larger than them
///   * Calculate `dynamic_thresholds` `λ(frame, bin)`
///     * For this, on every frequency bin, the threshold is the average **power of change** of the
///       `l` (`threshold_time_spread`) neighbouring time-domain frames, multiplied by a magic
///       constant `β` (`threshold_time_spread_factor`)
///     * A frequency bin's threshold is defined by how much its neighbour frequency bins have
///       changed in this frame* (change being quantified by `F`)
///   * Calculate `Γ(frame, bin)` (`have_bins_changed`), by flipping a flag to 1 or 0 depending on
///     whether **power of change**  `F(frame, bin)` is higher than its **dynamic threshold**
///     `λ(frame, bin)`
///   * Calculate `ΣΓ` `num_changed_bins`, by counting the number of frequency bins that have
///     changed in this frame
///     * Simply sum the above for each frame
///   * If `ΣΓ(frame)` `num_changed_bins` is higher than `λThr` - `frequency_bin_change_threshold`
///     * Update `P(frame, bin)` - `transient_magnitude_frames` adding `X(frame, bin)` times `δ`
///       `iteration_magnitude_factor` onto it
///     * Subtract `(1 - δ) * X(frame, bin)` from `X(frame, bin)`
/// * At the end of `N` iterations, perform the inverse fourier transform over the each polar
///   complex nº frame using magnitudes in `transient_magnitude_frames` and using phase from the
///   input FFT result
/// * There may now be extra filtering / smoothing steps to extract data or audio, but the output
///   should be the transient signal
fn find_transients<BufferType: AudioBuffer<SampleType = f32>>(
    params: IterativeTransientDetectionParams,
    data: &mut BufferType,
) -> Vec<f32> {
    let IterativeTransientDetectionParams {
        fft_size,
        fft_overlap_ratio,
        power_of_change_spectral_spread,
        threshold_time_spread,
        threshold_time_spread_factor,
        frequency_bin_change_threshold,
        iteration_magnitude_factor,
        iteration_count,
    } = params;

    log::info!("Performing FFT...");
    let fft_frames = get_fft_frames(fft_size, fft_overlap_ratio, data);

    log::info!("Finding base function values");
    let mut magnitude_frames: Vec<Vec<f32>> = get_magnitudes(&fft_frames);
    let mut transient_magnitude_frames: Vec<Vec<f32>> =
        initialize_result_transient_magnitude_frames(&mut magnitude_frames);

    for iteration in 0..iteration_count {
        log::info!("Running iteration {}", iteration);
        let t_results = frame_deltas::calculate_deltas(&magnitude_frames);
        let f_frames = power_change::calculate_power_of_change(
            PowerOfChangeParams {
                spectral_spread_bins: power_of_change_spectral_spread,
            },
            &t_results,
        );
        let threshold_frames = dynamic_thresholds::calculate_dynamic_thresholds(
            DynamicThresholdsParams {
                threshold_time_spread,
                threshold_time_spread_factor,
            },
            &f_frames,
        );

        let num_changed_bins_frames: Vec<usize> =
            count_changed_bins_per_frame(f_frames, threshold_frames);

        update_output_and_magnitudes(
            iteration_magnitude_factor,
            frequency_bin_change_threshold,
            num_changed_bins_frames,
            &mut magnitude_frames,
            &mut transient_magnitude_frames,
        );
    }

    generate_output_frames(
        fft_size,
        fft_overlap_ratio,
        data,
        &fft_frames,
        &mut transient_magnitude_frames,
    )
}

/// Last step on iteration, collect `iteration_magnitude_factor * M(frame, bin)` if this whole frame
/// is a transient, subtract `1.0 - iteration_magnitude_factor` from the magnitude frames.
fn update_output_and_magnitudes(
    iteration_magnitude_factor: f32,
    frequency_bin_change_threshold: usize,
    num_changed_bins_frames: Vec<usize>,
    magnitude_frames: &mut Vec<Vec<f32>>,
    transient_magnitude_frames: &mut Vec<Vec<f32>>,
) {
    for i in 0..transient_magnitude_frames.len() {
        for j in 0..transient_magnitude_frames[i].len() {
            if num_changed_bins_frames[i] >= frequency_bin_change_threshold {
                transient_magnitude_frames[i][j] +=
                    iteration_magnitude_factor * magnitude_frames[i][j];
                magnitude_frames[i][j] -=
                    (1.0 - iteration_magnitude_factor) * magnitude_frames[i][j];
            }
        }
    }
}

/// Equations 8 and 9
fn count_changed_bins_per_frame(
    f_frames: PowerOfChangeFrames,
    threshold_frames: DynamicThresholds,
) -> Vec<usize> {
    threshold_frames
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
        .collect()
}

/// Perform inverse FFT over spectogram frames
fn generate_output_frames<BufferType: AudioBuffer<SampleType = f32>>(
    fft_size: usize,
    fft_overlap_ratio: f32,
    data: &mut BufferType,
    fft_frames: &Vec<Vec<Complex<f32>>>,
    transient_magnitude_frames: &mut Vec<Vec<f32>>,
) -> Vec<f32> {
    let mut planner = rustfft::FftPlanner::new();
    let fft = planner.plan_fft(fft_size, FftDirection::Inverse);
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
            .zip(&transient_magnitude_frames[i])
            .map(|(input_signal_complex, transient_magnitude)| {
                Complex::from_polar(*transient_magnitude, input_signal_complex.arg())
            })
            .collect();

        fft.process_with_scratch(&mut buffer, &mut scratch);
        for j in 0..buffer.len() {
            if cursor + j < output.len() {
                output[cursor + j] += buffer[j].re;
            }
        }

        cursor += (frame.len() as f32 * (1.0 - fft_overlap_ratio)) as usize;
    }

    let maximum_output = output
        .iter()
        .map(|f| f.abs())
        .max_by(|f1, f2| f1.partial_cmp(f2).unwrap())
        .unwrap();
    for sample in &mut output {
        if sample.abs() > maximum_output * 0.05 {
            *sample = *sample / maximum_output;
        } else {
            *sample = 0.0;
        }
    }

    output
}

fn initialize_result_transient_magnitude_frames(magnitudes: &mut Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    magnitudes
        .clone()
        .iter()
        .map(|frame| frame.iter().map(|_| 0.0).collect())
        .collect()
}

fn get_magnitudes(fft_frames: &Vec<Vec<Complex<f32>>>) -> Vec<Vec<f32>> {
    fft_frames
        .iter()
        .map(|frame| {
            frame
                .iter()
                .map(|frequency_bin| frequency_bin.norm())
                .collect()
        })
        .collect()
}

fn get_fft_frames<BufferType: AudioBuffer<SampleType = f32>>(
    fft_size: usize,
    fft_overlap_ratio: f32,
    data: &mut BufferType,
) -> Vec<Vec<Complex<f32>>> {
    let mut fft = FftProcessor::new(fft_size, FftDirection::Forward, fft_overlap_ratio);
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
    use super::power_change::PowerOfChangeFrames;

    /// This is `λ(frame, bin)` in the paper.
    ///
    /// It holds dynamic thresholds for each frequency bin. Dynamic thresholds are defined as a
    /// factor of how much neighbouring frequency bins power-of-change is.
    #[derive(Debug)]
    pub struct DynamicThresholds {
        pub buffer: Vec<Vec<f32>>,
    }

    pub struct DynamicThresholdsParams {
        // This is `tau` on the paper
        pub threshold_time_spread: usize,
        // This is `beta` on the paper (Controls strength of transients to be extracted)
        pub threshold_time_spread_factor: f32,
    }

    pub fn calculate_dynamic_thresholds(
        params: DynamicThresholdsParams,
        power_of_change_frames: &PowerOfChangeFrames,
    ) -> DynamicThresholds {
        let DynamicThresholdsParams {
            threshold_time_spread: time_spread,
            threshold_time_spread_factor: beta,
        } = params;
        let mut result = {
            let mut result = Vec::with_capacity(power_of_change_frames.len());
            for _i in 0..power_of_change_frames.len() {
                result.push({
                    let mut v = Vec::with_capacity(power_of_change_frames.bins());
                    v.resize(power_of_change_frames.bins(), 0.0);
                    v
                });
            }
            result
        };

        for i in 0..power_of_change_frames.len() {
            for j in 0..power_of_change_frames.bins() {
                let mut sum = 0.0;

                {
                    let i = i as i32;
                    let time_spread = time_spread as i32;

                    for l in i - time_spread..i + time_spread {
                        if l >= 0 && l < power_of_change_frames.len() as i32 {
                            sum += power_of_change_frames.buffer[l as usize][j];
                        }
                    }
                }

                result[i][j] = beta * (sum / (2.0 * time_spread as f32 + 1.0));
            }
        }

        DynamicThresholds { buffer: result }
    }
}

mod power_change {
    use super::frame_deltas::DeltaFrames;

    /// This is `F(frame, bin)` in the paper. Equation `5`.
    ///
    /// This represents how much for each frame and each bin summed with `v`
    /// neighbours has changed compared with previous and next frames.
    pub struct PowerOfChangeFrames {
        pub buffer: Vec<Vec<f32>>,
    }

    impl PowerOfChangeFrames {
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

    pub struct PowerOfChangeParams {
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

    pub fn calculate_power_of_change(
        params: PowerOfChangeParams,
        t_results: &DeltaFrames,
    ) -> PowerOfChangeFrames {
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

        PowerOfChangeFrames { buffer: result }
    }
}

mod frame_deltas {
    /// This is `T-` and `T+` in the paper. Equations 3 and 4.
    ///
    /// For each frame:
    /// * `frame_deltas.minus` is the delta with the previous frame
    /// * `frame_deltas.plus` is the delta with the next frame
    pub struct DeltaFrames {
        pub minus: Vec<Vec<f32>>,
        pub plus: Vec<Vec<f32>>,
    }

    impl DeltaFrames {
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

    pub fn calculate_deltas(magnitudes: &Vec<Vec<f32>>) -> DeltaFrames {
        if magnitudes.is_empty() {
            return DeltaFrames {
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

        DeltaFrames { minus, plus }
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::relative_path;

    use audio_processor_file::{AudioFileProcessor, OutputAudioFileProcessor};
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

        // We read at most 10s of audio & mono it.
        let max_len = (settings.sample_rate() * 10.0) as usize;
        buffer.resize(1, input_buffer[0].len().min(max_len), 0.0);
        for channel in input_buffer.iter() {
            for (sample_index, sample) in channel.iter().enumerate().take(max_len) {
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

        // let input_path = relative_path!("../../../../input-files/C3-loop.mp3");
        let input_path = relative_path!("./hiphop-drum-loop.mp3");
        let transients_file_path = format!("{}.transients.wav", input_path);
        let mut input = read_input_file(&input_path);
        let frames: Vec<f32> = input.frames().map(|frame| frame[0]).collect();
        let max_input = frames
            .iter()
            .map(|f| f.abs())
            .max_by(|f1, f2| f1.partial_cmp(f2).unwrap())
            .unwrap();

        let transients = find_transients(IterativeTransientDetectionParams::default(), &mut input);
        assert_eq!(frames.len(), transients.len());
        draw(&output_path, &frames, &transients);

        let settings = AudioProcessorSettings {
            input_channels: 1,
            output_channels: 1,
            ..AudioProcessorSettings::default()
        };
        let mut output_processor =
            OutputAudioFileProcessor::from_path(settings.clone(), &transients_file_path);
        output_processor.prepare(settings);
        // match input signal
        let mut transients: Vec<f32> = transients.iter().map(|f| f * max_input).collect();
        output_processor.process(&mut transients);
    }

    mod visualization {
        use std::cmp::Ordering;

        use piet::kurbo::{Affine, Line, PathEl, Point, Rect};
        use piet::{Color, RenderContext, Text, TextAttribute, TextLayoutBuilder};
        use piet_common::{CoreGraphicsContext, Device};

        use audio_processor_traits::AudioProcessorSettings;

        use crate::peak_detector::PeakDetector;

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

            let num_charts = 5;

            // Draw audio line
            draw_line(
                &mut render_context,
                width,
                height / num_charts,
                frames,
                &signal_color,
            );
            let label = "Input audio";
            draw_text(&mut render_context, label);

            // Draw transient signal
            let _ = render_context.save();
            render_context.transform(Affine::translate((0.0, height as f64 / num_charts as f64)));
            let signal_color = Color::rgb(0.0, 1.0, 0.0);
            draw_line(
                &mut render_context,
                width,
                height / num_charts,
                transients,
                &signal_color,
            );
            let label = "Transient signal";
            draw_text(&mut render_context, label);
            let _ = render_context.restore();

            // Draw transient lines
            let _ = render_context.save();
            render_context.transform(Affine::translate((
                0.0,
                2.0 * (height as f64 / num_charts as f64),
            )));
            let signal_color = Color::rgb(0.0, 0.0, 1.0);
            let gated_transients = get_gated_transients(transients);
            draw_line(
                &mut render_context,
                width,
                height / num_charts,
                &gated_transients,
                &signal_color,
            );
            let label = "Transient magnitude";
            draw_text(&mut render_context, label);
            let _ = render_context.restore();

            // Draw transients through peak detector
            let mut peak_detector = PeakDetector::default();
            let attack_mult = crate::peak_detector::calculate_multiplier(
                AudioProcessorSettings::default().sample_rate,
                0.1,
            );
            let release_mult = crate::peak_detector::calculate_multiplier(
                AudioProcessorSettings::default().sample_rate,
                15.0,
            );
            let _ = render_context.save();
            let gated_transients: Vec<f32> = gated_transients
                .iter()
                .map(|f| {
                    peak_detector.accept_frame(attack_mult, release_mult, &[*f]);
                    peak_detector.value()
                })
                .collect();
            render_context.transform(Affine::translate((
                0.0,
                3.0 * (height as f64 / num_charts as f64),
            )));
            draw_line(
                &mut render_context,
                width,
                height / num_charts,
                &gated_transients,
                &Color::rgb(1.0, 0.0, 0.5),
            );
            let label = "Smoothed transients";
            draw_text(&mut render_context, label);
            let _ = render_context.restore();

            render_context.transform(Affine::translate((
                0.0,
                4.0 * (height as f64 / num_charts as f64),
            )));
            let test_thresholds = [0.2, 0.1];
            draw_line(
                &mut render_context,
                width,
                height / num_charts,
                frames,
                &Color::rgb(0.0, 0.0, 0.0).with_alpha(0.4),
            );
            for (i, threshold) in test_thresholds.iter().enumerate() {
                let mut inside_transient = false;
                let transient_positions: Vec<f32> = gated_transients
                    .iter()
                    .map(|f| {
                        if !inside_transient && *f > *threshold {
                            inside_transient = true;
                            1.0
                        } else if inside_transient && *f > *threshold {
                            0.0
                        } else {
                            inside_transient = false;
                            0.0
                        }
                    })
                    .collect();

                let base_height = (height / num_charts) as f32;
                let index_ratio = (test_thresholds.len() - i) as f32 / test_thresholds.len() as f32;
                let chart_height = (base_height * index_ratio) as usize;
                draw_transient_lines(
                    &mut render_context,
                    width,
                    chart_height,
                    &transient_positions,
                    &Color::rgb(1.0, 0.0, 0.0),
                );
            }
            let label = "Transient positions";
            draw_text(&mut render_context, label);
            let _ = render_context.restore();

            render_context.finish().unwrap();
            std::mem::drop(render_context);

            bitmap
                .save_to_file(output_file_path)
                .expect("Failed to save image");
        }

        fn draw_text(render_context: &mut CoreGraphicsContext, label: &str) {
            let text = render_context.text();
            let layout = text
                .new_text_layout(label.to_string())
                .default_attribute(TextAttribute::FontSize(20.0))
                .build()
                .unwrap();
            render_context.draw_text(&layout, (0.0, 0.0));
        }

        fn get_gated_transients(transients: &[f32]) -> Vec<f32> {
            let transients: Vec<f32> = transients.iter().map(|f| f.abs()).collect();
            let max_transient = transients
                .iter()
                .max_by(|f1, f2| f1.partial_cmp(f2).unwrap());
            let threshold = max_transient.unwrap() / 20.0;
            let gated_transients: Vec<f32> = transients
                .iter()
                .map(|transient| {
                    if *transient > threshold {
                        *transient
                    } else {
                        0.0
                    }
                })
                .collect();
            gated_transients
        }

        fn draw_transient_lines(
            render_context: &mut CoreGraphicsContext,
            width: usize,
            height: usize,
            frames: &[f32],
            signal_color: &Color,
        ) {
            let len = frames.len() as f64;
            let order = |f1: f32, f2: f32| f1.partial_cmp(&f2).unwrap_or(Ordering::Less);
            let fwidth = width as f64;
            let fheight = height as f64;

            let lines: Vec<Line> = frames
                .iter()
                .enumerate()
                .map(|(i, s)| ((i as f64 / len) * fwidth, *s))
                .filter(|(_i, x)| !(x.is_nan() || x.is_infinite()))
                .filter(|(_i, x)| *x > 0.0)
                .map(|(x, _y)| Line::new(Point::new(x, 0.0), (x, fheight)))
                .collect();
            for line in lines {
                render_context.stroke(&line, signal_color, 3.0);
            }
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
            render_context.stroke(&*path, signal_color, 1.0);
        }
    }
}
