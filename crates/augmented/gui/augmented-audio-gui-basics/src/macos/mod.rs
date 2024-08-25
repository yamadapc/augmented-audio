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

use cocoa::{appkit::NSView, base::id};
use core_graphics_types::geometry::CGSize;
use foreign_types_shared::{ForeignType, ForeignTypeRef};
use metal::{Device, MTLPixelFormat, MetalDrawableRef, MetalLayer};
use objc::{rc::autoreleasepool, runtime::YES};
use skia_safe::{
    gpu::mtl::BackendContext,
    gpu::{mtl, BackendRenderTarget, DirectContext, SurfaceOrigin},
    scalar, Canvas, ColorType, Size, Surface,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

pub mod colors;
pub mod multitouch;
pub mod prelude;
mod widget;

pub struct SketchContext<'a> {
    canvas: &'a mut Canvas,
    size: Size,
}

impl<'a> SketchContext<'a> {
    pub fn canvas(&mut self) -> &mut Canvas {
        self.canvas
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn widget_draw_context(&mut self) -> widget::DrawContext {
        widget::DrawContext {
            canvas: self.canvas,
        }
    }
}

pub fn sketch<B>(mut builder: B)
where
    B: FnMut(&mut SketchContext) + 'static,
{
    let ev = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new(&ev).unwrap();

    let device = Device::system_default().unwrap();
    let queue = device.new_command_queue();
    let backend = unsafe {
        BackendContext::new(
            device.as_ptr() as mtl::Handle,
            queue.as_ptr() as mtl::Handle,
            std::ptr::null(),
        )
    };
    let mut context = DirectContext::new_metal(&backend, None).unwrap();

    let metal_layer = {
        let draw_size = window.inner_size();
        let layer = MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);

        unsafe {
            use winit::platform::macos::WindowExtMacOS;
            let view = window.ns_view() as id;
            view.setWantsLayer(YES);
            view.setLayer(layer.as_ref() as *const _ as _);
        }

        layer.set_drawable_size(CGSize::new(draw_size.width as f64, draw_size.height as f64));
        layer
    };

    ev.run(move |event, _target, control_flow| {
        autoreleasepool(|| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::CursorMoved {
                        device_id,
                        position,
                        ..
                    } => {
                        log::debug!("{:?} {:?}", device_id, position);
                    }
                    WindowEvent::MouseInput {
                        button,
                        state,
                        device_id,
                        ..
                    } => {
                        log::debug!("{:?} {:?} {:?}", button, state, device_id);
                    }
                    WindowEvent::Touch(touch) => {
                        log::debug!("{:?}", touch);
                    }
                    WindowEvent::Resized(size) => {
                        metal_layer
                            .set_drawable_size(CGSize::new(size.width as f64, size.height as f64));
                        window.request_redraw();
                    }
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    if let Some((drawable, mut surface)) =
                        get_drawable_surface(&metal_layer, &mut context)
                    {
                        let canvas = surface.canvas();

                        let mut sketch_context = SketchContext {
                            canvas,
                            size: {
                                let draw_size = metal_layer.drawable_size();
                                Size::new(draw_size.width as scalar, draw_size.height as scalar)
                            },
                        };
                        builder(&mut sketch_context);

                        surface.flush_and_submit();
                        drop(surface);

                        let command_buffer = queue.new_command_buffer();
                        command_buffer.present_drawable(drawable);
                        command_buffer.commit();
                    }
                }
                _ => {}
            }
        });
    });
}

fn get_drawable_surface<'a>(
    metal_layer: &'a MetalLayer,
    context: &'a mut DirectContext,
) -> Option<(&'a MetalDrawableRef, Surface)> {
    let drawable = metal_layer.next_drawable()?;
    let drawable_size = {
        let size = metal_layer.drawable_size();
        Size::new(size.width as scalar, size.height as scalar)
    };

    let surface = unsafe {
        let texture_info = mtl::TextureInfo::new(drawable.texture().as_ptr() as mtl::Handle);

        let backend_render_target = BackendRenderTarget::new_metal(
            (drawable_size.width as i32, drawable_size.height as i32),
            1,
            &texture_info,
        );

        Surface::from_backend_render_target(
            context,
            &backend_render_target,
            SurfaceOrigin::TopLeft,
            ColorType::BGRA8888,
            None,
            None,
        )?
    };
    Some((drawable, surface))
}
