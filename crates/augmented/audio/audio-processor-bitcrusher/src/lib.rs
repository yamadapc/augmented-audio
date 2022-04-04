use audio_garbage_collector::{make_shared, Shared};
use audio_processor_traits::parameters::{
    make_handle_ref, AudioProcessorHandleProvider, AudioProcessorHandleRef,
};
use audio_processor_traits::{AtomicF32, AudioBuffer, AudioProcessor, AudioProcessorSettings};
pub use generic_handle::BitCrusherHandleRef;

mod generic_handle;

pub struct BitCrusherHandle {
    sample_rate: AtomicF32,
    bit_rate: AtomicF32,
}

impl BitCrusherHandle {
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate.get()
    }

    pub fn bit_rate(&self) -> f32 {
        self.bit_rate.get()
    }

    pub fn set_sample_rate(&self, sample_rate: f32) {
        self.sample_rate.set(sample_rate);
    }

    pub fn set_bit_rate(&self, bit_rate: f32) {
        self.bit_rate.set(bit_rate);
    }
}

impl Default for BitCrusherHandle {
    fn default() -> Self {
        Self {
            sample_rate: AtomicF32::new(44100.0),
            bit_rate: AtomicF32::new(44100.0),
        }
    }
}

pub struct BitCrusherProcessor {
    handle: Shared<BitCrusherHandle>,
}

impl AudioProcessorHandleProvider for BitCrusherProcessor {
    fn generic_handle(&self) -> AudioProcessorHandleRef {
        make_handle_ref(BitCrusherHandleRef::new(self.handle.clone()))
    }
}

impl BitCrusherProcessor {
    pub fn new(handle: Shared<BitCrusherHandle>) -> Self {
        BitCrusherProcessor { handle }
    }

    pub fn handle(&self) -> &Shared<BitCrusherHandle> {
        &self.handle
    }

    fn step_size(&self) -> usize {
        (self.handle.sample_rate() / self.handle.bit_rate()) as usize
    }
}

impl Default for BitCrusherProcessor {
    fn default() -> Self {
        Self::new(make_shared(BitCrusherHandle::default()))
    }
}

impl AudioProcessor for BitCrusherProcessor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.handle.set_sample_rate(settings.sample_rate());
        if (self.handle.sample_rate() - self.handle.bit_rate()).abs() < f32::EPSILON {
            self.handle.set_bit_rate(settings.sample_rate());
        }
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        let step_size = self.step_size();

        let mut sample_index = 0;
        let buffer_size = data.num_samples();

        while sample_index < buffer_size {
            let first_index = sample_index;
            let limit_index = (sample_index + step_size).min(buffer_size);

            while sample_index < limit_index {
                for channel_index in 0..data.num_channels() {
                    let value = *data.get(channel_index, first_index);
                    data.set(channel_index, sample_index, value);
                }
                sample_index += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use audio_processor_testing_helpers::sine_buffer;

    use audio_processor_traits::VecAudioBuffer;

    use super::*;

    #[test]
    fn test_construct_bitcrusher() {
        let _processor = BitCrusherProcessor::default();
    }

    #[test]
    fn test_step_size_is_1_on_passthrough() {
        let settings = AudioProcessorSettings::default();
        let mut processor = BitCrusherProcessor::default();
        processor.prepare(settings);
        assert_eq!(processor.step_size(), 1);
    }

    #[test]
    fn test_step_size_is_2_on_lower_bitrate() {
        let settings = AudioProcessorSettings::default();
        let mut processor = BitCrusherProcessor::default();
        processor.prepare(settings);
        processor
            .handle()
            .set_bit_rate(settings.sample_rate() / 2.0);
        assert_eq!(processor.step_size(), 2);
    }

    #[test]
    fn test_passthrough_bitcrusher() {
        let settings = AudioProcessorSettings::default();
        let mut processor = BitCrusherProcessor::default();
        processor.prepare(settings);

        let input_buffer = VecAudioBuffer::from(sine_buffer(
            settings.sample_rate(),
            440.0,
            Duration::from_millis(10),
        ));
        let mut output_buffer = input_buffer.clone();
        processor.process(&mut output_buffer);

        assert_eq!(input_buffer.slice(), output_buffer.slice());
    }
}
