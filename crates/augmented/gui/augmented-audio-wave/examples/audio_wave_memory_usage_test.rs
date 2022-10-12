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

use skia_safe::gpu::mtl::BackendContext;
use skia_safe::gpu::{mtl, DirectContext, RecordingContext};
use skia_safe::{AlphaType, Budgeted, ColorType, ISize, ImageInfo, Size, Surface};
use std::time::Duration;

use foreign_types_shared::ForeignType;
use metal::Device;

fn main() {
    wisual_logger::init_from_env();

    let draw_size = Size::new(500.0, 500.0);
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

    let mut recording_context = RecordingContext::from(context.clone());
    let surfaces: Vec<Surface> = (0..500)
        .map(|_i| {
            Surface::new_render_target(
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
            .unwrap()
        })
        .collect();

    log::info!("Created 100 surfaces of 500x500 pixels");
    std::thread::sleep(Duration::from_secs(60));
    drop(surfaces);
}
