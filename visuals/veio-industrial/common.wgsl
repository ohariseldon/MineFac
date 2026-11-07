// Original Veio shader. MIT; no code or assets from Satisfactory or other packs.
// Keep all work bounded per visible fragment; there is no world scan or mod tick.
override shadows: bool = true;
// Choices are WGSL indices. Horizon uses the numeric labels to size the shadow pass.
override shadow_resolution: u32 = 1u;
override shadow_distance: u32 = 1u;

fn veio_daylight() -> f32 {
    return smoothstep(-0.10, 0.18, frame.sun_direction.y);
}

fn veio_luma(color: vec3<f32>) -> f32 {
    return dot(color, vec3<f32>(0.2126, 0.7152, 0.0722));
}

fn veio_pow5(value: f32) -> f32 {
    let square = value * value;
    return square * square * value;
}
