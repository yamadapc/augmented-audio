use std::str::FromStr;

use skia_safe::{Canvas, Color4f, Paint, Path, Vector};

use audio_processor_traits::AudioBuffer;

struct AudioWaveFrame {
    offset: usize,
    path: Path,
}
unsafe impl Send for AudioWaveFrame {}

pub struct PathRendererHandle {
    frames: Vec<AudioWaveFrame>,
    rx: std::sync::mpsc::Receiver<AudioWaveFrame>,
    closed: bool,
}

impl PathRendererHandle {
    pub fn closed(&self) -> bool {
        self.closed
    }

    pub fn draw(&mut self, canvas: &mut Canvas) -> bool {
        let mut has_more = true;

        for i in 0..10 {
            match self.rx.try_recv() {
                Ok(frame) => {
                    self.frames.push(frame);
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    has_more = false;
                    if !self.closed {
                        log::info!("Finished rendering");
                        self.closed = true;
                    }
                    break;
                }
                _ => {
                    break;
                }
            }
        }

        for frame in &self.frames {
            canvas.save();

            let mut paint = Paint::new(Color4f::new(1.0, 0.0, 0.0, 1.0), None);
            paint.set_anti_alias(true);
            paint.set_stroke(true);
            paint.set_stroke_width(1.0);
            // canvas.translate(Vector::new(frame.offset as f32, 0.0));
            canvas.draw_path(&frame.path, &paint);

            canvas.restore();
        }

        has_more
    }
}

pub fn spawn_audio_drawer(
    samples: impl AudioBuffer<SampleType = f32> + Send + 'static,
    (width, height): (f32, f32),
) -> PathRendererHandle {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut cursor = 0;
    let frame_size: usize = samples.num_samples() / 100;
    let mut state = DrawState::new(height);
    std::thread::spawn(move || {
        log::info!("Starting renderer thread");
        loop {
            if cursor >= samples.num_samples() {
                break;
            }
            let offset = state.previous_point.0;
            let (new_state, path) = draw_audio(
                &samples,
                (cursor, cursor + frame_size),
                (width, height),
                state.clone(),
            );

            let frame = AudioWaveFrame {
                offset: offset as usize,
                path,
            };
            state = new_state;
            let result = tx.send(frame);

            if result.is_err() {
                break;
            }
            cursor += frame_size;
        }
    });

    PathRendererHandle {
        frames: vec![],
        rx,
        closed: false,
    }
}

#[derive(Clone, Copy)]
pub struct DrawState {
    previous_point: (f32, f32),
}

impl DrawState {
    pub fn new(height: f32) -> Self {
        Self {
            previous_point: (0.0, height / 2.0),
        }
    }
}

pub fn draw_audio(
    samples: &impl AudioBuffer<SampleType = f32>,
    (start, end): (usize, usize),
    (width, height): (f32, f32),
    mut state: DrawState,
) -> (DrawState, Path) {
    let mut path = Path::new();

    let num_samples = samples.num_samples();

    path.move_to((state.previous_point.0, height / 2.0));
    for (i, frame) in samples.frames().enumerate().skip(start).take(end - start) {
        let sample = (frame[0] + frame[1]) / 2.0;

        let x = (i as f32 / num_samples as f32) * width;
        let y = sample * height / 2.0 + height / 2.0;

        path.line_to((x, y));

        state.previous_point = (x, y);
    }
    path.line_to((state.previous_point.0, height / 2.0));

    (state, path)
}

#[cfg(test)]
mod tests {
    use audio_processor_file::AudioFileProcessor;
    use audio_processor_traits::{
        AudioProcessor, InterleavedAudioBuffer, OwnedAudioBuffer, VecAudioBuffer,
    };

    use super::*;

    #[test]
    fn it_renders_audio_files() {
        let buffer = read_test_buffer();
        // draw_audio(&buffer);
    }

    fn read_test_buffer() -> VecAudioBuffer<f32> {
        let input = audio_processor_testing_helpers::relative_path!("../../../../input-files");
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
}
