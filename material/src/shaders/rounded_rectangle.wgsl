#import bevy_sprite::mesh2d_vertex_output::VertexOutput,

@group(1) @binding(0)
var<uniform> color: vec4<f32>;

@group(1) @binding(1)
var<uniform> size: vec2<f32>;

@group(1) @binding(2) 
var<uniform> stroke_color: vec4<f32>;

@group(1) @binding(3) 
var<uniform> stroke: f32;

// (--, +-, +-, ++)
@group(1) @binding(4)
var<uniform> corners: vec4<f32>;

@group(1) @binding(5) 
var texture: texture_2d<f32>;

@group(1) @binding(6) 
var samplr: sampler;

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
    let origin = size / 2.0 - radius;

    var position = max(abs((in.uv - 0.5) * size) - origin, vec2(0.0, 0.0));

    var length = sqrt(position.x * position.x + position.y * position.y);
    radius = radius - stroke;
    let stroke_fac = (1.0 - smoothstep(stroke - 2.0, stroke, abs(length - radius))) * stroke_color.a;

    let factor = 1.0 - smoothstep(radius - 2.0, radius, length);
    let fill = color * factor * textureSample(texture, samplr, in.uv);
    return fill * (1.0 - stroke_fac) + stroke_color * stroke_fac;
}