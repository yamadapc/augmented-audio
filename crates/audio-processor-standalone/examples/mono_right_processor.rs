use audio_processor_traits::{AudioBuffer, AudioProcessor};

struct MonoRightProcessor;

impl AudioProcessor for MonoRightProcessor {
    type SampleType = f32;

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for sample_index in 0..data.num_samples() {
            data.set(0, sample_index, *data.get(1, sample_index));
        }
    }
}

fn main() {
    let mono_right_processor = MonoRightProcessor {};
    audio_processor_standalone::audio_processor_main(mono_right_processor);
}
