use audio_processor_traits::combinators::mono_generator_function;
use audio_processor_traits::BufferProcessor;
use std::f32::consts::PI;

fn main() {
    let mut phase: f32 = 0.0;
    let processor = mono_generator_function(move |ctx| {
        let step = (1.0 / ctx.settings().sample_rate()) * 440.0;
        phase += step * 2.0 * PI;
        phase.sin()
    });

    audio_processor_standalone::audio_processor_main(BufferProcessor(processor));
}
