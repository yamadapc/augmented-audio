use audio_processor_traits::{AudioBuffer, AudioProcessor};

struct MonoRightProcessor;

impl AudioProcessor for MonoRightProcessor {
    type SampleType = f32;

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for frame in data.frames_mut() {
            frame[0] = frame[1];
        }
    }
}

fn main() {
    let mono_right_processor = MonoRightProcessor {};
    audio_processor_standalone::audio_processor_main(mono_right_processor);
}
