use criterion::black_box;
/// This is an example here just to get some profiling data
use iced::canvas::{Fill, Frame};
use iced::{Point, Size};
use std::time::Instant;

fn run_fill(buffer: &Vec<f32>) -> Frame {
    let mut frame = Frame::new(Size::new(1000., 1000.));
    let mut path = iced::canvas::path::Builder::new();
    for (i, point) in buffer.iter().enumerate() {
        path.line_to(Point::new(i as f32, *point));
    }
    frame.fill(&path.build(), Fill::default());
    frame
}

fn main() {
    let mut oscillator = oscillator::Oscillator::sine(44100.0);
    oscillator.set_frequency(440.0);
    let mut output_buffer = Vec::new();
    output_buffer.resize(1000, 0.0);
    for sample in &mut output_buffer {
        *sample = oscillator.get();
        oscillator.tick();
    }
    let start = Instant::now();
    println!("Running...");
    for _ in 0..10000 {
        let frame = run_fill(&output_buffer);
        black_box(frame);
    }
    println!("Finished {}ms", start.elapsed().as_millis());
}
