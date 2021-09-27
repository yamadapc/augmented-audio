use cocoa::appkit::{NSEvent, NSView};
use cocoa::base::{id, YES};
use metal::{
    BinaryArchive, Device, Library, MTLClearColor, MTLLoadAction, MTLPixelFormat, MTLPrimitiveType,
    MTLResourceOptions, MTLStoreAction, MetalLayer, RenderPassDescriptor, RenderPassDescriptorRef,
    RenderPipelineDescriptor, RenderPipelineState, TextureRef, URL,
};
use std::ffi::c_void;
use std::mem::size_of;
use std::time::Duration;

#[repr(C)]
#[derive(Debug)]
pub struct position(cty::c_float, cty::c_float);

#[repr(C)]
#[derive(Debug)]
pub struct color(cty::c_float, cty::c_float, cty::c_float);

#[repr(C)]
#[derive(Debug)]
pub struct AAPLVertex {
    p: position,
    c: color,
}

fn prepare_render_pass_descriptor(
    descriptor: &RenderPassDescriptorRef,
    texture: &TextureRef,
    t: f32,
) {
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();

    color_attachment.set_texture(Some(texture));
    color_attachment.set_load_action(MTLLoadAction::Clear);
    // Setting a background color
    color_attachment.set_clear_color(MTLClearColor::new(t.sin() as f64, 0.5, 0.8, 1.0));
    color_attachment.set_store_action(MTLStoreAction::Store);
}

struct PipelineCtx {
    layer: MetalLayer,
    device: Device,
}
unsafe impl Send for PipelineCtx {}

#[no_mangle]
pub extern "C" fn chart_handler_on_chart_view(ns_view: *mut c_void) {
    log::info!("Setting-up Metal loop");

    unsafe {
        let device = Device::system_default().expect("no device found");
        let layer = MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);

        let view = ns_view as id;
        view.setWantsLayer(YES);
        view.setLayer(std::mem::transmute(layer.as_ref()));
        let window = view.window();
        let draw_size = window.frame(); // TODO - use this

        let context = PipelineCtx { layer, device };
        std::thread::spawn(move || {
            let PipelineCtx { layer, device } = context;

            let metal_lib_path = augmented::gui::macos_bundle_resources::get_path(
                "beijaflor-io.Recording-Buddy",
                "default.metallib",
                None,
                None,
            )
            .unwrap();
            let metal_lib_path = metal_lib_path
                .to_str()
                .unwrap()
                .to_string()
                .replace("file://", "")
                .replace("%20", " ");
            log::info!("Found library at {:?}", metal_lib_path);
            let metal_lib = device.new_library_with_file(metal_lib_path).unwrap();
            let vbuf = {
                let vertex_data = create_vertex_points_for_circle();
                let vertex_data = vertex_data.as_slice();

                device.new_buffer_with_data(
                    vertex_data.as_ptr() as *const _,
                    (vertex_data.len() * size_of::<AAPLVertex>()) as u64,
                    MTLResourceOptions::CPUCacheModeDefaultCache
                        | MTLResourceOptions::StorageModeManaged,
                )
            };

            // TODO - this can't be based on the project path

            let binary_archive: Option<BinaryArchive> = {
                #[cfg(debug_assertions)]
                {
                    None
                }
                #[cfg(not(debug_assertions))]
                {
                    let binary_archive_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                        .join("crates/apps/recording_buddy/binary_archive.metallib");
                    let binary_archive_url =
                        URL::new_with_string(&format!("file://{}", binary_archive_path.display()));
                    let binary_archive_descriptor = metal::BinaryArchiveDescriptor::new();
                    if binary_archive_path.exists() {
                        binary_archive_descriptor.set_url(&binary_archive_url);
                    }
                    Some(
                        device
                            .new_binary_archive_with_descriptor(&binary_archive_descriptor)
                            .unwrap(),
                    )
                }
            };

            let pipeline_state =
                prepare_pipeline_state(&device, &metal_lib, binary_archive.as_ref());
            let command_queue = device.new_command_queue();

            let mut t = 0.0;

            while let Some(drawable) = layer.next_drawable() {
                // Create a new command buffer for each render pass to the current drawable
                let command_buffer = command_queue.new_command_buffer();

                // Obtain a renderPassDescriptor generated from the view's drawable textures.
                let render_pass_descriptor = RenderPassDescriptor::new();
                prepare_render_pass_descriptor(&render_pass_descriptor, drawable.texture(), t);

                // Create a render command encoder.
                let encoder = command_buffer.new_render_command_encoder(&render_pass_descriptor);

                {
                    let descriptor = RenderPipelineDescriptor::new();
                    encoder.set_render_pipeline_state(&pipeline_state);
                }
                // Pass in the parameter data.
                encoder.set_vertex_buffer(0, Some(&vbuf), 0);
                // Draw the triangles which will eventually form the circle.
                encoder.draw_primitives(MTLPrimitiveType::TriangleStrip, 0, 1080);

                encoder.end_encoding();

                // Schedule a present once the framebuffer is complete using the current drawable.
                command_buffer.present_drawable(&drawable);

                // Finalize rendering here & push the command buffer to the GPU.
                command_buffer.commit();

                t += 0.001f32;

                std::thread::sleep(Duration::from_millis(16));
            }

            log::info!("Rendering loop exiting");
        });
    }
}

