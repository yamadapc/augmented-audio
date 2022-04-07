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
use std::cmp::Ordering;

use piet::kurbo::{Affine, Line, PathEl, Point, Rect};
use piet::{Color, RenderContext, Text, TextAttribute, TextLayoutBuilder};
use piet_common::Device;

use audio_processor_traits::AudioProcessorSettings;

use crate::peak_detector::PeakDetector;

pub fn draw(output_file_path: &str, frames: &[f32], transients: &[f32]) {
    log::info!("Rendering image...");
    let width = 2000;
    let height = 1000;

    let mut device = Device::new().unwrap();
    let mut bitmap = device.bitmap_target(width, height, 1.0).unwrap();
    let mut render_context = bitmap.render_context();

    render_context.fill(
        Rect::new(0.0, 0.0, width as f64, height as f64),
        &Color::rgb(1.0, 1.0, 1.0),
    );
    let signal_color = Color::rgb(1.0, 0.0, 0.0);

    let num_charts = 5;

    // Draw audio line
    draw_line(
        &mut render_context,
        width,
        height / num_charts,
        frames,
        &signal_color,
    );
    let label = "Input audio";
    draw_text(&mut render_context, label);

    // Draw transient signal
    let _ = render_context.save();
    render_context.transform(Affine::translate((0.0, height as f64 / num_charts as f64)));
    let signal_color = Color::rgb(0.0, 1.0, 0.0);
    draw_line(
        &mut render_context,
        width,
        height / num_charts,
        transients,
        &signal_color,
    );
    let label = "Transient signal";
    draw_text(&mut render_context, label);
    let _ = render_context.restore();

    // Draw transient lines
    let _ = render_context.save();
    render_context.transform(Affine::translate((
        0.0,
        2.0 * (height as f64 / num_charts as f64),
    )));
    let signal_color = Color::rgb(0.0, 0.0, 1.0);
    let gated_transients = get_gated_transients(transients);
    draw_line(
        &mut render_context,
        width,
        height / num_charts,
        &gated_transients,
        &signal_color,
    );
    let label = "Transient magnitude";
    draw_text(&mut render_context, label);
    let _ = render_context.restore();

    // Draw transients through peak detector
    let mut peak_detector = PeakDetector::default();
    let attack_mult = crate::peak_detector::calculate_multiplier(
        AudioProcessorSettings::default().sample_rate,
        0.1,
    );
    let release_mult = crate::peak_detector::calculate_multiplier(
        AudioProcessorSettings::default().sample_rate,
        15.0,
    );
    let _ = render_context.save();
    let gated_transients: Vec<f32> = gated_transients
        .iter()
        .map(|f| {
            peak_detector.accept_frame(attack_mult, release_mult, &[*f]);
            peak_detector.value()
        })
        .collect();
    render_context.transform(Affine::translate((
        0.0,
        3.0 * (height as f64 / num_charts as f64),
    )));
    draw_line(
        &mut render_context,
        width,
        height / num_charts,
        &gated_transients,
        &Color::rgb(1.0, 0.0, 0.5),
    );
    let label = "Smoothed transients";
    draw_text(&mut render_context, label);
    let _ = render_context.restore();

    render_context.transform(Affine::translate((
        0.0,
        4.0 * (height as f64 / num_charts as f64),
    )));
    let test_thresholds = [0.2, 0.1];
    draw_line(
        &mut render_context,
        width,
        height / num_charts,
        frames,
        &Color::rgb(0.0, 0.0, 0.0).with_alpha(0.4),
    );
    for (i, threshold) in test_thresholds.iter().enumerate() {
        let mut inside_transient = false;
        let transient_positions: Vec<f32> = gated_transients
            .iter()
            .map(|f| {
                if !inside_transient && *f > *threshold {
                    inside_transient = true;
                    1.0
                } else if inside_transient && *f > *threshold {
                    0.0
                } else {
                    inside_transient = false;
                    0.0
                }
            })
            .collect();

        let base_height = (height / num_charts) as f32;
        let index_ratio = (test_thresholds.len() - i) as f32 / test_thresholds.len() as f32;
        let chart_height = (base_height * index_ratio) as usize;
        draw_transient_lines(
            &mut render_context,
            width,
            chart_height,
            &transient_positions,
            &Color::rgb(1.0, 0.0, 0.0),
        );
    }
    let label = "Transient positions";
    draw_text(&mut render_context, label);
    let _ = render_context.restore();

    render_context.finish().unwrap();
    std::mem::drop(render_context);

    bitmap
        .save_to_file(output_file_path)
        .expect("Failed to save image");
}

fn draw_text(render_context: &mut impl RenderContext, label: &str) {
    let text = render_context.text();
    let layout = text
        .new_text_layout(label.to_string())
        .default_attribute(TextAttribute::FontSize(20.0))
        .build()
        .unwrap();
    render_context.draw_text(&layout, (0.0, 0.0));
}

fn get_gated_transients(transients: &[f32]) -> Vec<f32> {
    let transients: Vec<f32> = transients.iter().map(|f| f.abs()).collect();
    let max_transient = transients
        .iter()
        .max_by(|f1, f2| f1.partial_cmp(f2).unwrap());
    let threshold = max_transient.unwrap() / 20.0;
    let gated_transients: Vec<f32> = transients
        .iter()
        .map(|transient| {
            if *transient > threshold {
                *transient
            } else {
                0.0
            }
        })
        .collect();
    gated_transients
}

fn draw_transient_lines(
    render_context: &mut impl RenderContext,
    width: usize,
    height: usize,
    frames: &[f32],
    signal_color: &Color,
) {
    let len = frames.len() as f64;
    let fwidth = width as f64;
    let fheight = height as f64;

    let lines: Vec<Line> = frames
        .iter()
        .enumerate()
        .map(|(i, s)| ((i as f64 / len) * fwidth, *s))
        .filter(|(_i, x)| !(x.is_nan() || x.is_infinite()))
        .filter(|(_i, x)| *x > 0.0)
        .map(|(x, _y)| Line::new(Point::new(x, 0.0), (x, fheight)))
        .collect();
    for line in lines {
        render_context.stroke(&line, signal_color, 3.0);
    }
}

fn draw_line(
    render_context: &mut impl RenderContext,
    width: usize,
    height: usize,
    frames: &[f32],
    signal_color: &Color,
) {
    let len = frames.len() as f64;
    let order = |f1: f32, f2: f32| f1.partial_cmp(&f2).unwrap_or(Ordering::Less);
    let min_sample = *frames.iter().min_by(|f1, f2| order(**f1, **f2)).unwrap() as f64;
    let max_sample = *frames.iter().max_by(|f1, f2| order(**f1, **f2)).unwrap() as f64;
    let fwidth = width as f64;
    let fheight = height as f64;
    let map_sample = |sig| {
        let r = (sig as f64 - min_sample) / (max_sample - min_sample);
        fheight - r * fheight
    };

    let mut path: Vec<PathEl> = frames
        .iter()
        .enumerate()
        .map(|(i, s)| ((i as f64 / len) * fwidth, map_sample(*s)))
        .filter(|(_i, x)| !(x.is_nan() || x.is_infinite()))
        .map(|(x, y)| Point::new(x, y))
        .map(PathEl::LineTo)
        .collect();
    path.insert(0, PathEl::MoveTo(Point::new(0.0, fheight / 2.0)));
    path.push(PathEl::LineTo(Point::new(fwidth, fheight / 2.0)));
    render_context.stroke(&*path, signal_color, 1.0);
}
