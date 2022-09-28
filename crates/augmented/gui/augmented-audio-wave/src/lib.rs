use audio_processor_traits::AudioBuffer;
use skia_safe::Path;

pub fn draw_audio(
    samples: &impl AudioBuffer<SampleType = f32>,
    (width, height): (f32, f32),
) -> Path {
    let mut path = Path::new();
    path.move_to((0.0, height / 2.0));

    let num_samples = samples.num_samples();
    for (i, frame) in samples.frames().enumerate() {
        let sample = (frame[0] + frame[1]) / 2.0;
        let x = (i as f32 / num_samples as f32) * width;
        let y = sample * height / 2.0 + height / 2.0;
        path.line_to((x, y));
        path.line_to((x, -y));
        // path.line_to((x, -y));
    }
    path.line_to((width, height / 2.0));

    path
}

#[cfg(test)]
mod tests {
    use audio_processor_file::AudioFileProcessor;
    use audio_processor_traits::{
        AudioProcessor, InterleavedAudioBuffer, OwnedAudioBuffer, VecAudioBuffer,
    };

    use super::*;

    #[test]
    fn it_renders_audio_files() {
        let buffer = read_test_buffer();
        // draw_audio(&buffer);
    }

    fn read_test_buffer() -> VecAudioBuffer<f32> {
        let input = audio_processor_testing_helpers::relative_path!("../../../../input-files");
        let input = std::path::Path::new(&input).canonicalize().unwrap();

        let mut input_file = AudioFileProcessor::from_path(
            audio_garbage_collector::handle(),
            Default::default(),
            input.to_str().unwrap(),
        )
        .unwrap();

        input_file.prepare(Default::default());
        let input_file = input_file.buffer();

        let mut buffer = VecAudioBuffer::new();
        buffer.resize(input_file.len(), input_file[0].len(), 0.0);
        for (c, channel) in input_file.iter().enumerate() {
            for (s, sample) in channel.iter().enumerate() {
                buffer.set(c, s, *sample);
            }
        }
        buffer
    }
}