fn prepare_pipeline_state(
    device: &Device,
    library: &Library,
    binary_archive: Option<&BinaryArchive>,
) -> RenderPipelineState {
    let vert = library.get_function("vs", None).unwrap();
    let frag = library.get_function("ps", None).unwrap();

    let pipeline_state_descriptor = RenderPipelineDescriptor::new();
    pipeline_state_descriptor.set_vertex_function(Some(&vert));
    pipeline_state_descriptor.set_fragment_function(Some(&frag));
    pipeline_state_descriptor
        .color_attachments()
        .object_at(0)
        .unwrap()
        .set_pixel_format(MTLPixelFormat::BGRA8Unorm);
    // Set the binary archives to search for a cached pipeline in.

    #[cfg(not(debug_assertions))]
    {
        pipeline_state_descriptor.set_binary_archives(&[binary_archive.unwrap()]);
        // Add the pipeline descriptor to the binary archive cache.
        binary_archive
            .unwrap()
            .add_render_pipeline_functions_with_descriptor(&pipeline_state_descriptor)
            .unwrap();
    }

    device
        .new_render_pipeline_state(&pipeline_state_descriptor)
        .unwrap()
}

fn create_vertex_points_for_circle() -> Vec<AAPLVertex> {
    let mut v: Vec<AAPLVertex> = Vec::new();
    let origin_x: f32 = 0.0;
    let origin_y: f32 = 0.0;

    // Size of the circle
    let circle_size = 0.8f32;

    for i in 0..720 {
        let y = i as f32;
        // Get the X co-ordinate of each point on the perimeter of circle
        let position_x: f32 = y.to_radians().cos() * 100.0;
        let position_x: f32 = position_x.trunc() / 100.0;
        // Set the size of the circle
        let position_x: f32 = position_x * circle_size;
        // Get the Y co-ordinate of each point on the perimeter of circle
        let position_y: f32 = y.to_radians().sin() * 100.0;
        let position_y: f32 = position_y.trunc() / 100.0;
        // Set the size of the circle
        let position_y: f32 = position_y * circle_size;

        v.push(AAPLVertex {
            p: position(position_x, position_y),
            c: color(0.7, 0.3, 0.5),
        });

        if (i + 1) % 2 == 0 {
            // For each two points on perimeter, push one point of origin
            v.push(AAPLVertex {
                p: position(origin_x, origin_y),
                c: color(0.2, 0.7, 0.4),
            });
        }
    }

    v
}
