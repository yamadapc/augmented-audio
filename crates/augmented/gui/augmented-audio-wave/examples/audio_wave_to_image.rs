use foreign_types_shared::ForeignType;
use metal::Device;
use skia_safe::gpu::mtl::BackendContext;
use skia_safe::gpu::{mtl, DirectContext, RecordingContext};
use skia_safe::{
    AlphaType, Budgeted, Color4f, ColorType, EncodedImageFormat, ISize, ImageInfo, Surface,
};

use audio_processor_file::AudioFileProcessor;
use audio_processor_traits::{AudioBuffer, AudioProcessor, OwnedAudioBuffer, VecAudioBuffer};
use augmented_audio_wave::audio_wave::spawn_audio_drawer;

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

    let test_buffer = read_test_buffer();
    let mut path_renderer = spawn_audio_drawer(test_buffer.clone());

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
    let context = DirectContext::new_metal(&backend, None).unwrap();

    let draw_size = window.inner_size();
    let mut recording_context = RecordingContext::from(context.clone());
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
        path_renderer.draw(canvas, (draw_size.width as f32, draw_size.height as f32));
    }
    surface.flush_and_submit();

    log::info!("Outputting image...");
    let image = surface.image_snapshot();
    let image = image.encode_to_data(EncodedImageFormat::PNG).unwrap();
    std::fs::write("./output.png", image.as_bytes()).unwrap();
}
