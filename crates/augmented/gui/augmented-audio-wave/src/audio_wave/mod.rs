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

use std::sync::mpsc::RecvError;
use std::time::Duration;

use skia_safe::{scalar, Canvas, Color4f, Paint, Path, M44};

use audio_processor_analysis::running_rms_processor::RunningRMSProcessor;
use audio_processor_traits::{AudioBuffer, AudioProcessorSettings, SimpleAudioProcessor};

struct AudioWaveFrame {
    path: Path,
}

impl AudioWaveFrame {
    fn draw(&self, canvas: &mut Canvas, paint: &Paint) {
        canvas.draw_path(&self.path, paint);
    }
}

unsafe impl Send for AudioWaveFrame {}

pub struct PathRendererHandle {
    rx: std::sync::mpsc::Receiver<AudioWaveFrame>,
    closed_rx: std::sync::mpsc::Receiver<()>,
    closed: bool,
}

impl PathRendererHandle {
    pub fn closed(&self) -> bool {
        self.closed
    }

    pub fn wait(&mut self) -> Result<(), RecvError> {
        self.closed_rx.recv()
    }

    pub fn draw(&mut self, canvas: &mut Canvas, size: (f32, f32)) -> bool {
        let mut paint = Paint::new(Color4f::new(1.0, 0.0, 0.0, 1.0), None);
        paint.set_anti_alias(true);
        paint.set_stroke(true);

        let mut has_more = true;

        canvas.save();
        canvas.set_matrix(&M44::scale(size.0 as scalar, size.1 as scalar, 1.0));

        // How many new "pages" to receive per frame
        for _i in 0..10 {
            match self.rx.try_recv() {
                Ok(frame) => {
                    frame.draw(canvas, &paint);
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

        canvas.restore();

        has_more
    }
}

pub fn spawn_audio_drawer(
    mut samples: impl AudioBuffer<SampleType = f32> + Send + 'static,
) -> PathRendererHandle {
    let (tx, rx) = std::sync::mpsc::channel();
    let (closed_tx, closed_rx) = std::sync::mpsc::channel();

    let mut cursor = 0;
    // How many samples to draw per path "page"
    let frame_size: usize = samples.num_samples() / 100;
    let mut state = DrawState::new(1.0);
    state
        .rms_processor
        .s_prepare(AudioProcessorSettings::default());

    std::thread::spawn(move || {
        log::info!("Starting renderer thread");
        loop {
            if cursor >= samples.num_samples() {
                break;
            }

            let (new_state, path) = draw_audio(DrawAudioParams {
                samples: &mut samples,
                bounds: (cursor, cursor + frame_size),
                state,
            });

            let frame = AudioWaveFrame { path };
            state = new_state;
            let result = tx.send(frame);

            if result.is_err() {
                break;
            }
            cursor += frame_size;
        }

        let _ = closed_tx.send(());
    });

    PathRendererHandle {
        rx,
        closed_rx,
        closed: false,
    }
}

pub struct DrawState {
    previous_point: (f32, f32),
    rms_processor: RunningRMSProcessor,
}

impl DrawState {
    pub fn new(height: f32) -> Self {
        Self {
            previous_point: (0.0, height / 2.0),
            rms_processor: RunningRMSProcessor::new_with_duration(
                audio_garbage_collector::handle(),
                Duration::from_millis(1),
            ),
        }
    }
}

struct DrawAudioParams<'a, B: AudioBuffer<SampleType = f32>> {
    samples: &'a mut B,
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
    for (i, frame) in samples
        .frames_mut()
        .enumerate()
        .skip(start)
        .take(end - start)
    {
        state.rms_processor.s_process_frame(frame);
        let rms = state.rms_processor.handle().calculate_rms(0);

        let x = i as f32 / num_samples as f32;

        let y = rms * 0.5 + 0.5;
        path.line_to((x, y));
        let y = -rms * 0.5 + 0.5;
        path.line_to((x, y));

        state.previous_point = (x, y);
    }
    path.line_to((state.previous_point.0, 0.5));

    (state, path)
}
