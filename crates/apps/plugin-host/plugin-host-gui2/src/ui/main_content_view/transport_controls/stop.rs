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
use iced::canvas::{Cursor, Fill, Frame, Geometry, Program};
use iced::mouse::Interaction;
use iced::{Canvas, Color, Element, Point, Rectangle, Size};

pub struct Stop {
    color: Color,
    hover: Color,
}

impl Default for Stop {
    fn default() -> Self {
        Self::new()
    }
}

impl Stop {
    pub fn new() -> Self {
        Stop {
            color: Color::default(),
            hover: Default::default(),
        }
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn hover(&mut self, color: Color) -> &mut Self {
        self.hover = color;
        self
    }

    pub fn view(&mut self) -> Element<()> {
        Canvas::new(self).into()
    }
}

impl<Message> Program<Message> for Stop {
    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());
        let color = if cursor.is_over(&bounds) {
            self.hover
        } else {
            self.color
        };
        let fill = Fill::from(color);
        frame.fill_rectangle(
            Point::new(0., 0.),
            Size::new(bounds.width, bounds.height),
            fill,
        );
        vec![frame.into_geometry()]
    }

    fn mouse_interaction(&self, bounds: Rectangle, cursor: Cursor) -> Interaction {
        if cursor.is_over(&bounds) {
            Interaction::Pointer
        } else {
            Interaction::default()
        }
    }
}
