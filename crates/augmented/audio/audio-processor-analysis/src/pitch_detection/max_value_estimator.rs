use super::utils;

/// Implements pitch-detection by finding the maximum bin in a FFT buffer and calculating its
/// corresponding frequency.
///
/// This is not accurate and susceptible to jumping around harmonics.
pub fn estimate_frequency(
    sample_rate: f32,
    fft_real_buffer: impl ExactSizeIterator<Item = f32>,
) -> Option<f32> {
    let bin_count = fft_real_buffer.len();
    let max_bin = utils::maximum_index(fft_real_buffer)?;

    Some(utils::frequency_from_location(
        sample_rate,
        max_bin as f32,
        bin_count,
    ))
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::assert_f_eq;

    use super::*;

    #[test]
    fn test_maximum_estimate_location() {
        let iterator: Vec<f32> = vec![10.0, 30.0, 20.0, 5.0];
        let freq = estimate_frequency(1000.0, iterator.iter().cloned()).unwrap();
        // Maximum bin 1, sample rate is 1000Hz, num bins is 4
        // Estimated freq should be 250Hz
        assert_f_eq!(freq, 250.0);
    }
}
