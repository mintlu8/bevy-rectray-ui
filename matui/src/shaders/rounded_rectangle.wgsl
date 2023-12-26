#import bevy_sprite::mesh2d_vertex_output::VertexOutput,

@group(1) @binding(0)
var<uniform> color: vec4<f32>;

@group(1) @binding(1)
var<uniform> size: vec2<f32>;

@group(1) @binding(2) 
var<uniform> stroke_color: vec4<f32>;

@group(1) @binding(3) 
var<uniform> stroke: f32;

@group(1) @binding(4) 
var<uniform> capsule: f32;

// (--, +-, +-, ++)
@group(1) @binding(5)
var<uniform> corners: vec4<f32>;

@group(1) @binding(6) 
var texture: texture_2d<f32>;

@group(1) @binding(7) 
var samplr: sampler;

fn sdf(in: vec2<f32>) -> f32 {
    if (in.x > 0.0 && in.y > 0.0) {
        return sqrt(in.x * in.x + in.y * in.y);
    } else {
        return max(in.x, in.y);
    }
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
    let color = color * textureSample(texture, samplr, in.uv);
    var stroke = stroke;

    /// Short circuit stroke calculation to prevent artifects.
    if all(color == stroke_color) {
        stroke = 0.0;
    }

    let capsule_radius = min(size.x, size.y) / 2.0;
    radius = radius * (1.0 - capsule) + capsule_radius * capsule;
    
    let origin = size / 2.0 - radius;

    var position = abs((in.uv - 0.5) * size) - origin;

    var length = sdf(position);
    radius = radius - stroke;
    let smooth_fac = max(min(stroke / 2.0, 2.0), 0.0);
    let stroke_fac = (1.0 - smoothstep(stroke - smooth_fac, stroke, abs(length - radius))) * stroke_color.a;

    let factor = 1.0 - smoothstep(radius - 2.0, radius, length);
    let fill = color * factor;
    return fill * (1.0 - stroke_fac) + stroke_color * stroke_fac;
}