[[block]]
struct Storage {
    fft: array<f32>;
};

[[group(1), binding(1)]]
var<storage, read_write> s: Storage;

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] size: f32;
    [[location(1)]] dimensions: vec2<f32>;
    [[location(2)]] coord: vec2<f32>;
};

[[stage(fragment)]]
fn main(v: VertexOutput) -> [[location(0)]] vec4<f32> {
    let fft_idx = sqrt(v.position.y / v.dimensions[0]) * 2048.0;
    let fft_value = s.fft[u32(fft_idx)];

    let ratio_x = (v.coord.x + 1.0) / 2.0;
    let c = fft_value;
    let a = step(1.0 - ratio_x, 0.01);

    return vec4<f32>(c, c, c, a);
}