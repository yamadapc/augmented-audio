use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iced::canvas::{Fill, Frame, Path};
use iced::{Point, Size};

fn run_fill(frame: &mut Frame, path: &Path) {
    frame.fill(path, Fill::default());
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut oscillator = oscillator::Oscillator::sine(44100.0);
    oscillator.set_frequency(440.0);
    let mut output_buffer = Vec::new();
    output_buffer.resize(40000, 0.0);
    for sample in &mut output_buffer {
        *sample = oscillator.get();
        oscillator.tick();
    }
    let mut path = iced::canvas::path::Builder::new();
    for (i, point) in output_buffer.iter().enumerate() {
        path.line_to(Point::new(i as f32, *point));
    }
    let path = path.build();
    let mut frame = Frame::new(Size::new(1000., 1000.));

    c.bench_function("tessellate", |b| {
        b.iter(|| run_fill(black_box(&mut frame), black_box(&path)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
