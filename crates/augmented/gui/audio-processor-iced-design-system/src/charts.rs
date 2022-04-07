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
use iced::canvas::{Cursor, Frame, Geometry, Program, Stroke};
use iced::{canvas, Canvas, Color, Element, Length, Point, Rectangle, Size};

pub struct LineChart {
    data: Vec<(f32, f32)>,
}

impl LineChart {
    pub fn new(data: Vec<(f32, f32)>) -> Self {
        LineChart { data }
    }

    pub fn set_data(&mut self, data: Vec<(f32, f32)>) {
        self.data = data;
    }
}

#[derive(Debug, Clone)]
pub enum LineChartMessage {}

impl Program<LineChartMessage> for LineChart {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        if self.data.is_empty() {
            return vec![];
        }

        let size = bounds.size();
        let Size { width, height } = size;
        let mut frame = Frame::new(size);
        let mut path = canvas::path::Builder::new();
        let min_value_x = self.data.iter().map(|(x, _)| *x).fold(0.0, f32::min);
        let max_value_x = self.data.iter().map(|(x, _)| *x).fold(0.0, f32::max);
        let min_value_y = self.data.iter().map(|(_, y)| *y).fold(0.0, f32::min);
        let max_value_y = self.data.iter().map(|(_, y)| *y).fold(0.0, f32::max);
        let range_x = (min_value_x, max_value_x);
        let range_y = (min_value_y, max_value_y);

        for (x, y) in &self.data {
            let x_prime = interpolate(*x, range_x, (0.0, width));
            let y_prime = interpolate(*y, range_y, (0.0, height));
            path.line_to(Point::new(x_prime, y_prime));
        }

        let stroke = Stroke::default().with_color(Color::new(1.0, 1.0, 1.0, 1.0));
        frame.stroke(&path.build(), stroke);

        vec![frame.into_geometry()]
    }
}

impl LineChart {
    pub fn element(&mut self) -> Element<LineChartMessage> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn interpolate(value: f32, range_from: (f32, f32), range_to: (f32, f32)) -> f32 {
    let bounds_from = range_from.1 - range_from.0;
    let bounds_to = range_to.1 - range_to.0;
    (value - range_from.0) / bounds_from * bounds_to
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interpolate() {
        assert_eq!(interpolate(1., (0., 1.), (0., 2.)), 2.);
    }

    #[test]
    fn test_interpolate_negative_range() {
        assert_eq!(interpolate(0., (-1., 1.), (0., 2.)), 1.);
    }
}
