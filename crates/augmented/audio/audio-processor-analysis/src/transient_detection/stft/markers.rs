use audio_processor_traits::{AudioProcessorSettings, InterleavedAudioBuffer};

use super::{find_transients, IterativeTransientDetectionParams};

#[derive(Clone, Debug)]
pub struct AudioFileMarker {
    pub position_samples: usize,
}

/// Helper for extracting vector of transient markers from buffer
pub fn build_markers(
    settings: &AudioProcessorSettings,
    audio_file_buffer: &mut [f32],
    params: IterativeTransientDetectionParams,
    gate: f32,
) -> Vec<AudioFileMarker> {
    let latency_offset = params.fft_size;

    let transients = find_transients(
        params,
        &mut InterleavedAudioBuffer::new(1, audio_file_buffer),
    );
    let mut peak_detector = crate::peak_detector::PeakDetector::default();
    let attack_mult = crate::peak_detector::calculate_multiplier(settings.sample_rate, 0.1);
    let release_mult = crate::peak_detector::calculate_multiplier(settings.sample_rate, 15.0);
    let transients: Vec<f32> = transients
        .iter()
        .map(|f| {
            peak_detector.accept_frame(attack_mult, release_mult, &[*f]);
            peak_detector.value()
        })
        .collect();

    let markers_from_transients = {
        let mut markers = vec![];
        let mut inside_transient = false;
        for (index, sample) in transients.iter().cloned().enumerate() {
            if sample >= gate && !inside_transient {
                inside_transient = true;
                markers.push(index - latency_offset);
            } else if sample < gate {
                inside_transient = false;
            }
        }
        markers
    };

    markers_from_transients
        .into_iter()
        .map(|position_samples| AudioFileMarker { position_samples })
        .collect()
}
