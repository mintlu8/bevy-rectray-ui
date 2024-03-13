#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0)
var<uniform> fill: vec4<f32>;

@group(2) @binding(1)
var<uniform> stroke: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // the UVs are now adjusted around the middle of the rect.
    let uv = in.uv * 2.0 - 1.0;
    let len = sqrt(dot(uv, uv));
    if len < 0.8 {
        return vec4<f32>(fill.xyz, 1.0);
    } else if len < 1.0 {
        return vec4<f32>(stroke.xyz, 1.0);
    }
    return vec4<f32>(0.0);
}