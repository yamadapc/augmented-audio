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
use std::f32::consts::PI;

use skia_safe::{Canvas, Color4f, Paint, Path, Rect, Size};

pub enum KnobStyle {
    Center,
    Normal,
}

/// Represents an Arc that will be drawn for either the background or the filled track of a knob
#[allow(unused)]
struct KnobPathArc {
    center: (f32, f32),
    start_angle: f32,
    end_angle: f32,
    radius: f32,
}

/// Calculate the arc for a given value between 0-1.
///
/// This arc will leave a `0.25 * .pi` radians (45º) gap at the bottom of *each side* the knob and
/// be filled according to the `value` input.
///
/// For example, if the value is 0.5, the knob should be filled between 135º and 270º.
///
/// Drawing is inverted so angles are negative.
#[allow(unused)]
fn build_value_track(radius: f32, stroke_width: f32, value: f32) -> KnobPathArc {
    let center = (radius + stroke_width, radius + stroke_width);
    let start_angle = 0.0 - 0.75 * PI;
    let end_angle = start_angle - (0.75 * PI * 2.0) * value;

    KnobPathArc {
        center,
        start_angle,
        end_angle,
        radius,
    }
}

/// Calculate the arc for a given value between -1 and +1 for a center knob.
///
/// This arc will leave a 90º gap between start/end. The value `0` will be positioned mid-way between
/// 135º and 405º, at 270º past the right x-axis start.
///
/// The path is drawn by calculating what the `value * sweepAngle` total angle is between start/end
/// then starting at the `0` point at 270º.
///
/// If the value is negative, we rotate the *starting angle* counterclockwise by this amount.
#[allow(unused)]
fn build_center_value_track(radius: f32, stroke_width: f32, value: f32) -> KnobPathArc {
    let sweep_angle = sweep_angle(KnobStyle::Center);
    let center = (radius + stroke_width, radius + stroke_width);

    // This knob is a centered knob, so 0 is at the middle and -1/+1 at each
    // side. We will rotate the start angle either to the middle or back
    let mut start_angle = 0.0; // - 0.75 * .pi
    if value >= 0.0 {
        start_angle -= 0.75 * PI * 2.0
    } else {
        start_angle -= 0.75 * PI * 2.0 + sweep_angle * value
    }
    let end_angle = start_angle - sweep_angle * value.abs();

    KnobPathArc {
        center,
        start_angle,
        end_angle,
        radius,
    }
}

/// This returns the angle in radians that the value `1.0` represents.
///
/// For a normal knob, the track will start at 135º and end at 405º, counting from 0º at the right side x-axis.
/// The sweep-angle should be 270º degrees. This is the angle between start and end of the track.
///
/// For a centric knob that goes from -1 to +1 this is halved.
fn sweep_angle(style: KnobStyle) -> f32 {
    match style {
        KnobStyle::Center => 0.75 * PI,
        KnobStyle::Normal => 1.5 * PI,
    }
}

/// This represents the two points for a knob thumb. This is the line from the center of the knob to
/// the current value.
struct KnobPointerPath {
    start: (f32, f32),
    end: (f32, f32),
}

/// Calculate the center & current knob positions for the knob.
///
/// This calculation works in the following way.
/// First we determine the start angle of the track, depending on the knob style. This is either 135º or 270º for
/// normal and center knobs respectively.
///
/// Then, we calculate the angle at which the thumb will be from the 0º by subtracting the start angle by the
/// `value` times the total `sweepAngle` rotation for this style.
///
/// Then, we calculate the positions of both the pointer along the track and the center of the path considering
/// a `0.5 * strokeWidth` margin between the line and the circle edge and offsetting positions to account
/// for the circle line width.
fn build_pointer_path(radius: f32, style: KnobStyle, value: f32) -> KnobPointerPath {
    let start_angle = match style {
        KnobStyle::Normal => 0.75 * PI,
        KnobStyle::Center => 0.75 * PI * 2.0,
    };
    let thumb_angle = start_angle + value * sweep_angle(style);
    let center_coordinate = radius;

    let path_position = (
        // 0.8 is an arbitrary margin with the track
        center_coordinate + (radius * 0.8) * thumb_angle.cos(),
        center_coordinate + (radius * 0.8) * thumb_angle.sin(),
    );

    KnobPointerPath {
        start: (center_coordinate, center_coordinate),
        end: path_position,
    }
}

// pub struct KnobViewStyle {
//     background_color: Color,
// }

pub struct KnobView {
    radius: f32,
    stroke_width: f32,
    value: f32,
    // style: KnobViewStyle,
}

impl Default for KnobView {
    fn default() -> Self {
        Self {
            // style: KnobViewStyle { background_color: Color {

            // } },
            radius: 100.0,
            stroke_width: 8.0,
            value: 0.5,
        }
    }
}

impl KnobView {
    pub fn size(&self) -> Size {
        Size::new(self.radius * 2.0, self.radius * 2.0)
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.save();

        // Draw background
        let path = self.build_value_path(1.0);
        let mut paint = Paint::new(Color4f::new(1.0, 0.0, 0.0, 1.0), None);
        paint.set_stroke(true);
        paint.set_anti_alias(true);
        paint.set_stroke_width(self.stroke_width);
        canvas.draw_path(&path, &paint);

        // Draw value
        let path = self.build_value_path(self.value);
        let mut paint = Paint::new(Color4f::new(0.0, 1.0, 0.0, 1.0), None);
        paint.set_stroke(true);
        paint.set_anti_alias(true);
        paint.set_stroke_width(self.stroke_width);
        canvas.draw_path(&path, &paint);

        // Draw pointer
        let path = self.build_pointer_path(self.value);
        let mut paint = Paint::new(Color4f::new(0.0, 0.0, 1.0, 1.0), None);
        paint.set_stroke(true);
        paint.set_anti_alias(true);
        paint.set_stroke_width(self.stroke_width);
        canvas.draw_path(&path, &paint);

        canvas.restore();
    }

    fn build_value_path(&self, value: f32) -> Path {
        let mut path = Path::new();
        path.add_arc(
            Rect::new(0.0, 0.0, self.radius * 2.0, self.radius * 2.0),
            (0.75 * PI).to_degrees(),
            ((0.75 * PI * 2.0) * value).to_degrees(),
        );
        path
    }

    fn build_pointer_path(&self, value: f32) -> Path {
        let mut path = Path::new();
        let pointer_path = build_pointer_path(self.radius, KnobStyle::Normal, value);
        path.move_to(pointer_path.start);
        path.line_to(pointer_path.end);
        path
    }
}
