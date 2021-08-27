use std::ffi::c_void;
use std::thread;
use std::time::Duration;

use cocoa::appkit::{NSEvent, NSView};
use cocoa::base::{id, YES};
use core_graphics_types::geometry::CGSize;
use metal::{
    BinaryArchive, Device, Library, MTLClearColor, MTLLoadAction, MTLPixelFormat, MTLPrimitiveType,
    MTLStoreAction, MetalLayer, RenderPassDescriptor, RenderPassDescriptorRef,
    RenderPipelineDescriptor, RenderPipelineState, TextureRef,
};

use callbacks::*;
use plugin_host_lib::audio_io::AudioIOService;

// Taken from - https://www.nickwilcox.com/blog/recipe_swift_rust_callback/
mod callbacks {
    use std::ffi::c_void;

    #[repr(C)]
    pub struct CompletedCallback {
        userdata: *mut c_void,
        callback: extern "C" fn(*mut c_void, bool),
    }

    unsafe impl Send for CompletedCallback {}

    impl CompletedCallback {
        pub fn succeeded(self) {
            (self.callback)(self.userdata, true);
            std::mem::forget(self)
        }
        pub fn failed(self) {
            (self.callback)(self.userdata, false);
            std::mem::forget(self)
        }
    }

    impl Drop for CompletedCallback {
        fn drop(&mut self) {
            panic!("CompletedCallback must have explicit succeeded or failed call")
        }
    }
}

struct PipelineCtx {
    layer: MetalLayer,
    device: Device,
}
unsafe impl Send for PipelineCtx {}

#[no_mangle]
pub extern "C" fn run_loop(context: *mut c_void) {
    unsafe {
        let device = Device::system_default().expect("no device found");
        let layer = MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        layer.set_presents_with_transaction(false);

        let view = context as id;
        view.setWantsLayer(YES);
        view.setLayer(std::mem::transmute(layer.as_ref()));
        let window = view.window();
        let draw_size = window.frame(); // TODO - use this

        let context = PipelineCtx { layer, device };
        std::thread::spawn(move || {
            let PipelineCtx { layer, device } = context;
            // let pipeline_state = prepare_pipeline_state(&device, &library, &binary_archive);
            let command_queue = device.new_command_queue();

            let mut t = 0.0;

            loop {
                let drawable = match layer.next_drawable() {
                    Some(drawable) => drawable,
                    None => return,
                };

                // Create a new command buffer for each render pass to the current drawable
                let command_buffer = command_queue.new_command_buffer();

                // Obtain a renderPassDescriptor generated from the view's drawable textures.
                let render_pass_descriptor = RenderPassDescriptor::new();
                prepare_render_pass_descriptor(&render_pass_descriptor, drawable.texture(), t);

                // Create a render command encoder.
                let encoder = command_buffer.new_render_command_encoder(&render_pass_descriptor);

                {
                    // let descriptor = RenderPipelineDescriptor::new();
                    // let pipeline_state = device.new_render_pipeline_state(&descriptor).unwrap();
                    // encoder.set_render_pipeline_state(&pipeline_state);
                }
                // Pass in the parameter data.
                // encoder.set_vertex_buffer(0, Some(&vbuf), 0);
                // Draw the triangles which will eventually form the circle.
                // encoder.draw_primitives(MTLPrimitiveType::TriangleStrip, 0, 1080);

                encoder.end_encoding();

                // Schedule a present once the framebuffer is complete using the current drawable.
                command_buffer.present_drawable(&drawable);

                // Finalize rendering here & push the command buffer to the GPU.
                command_buffer.commit();

                t += 0.001f32;

                std::thread::sleep(Duration::from_millis(16));
            }
        });
    }
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

fn prepare_pipeline_state(
    device: &Device,
    library: &Library,
    binary_archive: &BinaryArchive,
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
    pipeline_state_descriptor.set_binary_archives(&[binary_archive]);

    // Add the pipeline descriptor to the binary archive cache.
    binary_archive
        .add_render_pipeline_functions_with_descriptor(&pipeline_state_descriptor)
        .unwrap();

    device
        .new_render_pipeline_state(&pipeline_state_descriptor)
        .unwrap()
}

#[no_mangle]
pub extern "C" fn run_draw() {
    unsafe {}
}

#[no_mangle]
pub extern "C" fn async_operation(callback: CompletedCallback) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        callback.succeeded()
    });
}

pub fn initialize_logger() {
    let _ = wisual_logger::try_init_from_env();
}

pub struct AudioGuiInitialModel {
    host_ids: Vec<String>,
    input_ids: Vec<String>,
    output_ids: Vec<String>,
}

pub fn get_audio_info() -> AudioGuiInitialModel {
    log::info!("get_audio_info called");
    let host_list = AudioIOService::hosts();
    let input_list = AudioIOService::input_devices(None).unwrap();
    let output_list = AudioIOService::output_devices(None).unwrap();

    AudioGuiInitialModel {
        host_ids: host_list,
        input_ids: input_list.into_iter().map(|device| device.name).collect(),
        output_ids: output_list.into_iter().map(|device| device.name).collect(),
    }
}

#[derive(Debug)]
pub struct AudioGuiModel {
    host_id: Option<String>,
    input_id: Option<String>,
    output_id: Option<String>,
}

pub fn set_audio_info(model: AudioGuiModel) {
    log::info!("set_audio_info called with {:?}", model);
}

uniffi_macros::include_scaffolding!("augmented");
