#import bevy_sprite::mesh2d_vertex_output::VertexOutput,

@group(1) @binding(0)
var<uniform> color: vec4<f32>;

@group(1) @binding(1)
var<uniform> shadow_size: f32;

@group(1) @binding(2)
var<uniform> size: vec2<f32>;

@group(1) @binding(3) 
var<uniform> capsule: f32;

// (--, +-, +-, ++)
@group(1) @binding(4)
var<uniform> corners: vec4<f32>;

fn sdf(in: vec2<f32>) -> f32 {
    if (in.x > 0.0 && in.y > 0.0) {
        return sqrt(in.x * in.x + in.y * in.y);
    } else {
        return max(in.x, in.y);
    }
} 


fn sigmoid(t: f32) -> f32 {
    return 1.0 / (1.0 + exp(-t));
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // the UVs are now adjusted around the middle of the rect.
    var radius = corners.x;
    if in.uv.x > 0.5 && in.uv.y > 0.5 {
        radius = corners.w;
    } else if in.uv.x > 0.5 {
        radius = corners.y;
    } else if in.uv.y > 0.5 {
        radius = corners.z;
    }
    let capsule_radius = min(size.x, size.y) / 2.0 - shadow_size;
    radius = radius * (1.0 - capsule) + capsule_radius * capsule;

    let origin = size / 2.0 - radius - shadow_size;

    var position = abs((in.uv - 0.5) * size + vec2(-shadow_size / 4.0, shadow_size / 6.0)) - origin;

    var factor = sdf(position);
    factor = smoothstep(0.5, 0.75, sigmoid(1.0 - smoothstep(radius - shadow_size, radius + shadow_size, factor)));
    return  color * factor;
}