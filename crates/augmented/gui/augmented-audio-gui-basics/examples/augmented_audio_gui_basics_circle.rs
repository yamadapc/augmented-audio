use skia_safe::{Color4f, Paint, Point};

fn main() {
    augmented_audio_gui_basics::sketch(|ctx| {
        let size = ctx.size();
        let canvas = ctx.canvas();

        canvas.clear(Color4f::new(0.0, 0.0, 0.0, 1.0));

        let paint = Paint::new(Color4f::new(0.0, 1.0, 0.0, 1.0), None);
        let center = Point::new(size.width / 2.0, size.height / 2.0);
        canvas.draw_circle(center, 100.0, &paint);
    })
}
