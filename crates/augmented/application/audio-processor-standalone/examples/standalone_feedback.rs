use audio_processor_traits::{AudioBuffer, AudioProcessor};

struct FeedbackProcessor;

impl AudioProcessor for FeedbackProcessor {
    type SampleType = f32;

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        _data: &mut BufferType,
    ) {
    }
}

fn main() {
    let feedback_processor = FeedbackProcessor {};
    audio_processor_standalone::audio_processor_main(feedback_processor);
}
