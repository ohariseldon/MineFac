// Where the sun is on the screen (uv), and whether it is in front of the camera.
fn veio_sun_screen() -> vec3<f32> {
    let clip = frame.projection * frame.view * vec4<f32>(frame.sun_direction * 1000.0, 1.0);
    if clip.w <= 0.0 {
        return vec3<f32>(0.0);
    }
    let ndc = clip.xy / clip.w;
    return vec3<f32>(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5, 1.0);
}

// Light shafts (half size): the sky's light marched toward the sun on the screen, blocked by
// whatever terrain is in front. Equilibrado takes 12 steps, Alto 24; Leve none.
fn post(input: PostInput) -> vec4<f32> {
    if quality == 0u || sun_rays <= 0.0 || frame.dimension != 0 || frame.eye_in != 0u {
        return vec4<f32>(0.0);
    }
    let sun = veio_sun_screen();
    let up = smoothstep(-0.02, 0.10, frame.sun_direction.y);
    if sun.z == 0.0 || up <= 0.0 {
        return vec4<f32>(0.0);
    }
    let steps = select(12, 24, quality >= 2u);
    let delta = (sun.xy - input.uv) / f32(steps);
    var uv = input.uv;
    var sum = 0.0;
    var weight = 1.0;
    // A per-pixel jitter hides the banding of few steps.
    let jitter = veio_hash2(input.pixel + vec2<f32>(f32(frame.frame_index % 64u), 0.0));
    uv += delta * jitter;
    for (var i = 0; i < 24; i++) {
        if i >= steps { break; }
        let open = select(0.0, 1.0, depth_at(uv) <= 0.0000001);
        sum += open * weight;
        weight *= 0.93;
        uv += delta;
    }
    // Strongest looking toward a low sun; gone in rain.
    let toward = 1.0 - smoothstep(0.0, 0.75, length((sun.xy - input.uv) * vec2<f32>(frame.resolution.x / frame.resolution.y, 1.0)));
    let colour = mix(vec3<f32>(1.0, 0.94, 0.82), vec3<f32>(1.0, 0.62, 0.32), veio_low_sun());
    let strength = sum / f32(steps) * toward * up * (1.0 - frame.rain) * sun_rays * 0.55;
    return vec4<f32>(colour * strength, 1.0);
}
