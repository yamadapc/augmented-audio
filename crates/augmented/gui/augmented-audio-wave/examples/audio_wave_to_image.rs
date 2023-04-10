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

use foreign_types_shared::ForeignType;
use metal::Device;
use skia_safe::gpu::mtl::BackendContext;
use skia_safe::gpu::{mtl, DirectContext, RecordingContext};
use skia_safe::{
    AlphaType, Budgeted, Color4f, ColorType, EncodedImageFormat, ISize, ImageInfo, Size, Surface,
};

use audio_processor_file::AudioFileProcessor;
use audio_processor_traits::{AudioBuffer, AudioContext, AudioProcessor};
use augmented_audio_wave::spawn_audio_drawer;

fn read_test_buffer() -> AudioBuffer<f32> {
    let input = audio_processor_testing_helpers::relative_path!("../../../../input-files/bass.mp3");
    let input = std::path::Path::new(&input).canonicalize().unwrap();

    let mut input_file = AudioFileProcessor::from_path(
        audio_garbage_collector::handle(),
        Default::default(),
        input.to_str().unwrap(),
    )
    .unwrap();

    let mut context = AudioContext::default();
    input_file.prepare(&mut context);
    let input_file = input_file.buffer();

    let mut buffer = AudioBuffer::empty();
    buffer.resize(input_file.len(), input_file[0].len());
    for (c, channel) in input_file.iter().enumerate() {
        for (s, sample) in channel.iter().enumerate() {
            buffer.set(c, s, *sample);
        }
    }
    buffer
}

fn main() {
    wisual_logger::init_from_env();

    let test_buffer = read_test_buffer();
    let mut path_renderer = spawn_audio_drawer(test_buffer);

    let device = Device::system_default().unwrap();
    let queue = device.new_command_queue();
    let backend = unsafe {
        BackendContext::new(
            device.as_ptr() as mtl::Handle,
            queue.as_ptr() as mtl::Handle,
            std::ptr::null(),
        )
    };
    let context = DirectContext::new_metal(&backend, None).unwrap();

    let draw_size = Size::new(500.0, 500.0);
    let mut recording_context = RecordingContext::from(context);
    let mut surface = Surface::new_render_target(
        &mut recording_context,
        Budgeted::No,
        &ImageInfo::new(
            ISize::new(draw_size.width as i32, draw_size.height as i32),
            ColorType::BGRA8888,
            AlphaType::Premul,
            None,
        ),
        None,
        None,
        None,
        None,
    )
    .unwrap();

    log::info!("Starting to render...");
    path_renderer.wait().unwrap();
    let canvas = surface.canvas();
    canvas.clear(Color4f::new(0.0, 0.0, 0.0, 1.0));
    while !path_renderer.closed() {
        path_renderer.draw(canvas, (draw_size.width, draw_size.height));
    }
    surface.flush_and_submit();

    log::info!("Outputting image...");
    let image = surface.image_snapshot();
    let image = image.encode_to_data(EncodedImageFormat::PNG).unwrap();
    std::fs::write("./output.png", image.as_bytes()).unwrap();
}
