use skia_safe::Canvas;

pub struct LayoutContext {}

pub struct DrawContext<'a> {
    canvas: &'a mut Canvas,
}

pub trait Widget {
    fn layout(&mut self, layout_context: LayoutContext, box_constraints: BoxConstraints) {}

    fn draw(&mut self, draw_context: DrawContext) {}
}
