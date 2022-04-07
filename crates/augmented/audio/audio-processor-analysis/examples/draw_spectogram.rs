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
use audio_processor_file::AudioFileProcessor;
use audio_processor_traits::{
    audio_buffer, audio_buffer::OwnedAudioBuffer, audio_buffer::VecAudioBuffer, simple_processor,
    AudioProcessorSettings, SimpleAudioProcessor,
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
        .expect("Please provide --output-file");
    log::info!("Reading input file input_file={}", input_file_path);
    let settings = AudioProcessorSettings::default();

    let mut input =
        AudioFileProcessor::from_path(audio_garbage_collector::handle(), settings, input_file_path)
            .unwrap();
    input.prepare(settings);

    let mut fft_processor = FftProcessor::default();
    fft_processor.s_prepare(settings);

    let mut buffer = VecAudioBuffer::new();
    buffer.resize(1, fft_processor.size(), 0.0);

    let mut frames = vec![];
    let num_chunks = input.buffer()[0].len() / fft_processor.size();
    log::info!("Processing num_chunks={}", num_chunks);
    for _chunk_idx in 0..num_chunks {
        audio_buffer::clear(&mut buffer);
        input.process(&mut buffer);
        simple_processor::process_buffer(&mut fft_processor, &mut buffer);
        frames.push(fft_processor.buffer().clone());
    }

    let width = 2000;
    let height = 500;
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

            let ratio = 2.0 * (drawing_magnitude.log10() / 2.0);
            let red = (255.0 * (ratio - 1.0)).floor();
            let blue = 0.0; // (255.0 * (1.0 - ratio)).floor();
            let green = (255.0 * (ratio * 0.5)).floor();
            let pixel = image::Rgb([red as u8, blue as u8, green as u8]);
            img[(x, y)] = pixel;
        }
    }

    log::info!("Saving file output_file={}", output_file_path);
    img.save(output_file_path).unwrap();
}
