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

use std::collections::HashMap;

use basedrop::Shared;
use foreign_types_shared::{ForeignType, ForeignTypeRef};
use metal::{CAMetalLayer, CommandQueue, Device, MTLPixelFormat, MetalLayer};
use skia_safe::{
    gpu::{mtl::BackendContext, DirectContext, RecordingContext},
    AlphaType, Budgeted, Color4f, ColorType, ISize, ImageInfo, Paint, Rect, SamplingOptions, Size,
    Surface,
};

use atomic_queue::Queue;
use audio_processor_traits::{AudioBuffer, VecAudioBuffer};
use augmented_audio_wave::spawn_audio_drawer;

use crate::{
    audio::multi_track_looper::track_events_worker::TrackEventsMessage, LooperId,
    MultiTrackLooperHandle,
};

pub struct AudioWaveRenderingController {
    layers: HashMap<LooperId, MetalLayer>,
    drawers: HashMap<LooperId, augmented_audio_wave::PathRendererHandle>,
    surfaces: HashMap<LooperId, Surface>,
    device: Device,
    queue: CommandQueue,
    _backend: BackendContext,
    context: DirectContext,
    recording_context: RecordingContext,
    handle: Shared<MultiTrackLooperHandle>,
    track_events: Shared<Queue<TrackEventsMessage>>,
}

impl AudioWaveRenderingController {
    pub fn new(
        device: &Device,
        handle: Shared<MultiTrackLooperHandle>,
        track_events: Shared<Queue<TrackEventsMessage>>,
    ) -> Self {
        let queue = device.new_command_queue();
        let backend = unsafe {
            BackendContext::new(
                device.as_ptr() as skia_safe::gpu::mtl::Handle,
                queue.as_ptr() as skia_safe::gpu::mtl::Handle,
                std::ptr::null(),
            )
        };
        let context = DirectContext::new_metal(&backend, None).unwrap();
        let recording_context = RecordingContext::from(context.clone());

        Self {
            layers: Default::default(),
            drawers: Default::default(),
            surfaces: Default::default(),
            handle,
            device: device.clone(),
            queue,
            context,
            _backend: backend,
            track_events,
            recording_context,
        }
    }

    pub fn create_layer(&mut self, looper_id: LooperId) -> *mut CAMetalLayer {
        let layer = MetalLayer::new();
        layer.set_device(&self.device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);
        layer.set_drawable_size(core_graphics_types::geometry::CGSize::new(500.0, 500.0));

        self.layers.insert(looper_id, layer.clone());

        let ptr = layer.as_ptr();
        ptr
    }

    pub fn draw(&mut self, looper_id: LooperId) -> Option<()> {
        let layer = self.layers.get(&looper_id)?;
        let drawable_size = layer_size(layer);

        let (drawable_ref, mut surface) = get_drawable_surface(&layer, &mut self.context)?;
        let canvas = surface.canvas();
        canvas.clear(Color4f::new(0.0, 0.0, 0.0, 1.0));

        if !self.surfaces.contains_key(&looper_id) {
            let surface = Surface::new_render_target(
                &mut self.recording_context,
                Budgeted::No,
                &ImageInfo::new(
                    ISize::new(drawable_size.width as i32, drawable_size.height as i32),
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
            self.surfaces.insert(looper_id.clone(), surface);
        }

        if let Some(msg) = self.track_events.pop() {
            match msg {
                TrackEventsMessage::StoppedRecording {
                    looper_id,
                    looper_clip,
                    ..
                } => {
                    let looper_clip = looper_clip.borrow();
                    let looper_clip_copy: Vec<f32> = looper_clip
                        .slice()
                        .iter()
                        .map(|sample| sample.get())
                        .collect();
                    let looper_clip_copy = VecAudioBuffer::new_with(
                        looper_clip_copy,
                        looper_clip.num_channels(),
                        looper_clip.num_samples(),
                    );
                    self.drawers
                        .insert(looper_id, spawn_audio_drawer(looper_clip_copy));
                }
                TrackEventsMessage::ClearedBuffer { looper_id } => {
                    let partial_surface = self.surfaces.get_mut(&looper_id).unwrap();
                    let partial_canvas = partial_surface.canvas();
                    partial_canvas.clear(Color4f::new(0.0, 0.0, 0.0, 1.0));
                }
            }
        }

        let partial_surface = self.surfaces.get_mut(&looper_id).unwrap();
        let partial_canvas = partial_surface.canvas();

        if let Some(drawer) = self.drawers.get_mut(&looper_id) {
            // let _ = drawer.wait();
            drawer.draw(partial_canvas, (drawable_size.width, drawable_size.height));
            partial_surface.flush_and_submit();
        }

        partial_surface.draw(canvas, (0.0, 0.0), SamplingOptions::default(), None);
        let position = self.handle.get_position_percent(looper_id);
        let paint = Paint::new(
            Color4f::new(77.0 / 255.0, 139.0 / 255.0, 49.0 / 255.0, 1.0),
            None,
        );
        let x = position * drawable_size.width;

        let playhead_height = (drawable_size.height)
            * self
                .handle
                .get(looper_id)
                .unwrap()
                .envelope()
                .adsr_envelope
                .volume();
        canvas.draw_rect(
            Rect::new(
                x,
                drawable_size.height - playhead_height,
                2.0,
                playhead_height,
            ),
            &paint,
        );

        surface.flush_and_submit();
        let command_buffer = self.queue.new_command_buffer();
        command_buffer.present_drawable(drawable_ref);
        command_buffer.commit();

        Some(())
    }
}

fn get_drawable_surface<'a>(
    layer: &'a MetalLayer,
    context: &'a mut DirectContext,
) -> Option<(&'a metal::MetalDrawableRef, Surface)> {
    let drawable = layer.next_drawable()?;
    let drawable_size = layer_size(layer);

    let texture_info = unsafe {
        skia_safe::gpu::mtl::TextureInfo::new(
            drawable.texture().as_ptr() as skia_safe::gpu::mtl::Handle
        )
    };

    let backend_render_target = skia_safe::gpu::BackendRenderTarget::new_metal(
        (drawable_size.width as i32, drawable_size.height as i32),
        1,
        &texture_info,
    );

    let surface = Surface::from_backend_render_target(
        context,
        &backend_render_target,
        skia_safe::gpu::SurfaceOrigin::TopLeft,
        ColorType::BGRA8888,
        None,
        None,
    )?;

    Some((drawable, surface))
}

fn layer_size(layer: &MetalLayer) -> Size {
    let size = layer.drawable_size();
    Size::new(
        size.width as skia_safe::scalar,
        size.height as skia_safe::scalar,
    )
}
