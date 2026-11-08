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
struct VertexInput {
    position: vec3<f32>,
    uv: vec2<f32>,
    material: u32,
    light: vec2<f32>,
    tint: vec4<f32>,
    face: u32,
    fluid: u32,
}
@group(3) @binding(0) var<uniform> frame: Frame;
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
