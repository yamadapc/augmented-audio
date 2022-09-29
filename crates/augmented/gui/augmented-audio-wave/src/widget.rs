use audio_processor_traits::Float;
use skia_safe::{Canvas, Color4f, Paint};

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, draw_context: &mut DrawContext) {
        let mut children = vec![
            FixedSize::new(
                Size {
                    width: 100.0,
                    height: 50.0,
                },
                Color4f::new(1.0, 0.0, 0.0, 1.0),
            ),
            FixedSize::new(
                Size {
                    width: 100.0,
                    height: 50.0,
                },
                Color4f::new(0.0, 1.0, 0.0, 1.0),
            ),
            FixedSize::new(
                Size {
                    width: 100.0,
                    height: 50.0,
                },
                Color4f::new(0.0, 0.0, 1.0, 1.0),
            ),
        ];

        let context = LayoutContext {};
        let constraints = BoxConstraints {
            min_width: 0.0,
            max_width: f32::infinity(),
            min_height: 0.0,
            max_height: f32::infinity(),
        };
        let sizes: Vec<Size> = children
            .iter_mut()
            .map(|child| child.layout(&context, constraints))
            .collect();

        let mut own_rect = Rectangle {
            origin: Point { x: 0.0, y: 0.0 },
            size: draw_context.size,
        };
        for (mut child, size) in children.iter_mut().zip(sizes) {
            draw_context.canvas.save();

            let child_rect = own_rect.take_left(size.width);

            draw_context
                .canvas
                .translate((child_rect.origin.x, child_rect.origin.y));
            let mut draw_context = DrawContext {
                size,
                canvas: draw_context.canvas,
            };
            child.draw(&mut draw_context);

            draw_context.canvas.restore();
        }

        // let canvas = draw_context.canvas;
        // let paint = Paint::new(Color4f::new(0.0, 1.0, 0.0, 1.0), None);
        // canvas.draw_circle(
        //     Point::new(
        //         draw_context.size.width / 2.0,
        //         draw_context.size.height / 2.0,
        //     ),
        //     100.0,
        //     &paint,
        // );
    }
}

pub struct LayoutContext {}

pub struct DrawContext<'a> {
    canvas: &'a mut Canvas,
    size: Size,
}

impl<'a> DrawContext<'a> {
    pub fn new(canvas: &'a mut Canvas, draw_size: Size) -> Self {
        Self {
            canvas,
            size: draw_size,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct BoxConstraints {
    min_width: f32,
    max_width: f32,
    min_height: f32,
    max_height: f32,
}

#[derive(Default, Clone, Copy)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

pub trait RenderObject {
    fn layout(&mut self, layout_context: &LayoutContext, box_constraints: BoxConstraints) -> Size {
        Size::default()
    }

    fn draw(&mut self, draw_context: &mut DrawContext) {}
}

#[derive(Default, Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32,
}

#[derive(Default, Clone, Copy)]
pub struct Rectangle {
    origin: Point,
    size: Size,
}

impl Rectangle {
    pub fn take_left(&mut self, x: f32) -> Rectangle {
        let origin = self.origin.clone();
        let mut size = self.size.clone();
        self.origin.x += x;
        self.size.width -= x;
        size.width = x;
        Rectangle { origin, size }
    }
}

struct FixedSize {
    size: Size,
    color: Color4f,
}

impl FixedSize {
    pub fn new(size: Size, color: Color4f) -> Self {
        Self { size, color }
    }
}

impl RenderObject for FixedSize {
    fn layout(
        &mut self,
        _layout_context: &LayoutContext,
        _box_constraints: BoxConstraints,
    ) -> Size {
        self.size
    }

    fn draw(&mut self, draw_context: &mut DrawContext) {
        let paint = Paint::new(self.color, None);
        draw_context
            .canvas
            .draw_rect(skia_safe::Rect::new(0.0, 0.0, 100.0, 100.0), &paint);
    }
}
