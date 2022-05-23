[[block]]
struct Uniform {
    size: f32;
    dimensions: vec2<f32>;
};

[[group(1), binding(0)]]
var<uniform> u: Uniform;

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] size: f32;
    [[location(1)]] dimensions: vec2<f32>;
    [[location(2)]] coord: vec2<f32>;
};

[[stage(vertex)]]
fn main(
    [[location(0)]] pos: vec2<f32>
) -> VertexOutput {
    var v: VertexOutput;
    v.coord = pos;
    v.position = vec4<f32>(pos.x, pos.y, 1.0, 1.0);
    v.size = u.size;
    v.dimensions = u.dimensions;
    return v;
}