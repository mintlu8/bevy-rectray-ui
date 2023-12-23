#import bevy_sprite::mesh2d_vertex_output::VertexOutput,

@group(1) @binding(0)
var<uniform> color: vec4<f32>;

@group(1) @binding(1)
var<uniform> shadow_size: f32;

@group(1) @binding(2)
var<uniform> size: vec2<f32>;

@group(1) @binding(3)
var<uniform> darken: f32;

fn sigmoid(t: f32) -> f32 {
    return 1.0 / (1.0 + exp(-t));
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var origin = vec2(size.x - size.y, 0.0) / 2.0;
    if size.y > size.x {
        origin = vec2(0.0, size.y - size.x) / 2.0;
    }
    let radius = size.x / 2.0 - shadow_size - origin.x;
    var position = max(abs((in.uv - 0.5) * size + vec2(-shadow_size / 4.0, shadow_size / 6.0)) - origin , vec2(0.0, 0.0));

    var factor = sqrt(position.x * position.x + position.y * position.y);
    factor = smoothstep(0.5, 0.75, sigmoid(1.0 - smoothstep(radius - shadow_size, radius + shadow_size, factor)));
    var result = color * factor;
    result.a = clamp((result.a * darken), 0.0, 1.0);
    return result;
}