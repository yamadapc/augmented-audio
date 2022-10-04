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

use crate::audio::multi_track_looper::slice_worker::SliceResult;
use crate::audio::processor::handle::LooperState;
use crate::c_api::into_ptr;
use crate::{LooperEngine, LooperHandleThread, LooperId};

pub use self::looper_buffer::*;

#[no_mangle]
pub unsafe extern "C" fn looper_engine__num_loopers(engine: *const LooperEngine) -> usize {
    (*engine).handle().voices().len()
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__record(engine: *const LooperEngine, looper_id: usize) {
    log::info!("looper_engine - Recording {}", looper_id);
    (*engine)
        .handle()
        .toggle_recording(LooperId(looper_id), LooperHandleThread::OtherThread)
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_input_level(engine: *const LooperEngine) -> f32 {
    ((*engine).handle().input_meter_handle().calculate_rms(0)
        + (*engine).handle().input_meter_handle().calculate_rms(1))
        / 2.0
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__play(engine: *const LooperEngine, looper_id: usize) {
    log::info!("looper_engine - Playing {}", looper_id);
    (*engine).handle().toggle_playback(LooperId(looper_id));
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__clear(engine: *const LooperEngine, looper_id: usize) {
    log::info!("looper_engine - Clearing {}", looper_id);
    (*engine).handle().clear(LooperId(looper_id));
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__set_active_looper(
    engine: *const LooperEngine,
    looper_id: usize,
) {
    (*engine).handle().set_active_looper(LooperId(looper_id));
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_looper_num_samples(
    engine: *const LooperEngine,
    looper_id: usize,
) -> usize {
    (*engine).handle().get_num_samples(LooperId(looper_id))
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_looper_state(
    engine: *const LooperEngine,
    looper_id: usize,
) -> LooperState {
    (*engine).handle().get_looper_state(LooperId(looper_id))
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_looper_position(
    engine: *const LooperEngine,
    looper_id: usize,
) -> f32 {
    (*engine).handle().get_position_percent(LooperId(looper_id))
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__start_rendering(
    engine: *mut LooperEngine,
    looper_id: usize,
    layer: *mut metal::CAMetalLayer,
) {
    fn get_drawable_surface<'a>(
        metal_layer: &'a MetalLayer,
        context: &'a mut skia_safe::gpu::DirectContext,
    ) -> Option<(&'a metal::MetalDrawableRef, skia_safe::Surface)> {
        let drawable = metal_layer.next_drawable();
        drawable.map(|drawable| (drawable, read_surface(context, &metal_layer, drawable)))
    }

    fn read_surface(
        mut context: &mut skia_safe::gpu::DirectContext,
        metal_layer: &MetalLayer,
        drawable: &metal::MetalDrawableRef,
    ) -> skia_safe::Surface {
        let drawable_size = {
            let size = metal_layer.drawable_size();
            skia_safe::Size::new(
                size.width as skia_safe::scalar,
                size.height as skia_safe::scalar,
            )
        };

        unsafe {
            let texture_info = skia_safe::gpu::mtl::TextureInfo::new(
                drawable.texture().as_ptr() as skia_safe::gpu::mtl::Handle
            );

            let backend_render_target = skia_safe::gpu::BackendRenderTarget::new_metal(
                (drawable_size.width as i32, drawable_size.height as i32),
                1,
                &texture_info,
            );

            skia_safe::Surface::from_backend_render_target(
                &mut context,
                &backend_render_target,
                skia_safe::gpu::SurfaceOrigin::TopLeft,
                skia_safe::ColorType::BGRA8888,
                None,
                None,
            )
            .unwrap()
        }
    }

    use cocoa::appkit::NSView;
    use foreign_types_shared::{ForeignType, ForeignTypeRef};
    use metal::{Device, MTLPixelFormat, MetalLayer};

    let device = &(*engine).device;
    let queue = device.new_command_queue();
    let backend = unsafe {
        skia_safe::gpu::mtl::BackendContext::new(
            device.as_ptr() as skia_safe::gpu::mtl::Handle,
            queue.as_ptr() as skia_safe::gpu::mtl::Handle,
            std::ptr::null(),
        )
    };
    let mut context = skia_safe::gpu::DirectContext::new_metal(&backend, None).unwrap();

    let layer = MetalLayer::from_ptr(layer);
    let drawable_size = {
        let size = layer.drawable_size();
        skia_safe::Size::new(
            size.width as skia_safe::scalar,
            size.height as skia_safe::scalar,
        )
    };

    let (drawable_ref, mut surface) = get_drawable_surface(&layer, &mut context).unwrap();
    let canvas = surface.canvas();
    canvas.clear(skia_safe::Color4f::new(0.0, 0.0, 0.0, 1.0));
    let mut paint = skia_safe::Paint::new(skia_safe::Color4f::new(1.0, 0.0, 0.0, 1.0), None);
    paint.set_anti_alias(true);
    canvas.draw_circle(
        (drawable_size.width / 2.0, drawable_size.height / 2.0),
        drawable_size.height / 2.0,
        &paint,
    );
    surface.flush_and_submit();

    let command_buffer = queue.new_command_buffer();
    command_buffer.present_drawable(drawable_ref);
    command_buffer.commit();

    // TODO - fix memory leaking?
    std::mem::forget(layer);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__create_metal_layer(
    engine: *mut LooperEngine,
    looper_id: usize,
) -> *mut metal::CAMetalLayer {
    use cocoa::appkit::NSView;
    use foreign_types_shared::{ForeignType, ForeignTypeRef};
    use metal::{Device, MTLPixelFormat, MetalLayer};

    let layer = MetalLayer::new();
    layer.set_device(&(*engine).device);
    layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
    layer.set_presents_with_transaction(false);
    layer.set_drawable_size(core_graphics_types::geometry::CGSize::new(500.0, 500.0));

    let ptr = layer.as_ptr();
    std::mem::forget(layer);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__render_looper_buffer(
    engine: *const LooperEngine,
    looper_id: usize,
    ns_view: cocoa::base::id,
) {
    let handle = (*engine).handle();
    let track_events_worker = handle.track_events_worker();
    track_events_worker.render_looper_buffer(LooperId(looper_id), ns_view);
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__has_looper_buffer(
    engine: *const LooperEngine,
    looper_id: usize,
) -> bool {
    let engine = &(*engine);
    let looper_id = LooperId(looper_id);
    let state = engine.handle().get_looper_state(looper_id);
    state != LooperState::Empty
        && state != LooperState::RecordingScheduled
        && engine.handle().get_looper_buffer(looper_id).is_some()
}

mod looper_buffer {
    use atomic_refcell::AtomicRefCell;
    use basedrop::Shared;

    use audio_processor_traits::{AudioBuffer, VecAudioBuffer};
    use augmented_atomics::AtomicF32;

    use crate::c_api::into_ptr;
    use crate::{LooperEngine, LooperId};

    pub enum LooperBuffer {
        Some {
            inner: Shared<AtomicRefCell<VecAudioBuffer<AtomicF32>>>,
        },
        None,
    }

    #[no_mangle]
    pub unsafe extern "C" fn looper_engine__get_looper_buffer(
        engine: *const LooperEngine,
        looper_id: usize,
    ) -> *mut LooperBuffer {
        let engine = &(*engine);
        into_ptr(
            if let Some(buffer) = engine.handle().get_looper_buffer(LooperId(looper_id)) {
                LooperBuffer::Some { inner: buffer }
            } else {
                LooperBuffer::None
            },
        )
    }

    #[no_mangle]
    pub unsafe extern "C" fn looper_buffer__free(buffer: *mut LooperBuffer) {
        let _ = Box::from_raw(buffer);
    }

    #[no_mangle]
    pub unsafe extern "C" fn looper_buffer__num_samples(buffer: *mut LooperBuffer) -> usize {
        let buffer = &(*buffer);
        match buffer {
            LooperBuffer::Some { inner } => inner.borrow().num_samples(),
            LooperBuffer::None => 0,
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn looper_buffer__get(buffer: *mut LooperBuffer, index: usize) -> f32 {
        let buffer = &(*buffer);
        match buffer {
            LooperBuffer::Some { inner } => {
                let inner = inner.borrow();
                let mut total = 0.0;
                for channel in 0..inner.num_channels() {
                    total += inner.get(channel, index).get();
                }
                total
            }
            LooperBuffer::None => 0.0,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__get_looper_slices(
    engine: *const LooperEngine,
    looper_id: usize,
) -> *mut Option<SliceResult> {
    let engine = &(*engine);
    into_ptr(engine.handle().get_looper_slices(LooperId(looper_id)))
}

#[no_mangle]
pub unsafe extern "C" fn slice_buffer__free(buffer: *mut Option<SliceResult>) {
    let _ = Box::from_raw(buffer);
}

#[no_mangle]
pub unsafe extern "C" fn slice_buffer__length(buffer: *mut Option<SliceResult>) -> usize {
    (*buffer)
        .as_ref()
        .map(|buffer| buffer.markers().len())
        .unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn slice_buffer__get(
    buffer: *mut Option<SliceResult>,
    index: usize,
) -> usize {
    (*buffer)
        .as_ref()
        .and_then(|buffer| buffer.markers().get(index))
        .map(|marker| marker.position_samples)
        .unwrap_or(0)
}
