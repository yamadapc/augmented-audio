mod auto_correlation;

use rustfft::num_complex::Complex;

/// Implements YIN estimation
///
/// This implements what is described in "YIN, a fundamental frequency estimator for speech and music",
/// published by Alain de Cheveigne and Hideki Kawahara.
pub struct YINEstimator {}

impl YINEstimator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn estimate_frequency(
        &mut self,
        sample_rate: f32,
        fft_buffer: &[Complex<f32>],
    ) -> Option<f32> {
        None
    }
}
