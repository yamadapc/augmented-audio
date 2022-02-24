use audio_processor_traits::{AudioBuffer, AudioProcessor};

/// Implements transient detection:
///
/// * https://www.researchgate.net/profile/Balaji-Thoshkahna/publication/220723752_A_Transient_Detection_Algorithm_for_Audio_Using_Iterative_Analysis_of_STFT/links/0deec52e6331412aed000000/A-Transient-Detection-Algorithm-for-Audio-Using-Iterative-Analysis-of-STFT.pdf
struct STFTTransientDetector {}

impl AudioProcessor for STFTTransientDetector {
    type SampleType = f32;

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_transient_detector() {}
}
