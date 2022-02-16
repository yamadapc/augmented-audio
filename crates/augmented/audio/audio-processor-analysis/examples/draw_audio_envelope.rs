use std::time::Duration;

use audio_processor_analysis::envelope_follower_processor::EnvelopeFollowerProcessor;
use audio_processor_file::AudioFileProcessor;
use audio_processor_traits::simple_processor::SimpleAudioProcessor;
use audio_processor_traits::{
    audio_buffer, audio_buffer::OwnedAudioBuffer, audio_buffer::VecAudioBuffer, AudioBuffer,
    AudioProcessor, AudioProcessorSettings,
};

fn main() {
    wisual_logger::init_from_env();
    let Options {
        input_file_path,
        output_file_path,
    } = parse_options();

    log::info!("Reading input file input_file={}", input_file_path);
    let settings = AudioProcessorSettings::default();
    let mut input = AudioFileProcessor::from_path(
        audio_garbage_collector::handle(),
        settings,
        &input_file_path,
    )
    .unwrap();
    input.prepare(settings);

    let mut envelope_processor =
        EnvelopeFollowerProcessor::new(Duration::from_millis(10), Duration::from_millis(2));
    envelope_processor.prepare(settings);

    let mut buffer = VecAudioBuffer::new();

    buffer.resize(1, settings.block_size(), 0.0);
    let mut frames = vec![];
    let num_chunks = input.buffer()[0].len() / settings.block_size();
    log::info!("Processing num_chunks={}", num_chunks);
    for _chunk_idx in 0..num_chunks {
        audio_buffer::clear(&mut buffer);
        input.process(&mut buffer);
        for frame in buffer.frames_mut() {
            envelope_processor.s_process(frame[0]);
            frames.push((frame[0], envelope_processor.handle().state()));
        }
    }

    let width = 8000;
    let height = 1000;
    let mut img = image::ImageBuffer::new(width, height);

    log::info!("Rendering chunks num_chunks={}", num_chunks);

    for (index, (sample, envelope)) in frames.iter().enumerate() {
        let x = ((index as f32 / frames.len() as f32) * (width as f32)) as u32;
        let fheight = height as f32;
        let y = ((sample * fheight / 2.0 + fheight / 2.0) as u32)
            .min(height - 1)
            .max(0);

        let pixel = image::Rgb([255u8, 0, 0]);
        img[(x, y)] = pixel;

        let envelope_y = ((fheight - (envelope * fheight + fheight / 2.0)) as u32)
            .min(height - 1)
            .max(0);
        let envelope_pixel = image::Rgb([0, 255u8, 0]);
        img[(x, envelope_y)] = envelope_pixel;
    }

    log::info!("Saving file output_file={}", output_file_path);
    img.save(output_file_path).unwrap();
}

struct Options {
    input_file_path: String,
    output_file_path: String,
}

fn parse_options() -> Options {
    let app = clap::App::new("draw-audio-envelope")
        .arg_from_usage("-i, --input-file=<INPUT_FILE>")
        .arg_from_usage("-o, --output-file=<OUTPUT_FILE>");
    let matches = app.get_matches();

    let input_file_path = matches
        .value_of("input-file")
        .expect("Please provide --input-file")
        .into();
    let output_file_path = matches
        .value_of("output-file")
        .expect("Please provide --output-file")
        .into();

    Options {
        input_file_path,
        output_file_path,
    }
}
