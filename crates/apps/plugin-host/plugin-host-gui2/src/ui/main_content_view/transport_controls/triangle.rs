use iced::canvas::{Cursor, Fill, Frame, Geometry, Path, Program};
use iced::mouse::Interaction;
use iced::{Canvas, Color, Element, Point, Rectangle};

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

impl<Message> Program<Message> for Triangle {
    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
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

    fn mouse_interaction(&self, bounds: Rectangle, cursor: Cursor) -> Interaction {
        if cursor.is_over(&bounds) {
            Interaction::Pointer
        } else {
            Interaction::default()
        }
    }
}
