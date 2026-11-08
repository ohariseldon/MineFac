// Generated from Horizon (MIT) crates/horizon-shader-api, shader API 0.1. Used only by tools/check_shaders.py.
fn horizon_block_sample(uv: vec2<f32>, layer: u32, lod: f32) -> vec4<f32> {
    return vec4<f32>(1.0);
}
fn horizon_block_size() -> vec2<f32> {
    return vec2<f32>(16.0);
}
fn horizon_block_pbr(uv: vec2<f32>, layer: u32, lod: f32, map: u32) -> vec4<f32> {
    return select(vec4<f32>(0.5, 0.5, 1.0, 1.0), vec4<f32>(0.0), map == 1u);
}
fn horizon_block_pbr_present() -> bool {
    return false;
}
fn horizon_block_pbr_size() -> vec2<f32> {
    return vec2<f32>(16.0);
}
struct HorizonMaterialIn {
    @builtin(position) position: vec4<f32>,
    @location(0) albedo: vec4<f32>,
    @location(1) tint: vec4<f32>,
    @location(2) light: vec2<f32>,
    @location(3) lightmap: vec3<f32>,
    @location(4) world: vec3<f32>,
    @location(5) uv: vec2<f32>,
    @location(6) @interpolate(flat) translucent: u32,
    @location(7) @interpolate(flat) tags: u32,
    @location(8) @interpolate(flat) face: u32,
    @location(9) @interpolate(flat) fluid_quad: u32,
    @location(10) @interpolate(flat) emission: f32,
    @location(11) @interpolate(flat) material: u32,
}
@fragment fn horizon_fs_material(in: HorizonMaterialIn) -> @location(0) vec4<f32> {
    let fluid = select(0u, select(2u, 1u, in.translucent != 0u), in.fluid_quad != 0u);
    let normal = horizon_material_normal(in.world);
    let tangent = horizon_material_tangent(in.world, in.uv, normal);
    let input = MaterialInput(in.albedo, in.tint, in.light, in.lightmap, in.world, in.uv, in.translucent != 0u, in.tags, in.material,
        in.position.xy / frame.resolution, in.face, fluid, in.emission, normal, tangent, vec3<f32>(0.0), 0.0);
    return material(input).color;
}
