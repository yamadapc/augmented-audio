use std::time::Duration;

use nannou::prelude::*;

use audio_processor_analysis::envelope_follower_processor::EnvelopeFollowerProcessor;
use audio_processor_file::AudioFileProcessor;
use audio_processor_traits::simple_processor::SimpleAudioProcessor;
use audio_processor_traits::{
    audio_buffer, audio_buffer::OwnedAudioBuffer, audio_buffer::VecAudioBuffer, AudioBuffer,
    AudioProcessor, AudioProcessorSettings,
};

struct Model {
    frames: Vec<(f32, f32)>,
    zoom_state: ZoomState,
}

struct ZoomState {
    zoom: f32,
}

fn model(_app: &App) -> Model {
    wisual_logger::init_from_env();
    let clap_app =
        clap::App::new("draw-audio-envelope").arg_from_usage("-i, --input-file=<INPUT_FILE>");
    let matches = clap_app.get_matches();

    let input_file_path = matches
        .value_of("input-file")
        .expect("Please provide --input-file");
    log::info!("Reading input file input_file={}", input_file_path);
    let settings = AudioProcessorSettings::default();
    let mut input =
        AudioFileProcessor::from_path(audio_garbage_collector::handle(), settings, input_file_path)
            .unwrap();
    input.prepare(settings);

    let mut envelope_processor =
        EnvelopeFollowerProcessor::new(Duration::from_millis(10), Duration::from_millis(20));
    envelope_processor.s_prepare(settings);

    let mut buffer = VecAudioBuffer::new();

    buffer.resize(1, settings.block_size(), 0.0);
    let mut frames = vec![];
    let num_chunks = input.buffer()[0].len() / settings.block_size();
    log::info!("Processing num_chunks={}", num_chunks);
    for _chunk_idx in 0..num_chunks {
        audio_buffer::clear(&mut buffer);
        input.process(&mut buffer);
        for frame in buffer.frames_mut() {
            envelope_processor.s_process(frame[0]);
            frames.push((frame[0], envelope_processor.handle().state()));
        }
    }

    log::info!("Rendering chunks num_chunks={}", num_chunks);

    Model {
        frames,
        zoom_state: ZoomState { zoom: 1.0 },
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let rect = app.window_rect();

    draw.background().color(WHITE);

    let mut audio_line = vec![];
    let mut envelope_line = vec![];

    let limit = (1.0 / model.zoom_state.zoom.max(0.1) * model.frames.len() as f32 / 50.0).max(100.0)
        as usize;
    let max_points = rect.w() * 10.0;
    let step_rate = ((limit as f32 / max_points) as usize).max(1);
    for (index, (sample, envelope)) in model
        .frames
        .iter()
        .enumerate()
        .take(limit)
        .step_by(step_rate)
    {
        let x = -rect.w() / 2.0 + (index as f32 / limit as f32) * rect.w();
        let fheight = rect.h();
        let y = (sample * fheight).min(rect.h() / 2.0).max(-rect.h() / 2.0);
        audio_line.push((pt2(x, y), BLACK));

        let envelope_y = (fheight * envelope * 2.0)
            .min(rect.h() / 2.0)
            .max(-rect.h() / 2.0);
        envelope_line.push((pt2(x, envelope_y), RED));
    }

    draw.polyline().weight(1.0).points_colored(audio_line);
    draw.polyline().weight(1.0).points_colored(envelope_line);
    draw.to_frame(app, &frame).unwrap();
}

fn event(app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent {
        simple: Some(WindowEvent::MouseWheel(MouseScrollDelta::PixelDelta(delta), _)),
        ..
    } = event
    {
        let delta_y = delta.y / (app.window_rect().h() / 2.0) as f64;
        model.zoom_state.zoom += delta_y as f32;
        model.zoom_state.zoom = model.zoom_state.zoom.min(10.0).max(0.1)
    }
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}
