use iced_baseview::canvas::{Cursor, Geometry, Program};
use iced_baseview::{Canvas, Element, Rectangle};

#[derive(Debug, Clone)]
pub enum Message {
    None,
}

pub struct LooperVisualizationView {}

impl LooperVisualizationView {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&mut self) -> Element<()> {
        Canvas::new(self).into()
    }
}

impl Program<()> for LooperVisualizationView {
    fn draw(&self, _bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        vec![]
    }
}
