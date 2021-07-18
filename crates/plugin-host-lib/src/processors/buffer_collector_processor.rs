use audio_garbage_collector::Shared;
use audio_processor_traits::audio_buffer::OwnedAudioBuffer;
use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};
use num_traits::{Float, Zero};
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

/// A processor which collects a certain duration of samples onto a shared buffer.
/// This may be used to implement background processing or visualisation
/// TODO - This is a MESS
pub struct BufferCollectorProcessor<InternalBuffer: OwnedAudioBuffer> {
    buffer: Shared<(InternalBuffer, AtomicUsize)>,
    duration: Duration,
}

impl<InternalBuffer: OwnedAudioBuffer> BufferCollectorProcessor<InternalBuffer> {
    pub fn new(buffer: Shared<(InternalBuffer, AtomicUsize)>, duration: Duration) -> Self {
        BufferCollectorProcessor { buffer, duration }
    }

    pub fn buffer(&self) -> &Shared<(InternalBuffer, AtomicUsize)> {
        &self.buffer
    }
}

impl<InternalBuffer: OwnedAudioBuffer + Send> AudioProcessor
    for BufferCollectorProcessor<InternalBuffer>
{
    type SampleType = InternalBuffer::SampleType;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        // TODO - Change basedrop to allow multiple readers depending on sync ; write a safe audio-buffer for this use-case
        // This will cause a crash if the UI thread got the buffer before prepare.
        let buffer = self.buffer.deref() as *const _ as *mut InternalBuffer;
        unsafe {
            (*buffer).resize(
                settings.input_channels(),
                (self.duration.as_secs_f32() * settings.sample_rate() * (1. / 40.)) as usize,
                Self::SampleType::zero(),
            );
        }
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        // There should be a better bulk copy method in audio-processor-traits - see #40
        unsafe {
            let buffer = self.buffer.deref() as *const _ as *mut (InternalBuffer, AtomicUsize);

            let mut cursor = (*buffer).1.load(Ordering::Relaxed);
            let mut accumulator = Self::SampleType::zero();
            for sample_index in 0..data.num_samples() {
                for channel_index in 0..data.num_channels() {
                    let sample = *data.get(channel_index, sample_index);
                    accumulator = accumulator + sample.abs();
                }

                if sample_index % 40 == 0 {
                    (*buffer).0.set(0, cursor, accumulator);
                    accumulator = Self::SampleType::zero();

                    cursor += 1;
                    if cursor >= (*buffer).0.num_samples() {
                        cursor = 0;
                    }
                }
            }
            (*buffer).1.store(cursor, Ordering::Relaxed);
        }
    }
}
