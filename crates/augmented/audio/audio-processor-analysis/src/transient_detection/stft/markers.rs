// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
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
