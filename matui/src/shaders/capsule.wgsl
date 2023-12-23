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
var texture: texture_2d<f32>;

@group(1) @binding(5) 
var samplr: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // the UVs are now adjusted around the middle of the rect.
    //let stroke = stroke / 2.0;
    var position = abs((in.uv * 2.0 - 1.0) * size);
    let radius = min(size.x, size.y) - stroke;
    if size.x > size.y {
        position.x = max(position.x - size.x + size.y, 0.0);
    } else {
        position.y = max(position.y - size.y + size.x, 0.0);
    }
    var length = sqrt(position.x * position.x + position.y * position.y);
    let fac = 1.0 - smoothstep(radius - 2.0, radius, length);
    let stroke_fac = (1.0 - smoothstep(stroke - 2.0, stroke, abs(length - radius))) * stroke_color.a;
    let fill = color * fac * textureSample(texture, samplr, in.uv);
    return fill * (1.0 - stroke_fac) + stroke_color * stroke_fac;
}
