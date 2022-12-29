use iced::{widget::Canvas, Color, Element, Point, Rectangle, Theme};
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
use iced::mouse::Interaction;
use iced::widget::canvas::{Cursor, Fill, Frame, Geometry, Path, Program};

#[derive(Clone, Copy)]
pub struct Triangle {
    color: Color,
    hover: Color,
}

impl Default for Triangle {
    fn default() -> Self {
        Self::new()
    }
}

impl Triangle {
    pub fn new() -> Self {
        Triangle {
            color: Color::default(),
            hover: Default::default(),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn hover(mut self, color: Color) -> Self {
        self.hover = color;
        self
    }

    pub fn view(&self) -> Element<()> {
        Canvas::new(self).into()
    }
}

impl<Message> Program<Message> for Triangle {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());
        let triangle_path = Path::new(|builder| {
            builder.line_to(Point::new(0., 0.));
            builder.line_to(Point::new(0., bounds.height));
            builder.line_to(Point::new(bounds.width, bounds.height / 2.));
            builder.line_to(Point::new(0., 0.));
        });
        let color = if cursor.is_over(&bounds) {
            self.hover
        } else {
            self.color
        };
        frame.fill(&triangle_path, Fill::from(color));
        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Interaction {
        if cursor.is_over(&bounds) {
            Interaction::Pointer
        } else {
            Interaction::default()
        }
    }
}
