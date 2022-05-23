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
use std::time::Duration;

use piet::kurbo::{PathEl, Point, Rect};
use piet::{Color, RenderContext};
use piet_common::Device;

use audio_processor_analysis::envelope_follower_processor::EnvelopeFollowerProcessor;
use audio_processor_file::AudioFileProcessor;
use audio_processor_traits::simple_processor::SimpleAudioProcessor;
use audio_processor_traits::{
    audio_buffer, audio_buffer::OwnedAudioBuffer, audio_buffer::VecAudioBuffer, AudioBuffer,
    AudioProcessorSettings,
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
    envelope_processor.s_prepare(settings);

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

    log::info!("Rendering chunks num_chunks={}", num_chunks);
    draw_audio_envelope(&output_file_path, &mut frames);
}

fn draw_audio_envelope(output_file_path: &str, frames: &mut Vec<(f32, f32)>) {
    if std::env::var("PIET").unwrap_or("false".into()) == "true" {
        draw_audio_envelope_piet(output_file_path, frames);
    } else {
        let width = 800;
        let height = 100;
        let mut img = image::ImageBuffer::new(width, height);

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
}

fn draw_audio_envelope_piet(output_file_path: &str, frames: &Vec<(f32, f32)>) {
    let width = 8000;
    let height = 2000;
    let signal_color = Color::rgb(1.0, 0.0, 0.0);

    let signal: Vec<f64> = frames.iter().map(|(sig, _)| *sig as f64).collect();

    let len = frames.len() as f64;
    let order = |f1: f64, f2: f64| f1.partial_cmp(&f2).unwrap();
    let min_sig = *signal.iter().min_by(|f1, f2| order(**f1, **f2)).unwrap();
    let max_sig = *signal.iter().max_by(|f1, f2| order(**f1, **f2)).unwrap();
    let min_envelope = 0.0;
    let max_envelope = max_sig;
    let fwidth = width as f64;
    let fheight = height as f64;

    let map_sig = |sig| {
        let r = (sig - min_sig) / (max_sig - min_sig);
        r * fheight
    };
    let map_envelope = |env| {
        let r = (env - min_envelope) / (max_envelope - min_envelope);
        fheight / 2.0 - r * fheight
    };

    let mut device = Device::new().unwrap();
    let mut bitmap = device.bitmap_target(width, height, 1.0).unwrap();
    let mut render_context = bitmap.render_context();

    render_context.fill(
        Rect::new(0.0, 0.0, fwidth, fheight),
        &Color::rgb(1.0, 1.0, 1.0),
    );

    let mut signal_path: Vec<PathEl> = signal
        .iter()
        .enumerate()
        .map(|(i, sig)| ((i as f64 / len) * fwidth, map_sig(*sig)))
        .map(|(x, y)| Point::new(x, y))
        .map(PathEl::LineTo)
        .collect();
    signal_path.insert(0, PathEl::MoveTo(Point::new(0.0, fheight / 2.0)));
    signal_path.push(PathEl::LineTo(Point::new(fwidth, fheight / 2.0)));
    render_context.stroke(&*signal_path, &signal_color, 0.5);

    let mut path: Vec<PathEl> = frames
        .iter()
        .map(|(_, envelope)| *envelope as f64)
        .enumerate()
        .map(|(i, envelope)| ((i as f64 / len) * fwidth, map_envelope(envelope)))
        .map(|(x, y)| Point::new(x, y))
        .map(PathEl::LineTo)
        .collect();
    path.insert(0, PathEl::MoveTo(Point::new(0.0, fheight / 2.0)));
    path.push(PathEl::LineTo(Point::new(fwidth, fheight / 2.0)));
    let envelope_color = Color::rgb(0.0, 1.0, 0.0);
    render_context.stroke(&*path, &envelope_color, 0.5);

    render_context.finish().unwrap();
    drop(render_context);

    bitmap
        .save_to_file(format!("{}.piet.png", output_file_path))
        .expect("Failed to save image");
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
