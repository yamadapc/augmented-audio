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
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use basedrop::Shared;

use audio_garbage_collector::make_shared;
use audio_processor_traits::{AudioBuffer, AudioContext, AudioProcessor};

pub struct EnvelopeHandle {
    pub adsr_envelope: augmented_adsr_envelope::Envelope,
    pub enabled: AtomicBool,
}

pub struct EnvelopeProcessor {
    pub handle: Shared<EnvelopeHandle>,
}

impl Default for EnvelopeProcessor {
    fn default() -> Self {
        let envelope = augmented_adsr_envelope::Envelope::new();
        envelope.set_attack(Duration::from_secs_f32(0.0));
        envelope.set_decay(Duration::from_secs_f32(0.0));
        envelope.set_sustain(1.0);
        envelope.set_release(Duration::from_secs_f32(1_000_000.0));
        Self {
            handle: make_shared(EnvelopeHandle {
                adsr_envelope: envelope,
                enabled: AtomicBool::new(false),
            }),
        }
    }
}

impl AudioProcessor for EnvelopeProcessor {
    type SampleType = f32;

    fn prepare(&mut self, context: &mut AudioContext) {
        self.handle
            .adsr_envelope
            .set_sample_rate(context.settings.sample_rate());
    }

    fn process(&mut self, _context: &mut AudioContext, data: &mut AudioBuffer<f32>) {
        if !self.handle.enabled.load(Ordering::Relaxed) {
            return;
        }
        for sample_num in 0..data.num_samples() {
            let volume = self.handle.adsr_envelope.volume();
            for channel_num in 0..data.num_channels() {
                let sample = data.get_mut(channel_num, sample_num);
                *sample *= volume;
            }
            self.handle.adsr_envelope.tick();
        }
    }
}
