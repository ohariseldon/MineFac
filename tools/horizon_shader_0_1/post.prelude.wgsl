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
struct PostInput {
    uv: vec2<f32>,
    pixel: vec2<f32>,
    color: vec4<f32>,
    depth: f32,
}
@group(3) @binding(0) var<uniform> frame: Frame;
@group(3) @binding(1) var scene_color: texture_2d<f32>;
@group(3) @binding(2) var scene_depth: texture_depth_2d;
@group(3) @binding(3) var previous: texture_2d<f32>;
@group(3) @binding(4) var linear_sampler: sampler;
@group(3) @binding(5) var nearest_sampler: sampler;
@group(3) @binding(6) var shadow_map: texture_depth_2d;
@group(3) @binding(7) var shadow_sampler: sampler_comparison;
@group(3) @binding(8) var history: texture_2d<f32>;
@group(3) @binding(9) var shadow_map_translucent: texture_depth_2d;
@group(3) @binding(10) var shadow_color: texture_2d<f32>;
@group(3) @binding(11) var scene_surface: texture_2d<u32>;
// The world's depth at `uv` (reversed: 0 is the sky).
fn depth_at(uv: vec2<f32>) -> f32 {
    let size = vec2<f32>(textureDimensions(scene_depth));
    let texel = vec2<i32>(clamp(uv * size, vec2<f32>(0.0), size - 1.0));
    return textureLoad(scene_depth, texel, 0);
}
// The camera-relative world position, in blocks, of the point at `uv` and `depth`.
fn view_position(uv: vec2<f32>, depth: f32) -> vec3<f32> {
    let clip = vec4<f32>(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0, depth, 1.0);
    let view = frame.projection_inverse * clip;
    return (frame.view_inverse * vec4<f32>(view.xyz / view.w, 0.0)).xyz;
}
// The solid terrain's surface at a pixel, as the material stage's `set_surface` left it.
struct SurfaceData {
    // False where the sky, an entity, water, glass or a particle shows instead.
    present: bool,
    // A unit vector in world axes.
    normal: vec3<f32>,
    smoothness: f32,
    // The pack's own value, passed through.
    data: f32,
}
fn surface_at(texel: vec2<i32>) -> SurfaceData {
    let size = vec2<i32>(textureDimensions(scene_surface));
    let t = clamp(texel, vec2<i32>(0), size - 1);
    let s = bitcast<vec4<f32>>(textureLoad(scene_surface, t, 0));
    let depth_size = vec2<i32>(textureDimensions(scene_depth));
    let depth = textureLoad(scene_depth, clamp(texel, vec2<i32>(0), depth_size - 1), 0);
    // The terrain's own depth is stored with it: anything drawn nearer since hides it.
    let present = s.w > 0.0 && abs(s.w - depth) <= s.w * 1e-5;
    let e = unpack2x16snorm(bitcast<u32>(s.x));
    var n = vec3<f32>(e.x, 1.0 - abs(e.x) - abs(e.y), e.y);
    if n.y < 0.0 {
        n = vec3<f32>((1.0 - abs(n.z)) * sign(n.x), n.y, (1.0 - abs(n.x)) * sign(n.z));
    }
    return SurfaceData(present, select(vec3<f32>(0.0), normalize(n), present),
        select(0.0, s.y, present), select(0.0, s.z, present));
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
