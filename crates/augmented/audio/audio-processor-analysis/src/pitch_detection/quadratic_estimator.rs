use super::utils;
use rustfft::num_complex::Complex;

/// Similar to `max_value_estimator`, this is also not accurate, but better.
///
/// Estimates the frequency by finding the maximum FFT bin, then performing quadratic interpolation
/// by fitting a second-order polynomial with the 3 points around the maximum bin.
pub fn estimate_frequency(sample_rate: f32, fft_buffer: &[Complex<f32>]) -> Option<f32> {
    let bin_count = fft_buffer.len() / 2;
    let max_bin = utils::maximum_index(fft_buffer.iter().map(|r| r.re).take(bin_count))?;

    let y2 = fft_buffer[max_bin].re;
    let y1 = if max_bin == 0 {
        y2
    } else {
        fft_buffer[max_bin - 1].re
    };
    let y3 = if max_bin == bin_count - 1 {
        y2
    } else {
        fft_buffer[max_bin + 1].re
    };
    let delta = (y3 - y1) / (2.0 * (2.0 * y2 - y1 - y3));
    let location = max_bin as f32 + delta;

    Some(utils::frequency_from_location(
        sample_rate,
        location,
        bin_count,
    ))
}
