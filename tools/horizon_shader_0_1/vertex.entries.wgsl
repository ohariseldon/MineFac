// Generated from Horizon (MIT) crates/horizon-shader-api, shader API 0.1. Used only by tools/check_shaders.py.
@vertex fn horizon_vs_vertex(
    @location(0) position: vec3<f32>, @location(1) uv: vec2<f32>, @location(2) light: vec2<f32>,
    @location(3) tint: vec4<f32>, @location(4) material: u32, @location(5) flags: u32,
) -> @builtin(position) vec4<f32> {
    let input = VertexInput(position, uv, material, light, tint, flags & 7u, flags >> 31u);
    return frame.projection * frame.view * vec4<f32>(vertex(input), 1.0);
}
