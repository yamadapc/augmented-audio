use cocoa::appkit::NSView;
use cocoa::base::{id, YES};
use core_graphics_types::geometry::CGSize;
use foreign_types_shared::{ForeignType, ForeignTypeRef};
use metal::{Device, MTLPixelFormat, MetalDrawableRef, MetalLayer};
use objc::rc::autoreleasepool;
use skia_safe::gpu::mtl::BackendContext;
use skia_safe::gpu::{mtl, BackendRenderTarget, DirectContext, SurfaceOrigin};
use skia_safe::{scalar, Canvas, Color4f, ColorType, Paint, Path, Point, Size, Surface};
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::platform::macos::WindowExtMacOS;

use audio_processor_file::AudioFileProcessor;
use audio_processor_traits::{AudioBuffer, AudioProcessor, OwnedAudioBuffer, VecAudioBuffer};
use augmented_audio_wave::{draw_audio, spawn_audio_drawer};

fn read_test_buffer() -> VecAudioBuffer<f32> {
    let input = audio_processor_testing_helpers::relative_path!("../../../../input-files/bass.mp3");
    let input = std::path::Path::new(&input).canonicalize().unwrap();

    let mut input_file = AudioFileProcessor::from_path(
        audio_garbage_collector::handle(),
        Default::default(),
        input.to_str().unwrap(),
    )
    .unwrap();

    input_file.prepare(Default::default());
    let input_file = input_file.buffer();

    let mut buffer = VecAudioBuffer::new();
    buffer.resize(input_file.len(), input_file[0].len(), 0.0);
    for (c, channel) in input_file.iter().enumerate() {
        for (s, sample) in channel.iter().enumerate() {
            buffer.set(c, s, *sample);
        }
    }
    buffer
}

fn main() {
    wisual_logger::init_from_env();
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

    let test_buffer = read_test_buffer();

    let build_layer = || {
        let draw_size = window.inner_size();
        let layer = MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);

        unsafe {
            let view = window.ns_view() as id;
            view.setWantsLayer(YES);
            view.setLayer(layer.as_ref() as *const _ as _);
        }

        layer.set_drawable_size(CGSize::new(draw_size.width as f64, draw_size.height as f64));
        layer
    };
    let metal_layer = build_layer();

    let mut mouse_position = Point::new(0.0, 0.0);
    let draw_size = window.inner_size();
    let mut path_renderer = spawn_audio_drawer(
        test_buffer,
        (draw_size.width as f32, draw_size.height as f32),
    );

    log::info!("Starting to render");
    ev.run(move |event, _target, control_flow| {
        autoreleasepool(|| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::CursorMoved { position, .. } => {
                        mouse_position.x = position.x as scalar;
                        mouse_position.y = position.y as scalar;
                        window.request_redraw();
                    }
                    WindowEvent::Resized(size) => {
                        metal_layer
                            .set_drawable_size(CGSize::new(size.width as f64, size.height as f64));
                        window.request_redraw();
                    }
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    get_drawable_surface(&metal_layer, &mut context).map(
                        |(drawable, mut surface)| {
                            let canvas = surface.canvas();

                            canvas.clear(Color4f::new(0.0, 0.0, 0.0, 1.0));

                            if path_renderer.draw(canvas) {
                                window.request_redraw();
                            }
                            let paint = Paint::new(Color4f::new(1.0, 0.0, 0.0, 1.0), None);
                            canvas.draw_circle(mouse_position, 30.0, &paint);

                            surface.flush_and_submit();
                            drop(surface);

                            let command_buffer = queue.new_command_buffer();
                            command_buffer.present_drawable(drawable);
                            command_buffer.commit();
                        },
                    );
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
    let drawable = metal_layer.next_drawable();
    drawable.map(|drawable| (drawable, read_surface(context, &metal_layer, drawable)))
}

fn read_surface(
    mut context: &mut DirectContext,
    metal_layer: &MetalLayer,
    drawable: &MetalDrawableRef,
) -> Surface {
    let drawable_size = {
        let size = metal_layer.drawable_size();
        Size::new(size.width as scalar, size.height as scalar)
    };

    unsafe {
        let texture_info = mtl::TextureInfo::new(drawable.texture().as_ptr() as mtl::Handle);

        let backend_render_target = BackendRenderTarget::new_metal(
            (drawable_size.width as i32, drawable_size.height as i32),
            1,
            &texture_info,
        );

        Surface::from_backend_render_target(
            &mut context,
            &backend_render_target,
            SurfaceOrigin::TopLeft,
            ColorType::BGRA8888,
            None,
            None,
        )
        .unwrap()
    }
}

fn draw(canvas: &mut Canvas, path: &Path, mouse_position: Point) {
    let mut paint = Paint::new(Color4f::new(1.0, 0.0, 0.0, 1.0), None);
    paint.set_anti_alias(true);
    canvas.draw_path(path, &paint);

    let paint = Paint::new(Color4f::new(1.0, 0.0, 0.0, 1.0), None);
    canvas.draw_circle(mouse_position, 30.0, &paint);
}
