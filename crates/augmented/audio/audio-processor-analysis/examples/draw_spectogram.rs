use audio_processor_file::AudioFileProcessor;
use audio_processor_traits::{
    audio_buffer::OwnedAudioBuffer, audio_buffer::VecAudioBuffer, AudioBuffer, AudioProcessor,
    AudioProcessorSettings, Zero,
};

use audio_processor_analysis::fft_processor::FftProcessor;

fn main() {
    wisual_logger::init_from_env();
    let app = clap::App::new("draw-spectogram")
        .arg_from_usage("-i, --input-file=<INPUT_FILE>")
        .arg_from_usage("-o, --output-file=<OUTPUT_FILE>");
    let matches = app.get_matches();

    let input_file_path = matches
        .value_of("input-file")
        .expect("Please provide --input-file");
    let output_file_path = matches
        .value_of("output-file")
        .expect("Please provide --input-file");
    log::info!("Reading input file input_file={}", input_file_path);
    let mut input = AudioFileProcessor::from_path(
        audio_garbage_collector::handle(),
        AudioProcessorSettings::default(),
        input_file_path,
    )
    .unwrap();
    input.prepare(AudioProcessorSettings::default());

    let mut fft_processor = FftProcessor::default();
    fft_processor.prepare(AudioProcessorSettings::default());

    let mut buffer = VecAudioBuffer::new();
    buffer.resize(1, fft_processor.size(), 0.0);

    let mut frames = vec![];
    let num_chunks = input.buffer()[0].len() / fft_processor.size();
    log::info!("Processing num_chunks={}", num_chunks);
    for _chunk_idx in 0..num_chunks {
        AudioBuffer::<SampleType = f32>::clear(&mut buffer);
        input.process(&mut buffer);
        fft_processor.process(&mut buffer);
        frames.push(fft_processor.buffer().clone());
    }

    let width = 2000;
    let height = 2000;
    let mut img = image::ImageBuffer::new(width, height);

    log::info!("Rendering chunks num_chunks={}", num_chunks);
    let magnitude_frames: Vec<Vec<f32>> = frames
        .iter()
        .map(|frame| {
            let mut magnitudes: Vec<f32> = frame.iter().map(|c| c.norm()).collect();
            magnitudes.reverse();
            magnitudes
                .iter()
                .take(magnitudes.len() / 4)
                .copied()
                .collect()
        })
        .collect();

    for x in 0..width {
        let x_perc = x as f32 / width as f32;
        let frame_idx_f = x_perc * magnitude_frames.len() as f32;
        let frame_idx = frame_idx_f as usize;
        let magnitudes = &magnitude_frames[frame_idx];
        let next_magnitudes = if frame_idx + 1 < magnitude_frames.len() {
            Some(&magnitude_frames[frame_idx + 1])
        } else {
            None
        };
        let delta = frame_idx_f - frame_idx as f32;

        for y in 0..height {
            let y_perc = y as f32 / height as f32;
            let y_bin_idx_f = y_perc * (magnitudes.len() / 4) as f32;
            let y_bin_idx = y_bin_idx_f as usize;
            let y_delta = y_bin_idx_f - y_bin_idx as f32;

            let mut drawing_magnitude = 0.0f32;
            let mut add_y = |idx, y_perc| {
                let magnitude = magnitudes[idx];
                drawing_magnitude += y_perc * delta * magnitude;
                if let Some(next_magnitudes) = next_magnitudes {
                    drawing_magnitude += y_perc * (1.0 - delta) * next_magnitudes[idx];
                }
            };
            add_y(y_bin_idx, y_delta);
            if y_bin_idx + 1 < magnitudes.len() {
                add_y(y_bin_idx + 1, 1.0 - y_delta);
            }

            let red_f = drawing_magnitude * 255.0 / 20.0;
            let pixel = image::Rgb([red_f as u8, (red_f * 0.6) as u8, 0]);
            img[(x, y)] = pixel;
        }
    }

    log::info!("Saving file output_file={}", output_file_path);
    img.save(output_file_path).unwrap();
}
