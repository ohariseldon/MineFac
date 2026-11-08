// Where the sun is on the screen (uv), and whether it is in front of the camera.
fn veio_sun_screen() -> vec3<f32> {
    let clip = frame.projection * frame.view * vec4<f32>(frame.sun_direction * 1000.0, 1.0);
    if clip.w <= 0.0 {
        return vec3<f32>(0.0);
    }
    let ndc = clip.xy / clip.w;
    return vec3<f32>(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5, 1.0);
}

// Light shafts without a shadow map: the sky's light marched toward the sun on the screen,
// blocked by whatever terrain is in front.
fn veio_screen_rays(input: PostInput) -> vec4<f32> {
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

// How thick the air is at a camera-relative point: a base haze, morning and autumn mist, rain,
// factory smog, thinning with height above the sea.
fn veio_air_density(p: vec3<f32>) -> f32 {
    let height = p.y + frame.camera_position.y;
    let w = veio_season_weights();
    let morning = veio_low_sun() * veio_daylight() * smoothstep(0.0, 0.4, frame.sun_direction.x);
    let mist = 0.35 + 0.9 * morning * (0.6 + 0.6 * w.z + 0.3 * w.x) + 1.4 * frame.rain + 1.2 * veio_smog();
    return mist * exp(-max(height - 62.0, 0.0) / 38.0);
}

// Volumetric light (half size): the view ray marched through the air, each step asking the
// shadow map whether the sun reaches it. Beams pass between leaves, windows and chimneys, also
// with the sun behind the camera. 8 steps on Equilibrado, 16 on Alto; Leve none.
fn post(input: PostInput) -> vec4<f32> {
    if quality == 0u || sun_rays <= 0.0 || frame.dimension != 0 || frame.eye_in != 0u {
        return vec4<f32>(0.0);
    }
    if !shadows || frame.has_shadow == 0u {
        return veio_screen_rays(input);
    }
    let depth = depth_at(input.uv);
    let far_point = view_position(input.uv, max(depth, 0.0000001));
    let reach = min(frame.shadow_distance * 0.95, 96.0);
    let length_to = select(min(length(far_point), reach), reach, depth <= 0.000001);
    let direction = normalize(view_position(input.uv, 0.5));
    let steps = select(8, 16, quality >= 2u);
    let step_length = length_to / f32(steps);
    // Interleaved gradient noise: steady per pixel, the bilinear upscale hides it.
    let jitter = fract(52.9829189 * fract(dot(floor(input.pixel), vec2<f32>(0.06711056, 0.00583715))));
    var lit = 0.0;
    var travelled = 0.0;
    for (var i = 0; i < 16; i++) {
        if i >= steps { break; }
        let t = (f32(i) + jitter) * step_length;
        let p = direction * t;
        let s = shadow_position(p);
        var sun = 1.0;
        if all(s.xy > vec2<f32>(0.0)) && all(s.xy < vec2<f32>(1.0)) && s.z < 1.0 {
            sun = textureSampleCompareLevel(shadow_map, shadow_sampler, s.xy, s.z - 0.0005);
        }
        let density = veio_air_density(p);
        // Light already scattered away on the way in thins what comes from farther.
        lit += sun * density * exp(-travelled * 0.012) * step_length;
        travelled += density * step_length;
    }
    let key = veio_key_direction();
    let cos_theta = dot(direction, key);
    // Henyey-Greenstein forward peak plus a little all-around glow.
    let g = 0.62;
    let phase = (1.0 - g * g) / pow(max(1.0 + g * g - 2.0 * g * cos_theta, 0.0001), 1.5) * 0.08 + 0.05;
    let underground = smoothstep(0.2, 0.7, frame.eye_light_smooth.y);
    let strength = lit * phase * 0.020 * sun_rays * underground * (1.0 - 0.6 * frame.rain) * mix(0.25, 1.0, veio_daylight());
    // Thick mist saturates instead of washing the picture out.
    return vec4<f32>(veio_key_color() * (1.0 - exp(-strength)) * 0.8, 1.0);
}
