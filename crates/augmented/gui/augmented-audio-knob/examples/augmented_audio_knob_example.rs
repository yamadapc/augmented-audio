use skia_safe::Color4f;

use augmented_audio_knob::KnobView;

fn main() {
    augmented_audio_gui_basics::sketch(|ctx| {
        let size = ctx.size();
        let canvas = ctx.canvas();
        canvas.clear(Color4f::new(0.0, 0.0, 0.0, 1.0));

        canvas.save();

        let knob = KnobView::default();
        canvas.translate((
            size.width / 2.0 - knob.size().width / 2.0,
            size.height / 2.0 - knob.size().height / 2.0,
        ));

        knob.draw(canvas);

        canvas.restore();
    })
}
