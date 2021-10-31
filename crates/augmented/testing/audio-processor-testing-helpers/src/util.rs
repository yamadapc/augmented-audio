/// Get RMS level for a buffer
pub fn rms_level(input: &[f32]) -> f32 {
    if input.is_empty() {
        return 0.0;
    }
    let mut s = 0.0;
    for i in input {
        s += i.abs();
    }
    s / (input.len() as f32)
}
