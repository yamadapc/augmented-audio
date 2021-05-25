// TODO - Add smoothstep as it's very useful
// https://en.wikipedia.org/wiki/Smoothstep

static TWO_PI: f32 = std::f32::consts::PI * 2.0;

pub fn sine_generator(phase: f32) -> f32 {
    (phase * TWO_PI).sin()
}

pub fn square_generator(phase: f32) -> f32 {
    static LIMIT: f32 = 0.5;

    if (phase % 1.0) < LIMIT {
        0.0
    } else {
        1.0
    }
}

pub fn saw_generator(phase: f32) -> f32 {
    1.0 - (phase % 1.0)
}
