use iced::canvas::{Cursor, Fill, Frame, Geometry, Program};
use iced::mouse::Interaction;
use iced::{Canvas, Color, Element, Point, Rectangle, Size};

pub struct Stop {
    color: Color,
    hover: Color,
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
