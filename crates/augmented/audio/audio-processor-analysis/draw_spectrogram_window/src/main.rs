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

use audio_processor_analysis::fft_processor::FftProcessor;
use audio_processor_file::AudioFileProcessor;
use audio_processor_traits::{
    audio_buffer, audio_buffer::OwnedAudioBuffer, audio_buffer::VecAudioBuffer, simple_processor,
    AudioProcessor, AudioProcessorSettings, SimpleAudioProcessor,
};
use augmented_audio_gui_basics::prelude::skia_safe::{
    self, runtime_effect::ChildPtr, AlphaType, Color4f, ColorType, Data, FilterMode, ISize,
    MipmapMode, Paint, SamplingOptions,
};

fn main() {
    wisual_logger::init_from_env();
    let app = clap::App::new("draw-spectrogram").arg_from_usage("-i, --input-file=<INPUT_FILE>");
    let matches = app.get_matches();

    let input_file_path = matches
        .value_of("input-file")
        .expect("Please provide --input-file");
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

    // let mut img = image::ImageBuffer::new(width, height);

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

    let bitmap = {
        let mut b = skia_safe::Bitmap::new();
        let success = b.try_alloc_pixels_flags(&skia_safe::ImageInfo::new(
            ISize::new(
                magnitude_frames.len() as i32,
                magnitude_frames[0].len() as i32,
            ),
            ColorType::Gray8,
            AlphaType::Premul,
            None,
        ));
        assert!(success);

        let num_bytes = b.bytes_per_pixel() * b.width() as usize * b.height() as usize;
        let pixels: &mut [u8] =
            unsafe { std::slice::from_raw_parts_mut(b.pixels() as *mut u8, num_bytes) };

        for (i, frame) in magnitude_frames.iter().enumerate() {
            for (j, sample) in frame.iter().enumerate() {
                let index = i * frame.len() + j;
                assert!(index < num_bytes);
                pixels[index] = (*sample * 256.0) as u8;
            }
        }

        b
    };

    augmented_audio_gui_basics::sketch(move |ctx| {
        let size = ctx.size();
        let canvas = ctx.canvas();

        canvas.clear(Color4f::new(0.0, 0.0, 0.0, 1.0));
        log::info!("render!");
        let bitmap = bitmap
            .to_shader(
                None,
                SamplingOptions::new(FilterMode::Linear, MipmapMode::Linear),
                None,
            )
            .unwrap();
        let effect = skia_safe::RuntimeEffect::make_for_shader(
            r#"
uniform shader fft;
uniform vec2 fft_size;
uniform vec2 canvas_size;

vec4 jet (float x) {
  const float e0 = 0.0;
  const vec4 v0 = vec4(0,0,0.5137254901960784,1);
  const float e1 = 0.125;
  const vec4 v1 = vec4(0,0.23529411764705882,0.6666666666666666,1);
  const float e2 = 0.375;
  const vec4 v2 = vec4(0.0196078431372549,1,1,1);
  const float e3 = 0.625;
  const vec4 v3 = vec4(1,1,0,1);
  const float e4 = 0.875;
  const vec4 v4 = vec4(0.9803921568627451,0,0,1);
  const float e5 = 1.0;
  const vec4 v5 = vec4(0.5019607843137255,0,0,1);
  float a0 = smoothstep(e0,e1,x);
  float a1 = smoothstep(e1,e2,x);
  float a2 = smoothstep(e2,e3,x);
  float a3 = smoothstep(e3,e4,x);
  float a4 = smoothstep(e4,e5,x);
  return max(mix(v0,v1,a0)*step(e0,x)*step(x,e1),
    max(mix(v1,v2,a1)*step(e1,x)*step(x,e2),
    max(mix(v2,v3,a2)*step(e2,x)*step(x,e3),
    max(mix(v3,v4,a3)*step(e3,x)*step(x,e4),mix(v4,v5,a4)*step(e4,x)*step(x,e5)
  ))));
}

vec4 main(float2 coord) {
    vec4 c = fft.eval(vec2(coord.x / canvas_size.x * fft_size.x, coord.y / canvas_size.y * fft_size.y));
    vec4 result = jet(c.r * 100);
    result.a = c.r;
    return result;
}
            "#,
            None,
        )
        .unwrap();

        let data = [
            magnitude_frames.len() as f32,
            magnitude_frames[0].len() as f32,
            size.width,
            size.height,
        ];
        let data = bytemuck::cast_slice(&data);
        assert!(data.len() > 4);
        let uniforms = Data::new_copy(data);
        let shader = effect
            .make_shader(uniforms, &[ChildPtr::Shader(bitmap)], None)
            .unwrap();
        let mut paint = Paint::default();
        paint.set_shader(shader);
        canvas.draw_paint(&paint);
    });
}
