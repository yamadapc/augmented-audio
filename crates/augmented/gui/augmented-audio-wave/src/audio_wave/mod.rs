use skia_safe::{scalar, Canvas, Color4f, Paint, Path, M44};

use audio_processor_traits::AudioBuffer;

struct AudioWaveFrame {
    path: Path,
}

impl AudioWaveFrame {
    fn draw(&self, canvas: &mut Canvas) {
        let mut paint = Paint::new(Color4f::new(1.0, 0.0, 0.0, 1.0), None);
        paint.set_anti_alias(true);
        paint.set_stroke(true);
        canvas.draw_path(&self.path, &paint);
    }
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

    pub fn draw(&mut self, canvas: &mut Canvas, size: (f32, f32)) -> bool {
        let mut has_more = true;

        // How many new "pages" to receive per frame
        for _i in 0..10 {
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

        canvas.save();
        canvas.set_matrix(&M44::scale(size.0 as scalar, size.1 as scalar, 1.0));
        for frame in &self.frames {
            frame.draw(canvas);
        }
        canvas.restore();

        has_more
    }
}

pub fn spawn_audio_drawer(
    samples: impl AudioBuffer<SampleType = f32> + Send + 'static,
) -> PathRendererHandle {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut cursor = 0;
    // How many samples to draw per path "page"
    let frame_size: usize = samples.num_samples() / 100;
    let mut state = DrawState::new(1.0);

    std::thread::spawn(move || {
        log::info!("Starting renderer thread");
        loop {
            if cursor >= samples.num_samples() {
                break;
            }
            let (new_state, path) = draw_audio(DrawAudioParams {
                samples: &samples,
                bounds: (cursor, cursor + frame_size),
                state: state.clone(),
            });

            let frame = AudioWaveFrame { path };
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

struct DrawAudioParams<'a, B: AudioBuffer<SampleType = f32>> {
    samples: &'a B,
    bounds: (usize, usize),
    state: DrawState,
}

fn draw_audio<B: AudioBuffer<SampleType = f32>>(
    DrawAudioParams {
        samples,
        bounds: (start, end),
        mut state,
    }: DrawAudioParams<B>,
) -> (DrawState, Path) {
    let mut path = Path::new();

    let num_samples = samples.num_samples();

    path.move_to((state.previous_point.0, 0.5));
    for (i, frame) in samples.frames().enumerate().skip(start).take(end - start) {
        let sample = (frame[0] + frame[1]) / 2.0;

        let x = i as f32 / num_samples as f32;
        let y = sample * 0.5 + 0.5;

        path.line_to((x, y));

        state.previous_point = (x, y);
    }
    path.line_to((state.previous_point.0, 0.5));

    (state, path)
}
