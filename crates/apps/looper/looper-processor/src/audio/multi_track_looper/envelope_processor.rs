use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use basedrop::Shared;

use audio_garbage_collector::make_shared;
use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};

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

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.handle
            .adsr_envelope
            .set_sample_rate(settings.sample_rate());
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        if self.handle.enabled.load(Ordering::Relaxed) {
            for frame in data.frames_mut() {
                let volume = self.handle.adsr_envelope.volume();
                for sample in frame {
                    *sample *= volume;
                }
                self.handle.adsr_envelope.tick();
            }
        }
    }
}
