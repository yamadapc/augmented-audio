use augmented_audio_gui_basics::prelude::*;

fn main() {
    sketch(|ctx| {
        let size = ctx.size();
        let canvas = ctx.canvas();
        canvas.clear(black());

        let widget = Rectangle::default();
        render(canvas, size, widget.into());
    })
}
