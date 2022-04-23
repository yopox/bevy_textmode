#import bevy_pbr::mesh_view_bind_group

struct ColorWrapper {
    color: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

[[group(1), binding(0)]]
var texture: texture_2d<f32>;
[[group(1), binding(1)]]
var texture_sampler: sampler;
[[group(1), binding(2)]]
var<uniform> bg: ColorWrapper;
[[group(1), binding(3)]]
var<uniform> fg: ColorWrapper;

[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let color = textureSample(texture, texture_sampler, in.uv);
    if (color[0] == 0.0) {
        return bg.color;
    } else {
        return fg.color;
    }
}
