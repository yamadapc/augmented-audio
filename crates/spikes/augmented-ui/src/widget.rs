pub struct DrawContext {}

pub trait Widget {
    fn draw(&mut self, context: &DrawContext);
}
