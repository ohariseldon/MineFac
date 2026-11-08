// Original Veio shader. MIT; no code or assets from Satisfactory or other packs.
// Keep all work bounded per visible fragment; there is no world scan or mod tick.
override shadows: bool = true;
// Choices are WGSL indices. Horizon uses the numeric labels to size the shadow pass.
override shadow_resolution: u32 = 1u;
override shadow_distance: u32 = 1u;
override atmosphere: f32 = 0.55;

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

fn veio_low_sun() -> f32 {
    return 1.0 - smoothstep(0.06, 0.50, frame.sun_direction.y);
}

fn veio_key_direction() -> vec3<f32> {
    // Match the body that actually rendered the map, including moon shadows.
    if frame.has_shadow == 1u {
        return frame.light_direction;
    }
    return select(frame.moon_direction, frame.sun_direction, frame.sun_direction.y > -0.04);
}

fn veio_key_color() -> vec3<f32> {
    let sun = mix(vec3<f32>(1.04, 1.025, 0.97), vec3<f32>(1.14, 0.94, 0.76), veio_low_sun() * 0.65);
    return mix(vec3<f32>(0.78, 0.88, 1.08), sun, veio_daylight());
}

fn veio_environment(direction: vec3<f32>) -> vec3<f32> {
    // Analytic sky colour, not a reflected camera or a world lookup.
    let elevation = smoothstep(-0.15, 0.90, direction.y);
    let day_sky = mix(vec3<f32>(0.62, 0.75, 0.79), vec3<f32>(0.19, 0.40, 0.63), elevation);
    let night_sky = mix(frame.fog_color, vec3<f32>(0.018, 0.029, 0.055), elevation);
    let sky = mix(night_sky, day_sky, veio_daylight());
    return mix(sky, frame.fog_color, clamp(frame.rain * 0.65 + frame.thunder * 0.20, 0.0, 0.85));
}

fn veio_fresnel(f0: vec3<f32>, cosine: f32) -> vec3<f32> {
    return f0 + (vec3<f32>(1.0) - f0) * veio_pow5(1.0 - clamp(cosine, 0.0, 1.0));
}
