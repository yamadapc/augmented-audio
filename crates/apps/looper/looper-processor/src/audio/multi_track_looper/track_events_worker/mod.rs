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

use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Mutex;

use atomic_refcell::AtomicRefCell;
use basedrop::Shared;

use atomic_queue::Queue;
use audio_garbage_collector::make_shared;
use audio_processor_traits::{AudioBuffer, AudioProcessorSettings, VecAudioBuffer};
use augmented_atomics::AtomicF32;
use augmented_audio_wave::spawn_audio_drawer;

use crate::audio::multi_track_looper::long_backoff::LongBackoff;
use crate::LooperId;

enum TrackEventsMessage {
    StoppedRecording {
        looper_id: LooperId,
        settings: Shared<AudioProcessorSettings>,
        looper_clip: Shared<AtomicRefCell<VecAudioBuffer<AtomicF32>>>,
    },
}

pub struct TrackEventsWorker {
    tx: Shared<Queue<TrackEventsMessage>>,
}

impl TrackEventsWorker {
    pub fn new() -> Self {
        let queue = make_shared(Queue::new(10));
        let tx = queue.clone();

        std::thread::spawn(move || {
            let mut backoff = LongBackoff::new();
            let mut audio_drawers = HashMap::new();

            loop {
                if let Some(msg) = queue.pop() {
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
                            let drawer = spawn_audio_drawer(looper_clip_copy);
                            audio_drawers.insert(looper_id, drawer);
                        }
                    }

                    println!("StoppedRecording message");
                    backoff.reset();
                } else {
                    backoff.snooze();
                }
            }
        });

        Self { tx }
    }

    pub fn on_stopped_recording(
        &self,
        looper_id: LooperId,
        settings: Shared<AudioProcessorSettings>,
        looper_clip: Shared<AtomicRefCell<VecAudioBuffer<AtomicF32>>>,
    ) {
        let _ = self.tx.push(TrackEventsMessage::StoppedRecording {
            looper_id,
            looper_clip,
            settings,
        });
    }

    pub fn render_looper_buffer(&self, looper_id: LooperId, ns_view: cocoa::base::id) {
        let layer = LooperBufferLayer::new(looper_id, ns_view);
    }
}

struct TrackEventsRenderer {
    layers: HashMap<LooperId, LooperBufferLayer>,
}

struct LooperBufferLayer {
    looper_id: LooperId,
    surface: skia_safe::Surface,
    layer: metal::MetalLayer,
}

impl LooperBufferLayer {
    fn new(looper_id: LooperId, ns_view: cocoa::base::id) -> Self {
        use cocoa::appkit::NSView;
        use foreign_types_shared::{ForeignType, ForeignTypeRef};
        use metal::{Device, MTLPixelFormat, MetalLayer};

        let device = Device::system_default().unwrap();
        let queue = device.new_command_queue();

        let layer = MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);

        unsafe {
            ns_view.setWantsLayer(cocoa::base::YES);
            ns_view.setLayer(layer.as_ref() as *const _ as _);
        }
        let size = unsafe { ns_view.frame().size };

        layer.set_drawable_size(core_graphics_types::geometry::CGSize::new(
            size.width,
            size.height,
        ));

        let backend = unsafe {
            skia_safe::gpu::mtl::BackendContext::new(
                device.as_ptr() as skia_safe::gpu::mtl::Handle,
                queue.as_ptr() as skia_safe::gpu::mtl::Handle,
                std::ptr::null(),
            )
        };
        let mut context = skia_safe::gpu::DirectContext::new_metal(&backend, None).unwrap();
        let mut recording_context = skia_safe::gpu::RecordingContext::from(context.clone());
        let mut surface = skia_safe::Surface::new_render_target(
            &mut recording_context,
            skia_safe::Budgeted::No,
            &skia_safe::ImageInfo::new(
                skia_safe::ISize::new(size.width as i32, size.height as i32),
                skia_safe::ColorType::BGRA8888,
                skia_safe::AlphaType::Premul,
                None,
            ),
            None,
            None,
            None,
            None,
        )
        .unwrap();

        let canvas = surface.canvas();
        canvas.clear(skia_safe::Color4f::new(0.0, 0.0, 0.0, 1.0));
        surface.flush_and_submit();

        Self {
            looper_id,
            surface,
            layer,
        }
    }
}
