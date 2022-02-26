use rustfft::num_complex::Complex;

use audio_processor_traits::{AudioBuffer, SimpleAudioProcessor};
use dynamic_thresholds::{DynamicThresholds, DynamicThresholdsParams};
use power_change::{PowerOfChangeFrames, PowerOfChangeParams};

use crate::fft_processor::{FftDirection, FftProcessor};

mod dynamic_thresholds;
mod frame_deltas;
mod power_change;

#[cfg(any(test, feature = "visualization"))]
pub mod visualization;

pub struct IterativeTransientDetectionParams {
    /// Size of the FFT windows, defaults to 2048; at 44.1kHz each frame should be ~40ms
    pub fft_size: usize,
    /// If 0.75 is provided, 3/4 of the windows will overlap. Defaults to 3/4
    pub fft_overlap_ratio: f32,
    /// `v` in the paper (equation `5`)
    ///
    /// Defaults to 3 frequency bins or roughly 60Hz at 44.1kHz sample rate
    pub power_of_change_spectral_spread: usize,
    /// `τ` in the paper (equation 7)
    ///
    /// * When calculating dynamic thresholds, controls how many neighbouring time frames are
    ///   considered
    /// * For example, if `threshold_time_spread_factor` is 2.0, a frequency bin and its
    ///   `spectral_spread` neighbours will have to be 2.0 the average of the `time_spread` time
    ///   frames' rate of change for this bin
    ///
    /// Defaults to 3
    pub threshold_time_spread: usize,
    /// `β` - `threshold_time_spread_factor` (equation 7)
    ///
    /// * Internal multiplier of dynamic thresholds
    /// * This nº affects by what factor a frequency bin needs to change in relation to what it has
    ///   changed in neighboring frames to be considered a transient
    /// * Higher nºs means sensitivity is decreased
    ///
    /// Defaults to 2.0
    pub threshold_time_spread_factor: f32,
    /// How many bins should change for a frame to be considered a transient
    ///
    /// Defaults to 1/4 of the fft_size
    pub frequency_bin_change_threshold: usize,
    /// `δ` - `iteration_magnitude_factor` (equation 10)
    ///
    /// * What factor of the magnitude is collected onto the output per iteration
    ///
    /// Defaults to 0.1
    pub iteration_magnitude_factor: f32,
    /// `N` - `iteration_count` (algorithm 1)
    ///
    /// Defaults to 20
    pub iteration_count: usize,
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
            threshold_time_spread_factor: 2.0,
            iteration_magnitude_factor: 0.05,
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
pub fn find_transients<BufferType: AudioBuffer<SampleType = f32>>(
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

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::relative_path;

    use audio_processor_file::{AudioFileProcessor, OutputAudioFileProcessor};
    use audio_processor_traits::{AudioProcessorSettings, OwnedAudioBuffer, VecAudioBuffer};

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
}
