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
use criterion::black_box;
/// This is an example here just to get some profiling data
use iced::canvas::{Fill, Frame};
use iced::{Point, Size};
use std::time::Instant;

fn run_fill(buffer: &[f32]) -> Frame {
    let mut frame = Frame::new(Size::new(1000., 1000.));
    let mut path = iced::canvas::path::Builder::new();
    for (i, point) in buffer.iter().enumerate() {
        path.line_to(Point::new(i as f32, *point));
    }
    frame.fill(&path.build(), Fill::default());
    frame
}

fn main() {
    let mut oscillator = augmented_oscillator::Oscillator::sine(44100.0);
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
