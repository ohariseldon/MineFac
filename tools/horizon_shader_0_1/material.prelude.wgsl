// Generated from Horizon (MIT) crates/horizon-shader-api, shader API 0.1. Used only by tools/check_shaders.py.
struct Frame {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
    view_inverse: mat4x4<f32>,
    projection_inverse: mat4x4<f32>,
    camera_position: vec3<f32>,
    time: f32,
    sun_direction: vec3<f32>,
    day_time: f32,
    moon_direction: vec3<f32>,
    game_time: f32,
    sky_color: vec3<f32>,
    rain: f32,
    fog_color: vec3<f32>,
    thunder: f32,
    resolution: vec2<f32>,
    eye_light: vec2<f32>,
    fog_start: f32,
    fog_end: f32,
    render_distance: f32,
    far: f32,
    night_vision: f32,
    blindness: f32,
    darkness: f32,
    brightness: f32,
    eye_in: u32,
    dimension: i32,
    frame_index: u32,
    hide_gui: u32,
    precipitation: u32,
    temperature: f32,
    downfall: f32,
    wetness: f32,
    previous_view_projection: mat4x4<f32>,
    camera_motion: vec3<f32>,
    delta_time: f32,
    eye_light_smooth: vec2<f32>,
    in_water: f32,
    in_lava: f32,
    water_plane: f32,
    has_reflection: u32,
    reflection_resolution: vec2<f32>,
    mods: array<vec4<f32>, 8>,
    light_direction: vec3<f32>,
    has_shadow: u32,
    shadow_view_proj: mat4x4<f32>,
    shadow_distance: f32,
    shadow_distortion: f32,
    shadow_resolution: f32,
    shadow_center_texel: f32,
}
struct MaterialInput {
    albedo: vec4<f32>,
    tint: vec4<f32>,
    light: vec2<f32>,
    lightmap: vec3<f32>,
    position: vec3<f32>,
    uv: vec2<f32>,
    translucent: bool,
    tags: u32,
    material: u32,
    screen: vec2<f32>,
    face: u32,
    fluid: u32,
    emission: f32,
    normal: vec3<f32>,
    tangent: vec4<f32>,
    colored_light: vec3<f32>,
    displaced: f32,
}
struct Surface {
    color: vec4<f32>,
}
struct VertexInput {
    position: vec3<f32>,
    uv: vec2<f32>,
    material: u32,
    light: vec2<f32>,
    tint: vec4<f32>,
    face: u32,
    fluid: u32,
}
struct DisplacementInput {
    position: vec3<f32>,
    normal: vec3<f32>,
    uv: vec2<f32>,
    material: u32,
    height: f32,
}
@group(3) @binding(0) var<uniform> frame: Frame;
@group(3) @binding(1) var reflection: texture_2d<f32>;
@group(3) @binding(2) var reflection_sampler: sampler;
@group(3) @binding(3) var shadow_map: texture_depth_2d;
@group(3) @binding(4) var shadow_sampler: sampler_comparison;
@group(3) @binding(7) var shadow_map_translucent: texture_depth_2d;
@group(3) @binding(8) var shadow_color: texture_2d<f32>;
@group(3) @binding(6) var opaque_depth: texture_depth_2d;
@group(3) @binding(9) var block_light_colors: texture_3d<f32>;
@group(3) @binding(10) var<uniform> block_light_grid: vec4<f32>;
@group(3) @binding(11) var opaque_color: texture_2d<f32>;
// The colour of the block light at the camera-relative `position` (rgb, from the `[[lights]]`
// of the sources nearby) and how strongly they reach it (a, 0 where none does). Block light
// keeps its own brightness (`input.light.x`): this is its colour only.
fn block_light_color(position: vec3<f32>) -> vec4<f32> {
    let size = vec3<f32>(textureDimensions(block_light_colors));
    let uvw = (position - block_light_grid.xyz) / (block_light_grid.w * size);
    if any(uvw < vec3<f32>(0.0)) || any(uvw > vec3<f32>(1.0)) {
        return vec4<f32>(0.0);
    }
    let c = textureSampleLevel(block_light_colors, reflection_sampler, uvw, 0.0);
    return vec4<f32>(c.rgb * 4.0, c.a);
}
// What the post stage's `surface_at` reads for this point of solid terrain: its `normal` (a
// unit vector in world axes), `smoothness` (0..1, say for reflections) and `data`, a value of
// the pack's own passed through. Unset, the point keeps the face's own normal and 0 for both;
// translucent terrain stores none.
var<private> horizon_surface: vec4<f32>;
fn set_surface(normal: vec3<f32>, smoothness: f32, data: f32) {
    // Octahedral, y up: the upper half folds onto xz, the lower half unfolds around it.
    let n = normal / (abs(normal.x) + abs(normal.y) + abs(normal.z));
    var e = n.xz;
    if n.y < 0.0 {
        e = (1.0 - abs(n.zx)) * select(vec2<f32>(-1.0), vec2<f32>(1.0), n.xz >= vec2<f32>(0.0));
    }
    horizon_surface = vec4<f32>(bitcast<f32>(pack2x16snorm(e)), smoothness, data, 1.0);
}
// The point being shaded: its sprite's layer and mip level (the entries set them).
var<private> horizon_block_layer: u32;
var<private> horizon_block_lod: f32;
// The surface's block texture at `uv` (the sprite's space, as `input.uv`), without the tint:
// what `input.albedo` holds at the point itself, readable anywhere on the sprite, for normals
// or detail generated from the texture.
fn block_texture(uv: vec2<f32>) -> vec4<f32> {
    return horizon_block_sample(uv, horizon_block_layer, horizon_block_lod);
}
// `block_texture` at mip level `lod` (0 the full-size texture).
fn block_texture_level(uv: vec2<f32>, lod: f32) -> vec4<f32> {
    return horizon_block_sample(uv, horizon_block_layer, lod);
}
// The block texture's size, in texels (16 x 16 for vanilla's).
fn block_texture_size() -> vec2<f32> {
    return horizon_block_size();
}
// The mip level `block_texture` reads at this point (0 up close, more far away).
fn block_texture_lod() -> f32 {
    return horizon_block_lod;
}
// LabPBR: whether the resource packs ship normal (`_n`) or specular (`_s`) maps beside their
// block textures. Without them `block_normal_map` and `block_specular_map` give the defaults
// below.
fn block_pbr_maps() -> bool {
    return horizon_block_pbr_present();
}
// The surface's LabPBR normal map at `uv` (the sprite's space, as `input.uv`): rg the normal's
// x and y in the texture's space (x along `input.tangent`, y down the texture; 0..1 for
// -1..1), b ambient occlusion (1 none), a height (1 the top). (0.5, 0.5, 1, 1) where a texture
// has none.
fn block_normal_map(uv: vec2<f32>) -> vec4<f32> {
    return horizon_block_pbr(uv, horizon_block_layer, horizon_block_lod, 0u);
}
// `block_normal_map` at the block texture's mip level `lod` (the same footprint on the surface,
// where the map is smaller than the texture).
fn block_normal_map_level(uv: vec2<f32>, lod: f32) -> vec4<f32> {
    return horizon_block_pbr(uv, horizon_block_layer, lod, 0u);
}
// The surface's LabPBR specular map at `uv`: r perceptual smoothness, g F0 up to 229/255 and
// a metal's id above, b porosity or subsurface scattering, a emission (up to 254/255; 1
// none). (0, 0, 0, 0) where a texture has none.
fn block_specular_map(uv: vec2<f32>) -> vec4<f32> {
    return horizon_block_pbr(uv, horizon_block_layer, horizon_block_lod, 1u);
}
// `block_specular_map` at the block texture's mip level `lod`.
fn block_specular_map_level(uv: vec2<f32>, lod: f32) -> vec4<f32> {
    return horizon_block_pbr(uv, horizon_block_layer, lod, 1u);
}
// The LabPBR maps' size, in texels (they may be smaller than the block's texture).
fn block_pbr_size() -> vec2<f32> {
    return horizon_block_pbr_size();
}
// Block faces are flat: the derivatives of the position give the face's normal, turned
// toward the camera. Far away they lose precision, so a normal within about 3.6 degrees of
// an axis (every full block face) is that axis exactly: no shimmer on distant faces. The
// gentlest slope of flowing water, one level over a block, tilts about 6 degrees.
fn horizon_material_normal(position: vec3<f32>) -> vec3<f32> {
    let n = normalize(cross(dpdx(position), dpdy(position)));
    let a = abs(n);
    let m = max(a.x, max(a.y, a.z));
    let axis = select(select(vec3(0.0, 0.0, sign(n.z)), vec3(0.0, sign(n.y), 0.0), a.y == m),
        vec3(sign(n.x), 0.0, 0.0), a.x == m);
    let flat_n = select(n, axis, m > 0.998);
    return select(flat_n, -flat_n, dot(flat_n, position) > 0.0);
}
// The direction `uv.x` grows along on the surface, and the sign of the one `uv.y` grows
// along against `cross(normal, tangent)`.
fn horizon_material_tangent(position: vec3<f32>, uv: vec2<f32>, normal: vec3<f32>) -> vec4<f32> {
    let dp1 = dpdx(position);
    let dp2 = dpdy(position);
    let duv1 = dpdx(uv);
    let duv2 = dpdy(uv);
    let dp2perp = cross(dp2, normal);
    let dp1perp = cross(normal, dp1);
    let t = dp2perp * duv1.x + dp1perp * duv2.x;
    let b = dp2perp * duv1.y + dp1perp * duv2.y;
    if length(t) < 1e-12 {
        return vec4<f32>(1.0, 0.0, 0.0, 1.0);
    }
    let tangent = normalize(t - normal * dot(normal, t));
    return vec4<f32>(tangent, select(-1.0, 1.0, dot(cross(normal, tangent), b) >= 0.0));
}
// The shadow map's distortion toward its centre, of a point at shadow clip `xy`.
fn shadow_distort(xy: vec2<f32>) -> vec2<f32> {
    let k = frame.shadow_distortion;
    return xy / (length(xy) * k + 1.0 - k);
}
// Where the camera-relative point `position` is stored in the shadow map: xy its 0..1 texture
// coordinate (from the top left), z its depth from the light, for `textureSampleCompareLevel`.
fn shadow_position(position: vec3<f32>) -> vec3<f32> {
    let clip = frame.shadow_view_proj * vec4<f32>(position, 1.0);
    let xy = shadow_distort(clip.xy);
    // At most 1, the depth where nothing is: past the map's far end a point stays lit.
    return vec3<f32>(xy.x * 0.5 + 0.5, 0.5 - xy.y * 0.5, min(clip.z, 1.0));
}
