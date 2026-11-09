// Light from the scene's shape (half size), two effects in one target so the pack stays within
// the four post outputs a pack may write. r: ambient occlusion, how open the point is; g: its
// distance, for the depth-aware upscale in the last pass; b: how much sunlight the air sends
// (light shafts), coloured in the last pass by veio_rays_colour.

fn veio_ao(input: PostInput) -> vec2<f32> {
    if quality == 0u || ambient_occlusion <= 0.0 {
        return vec2<f32>(1.0, 0.0);
    }
    let depth = depth_at(input.uv);
    if depth <= 0.000001 {
        return vec2<f32>(1.0, 4096.0);
    }
    let position = view_position(input.uv, depth);
    let distance = length(position);
    let surface = surface_at(vec2<i32>(input.uv * vec2<f32>(textureDimensions(scene_surface))));
    if !surface.present || distance > 72.0 {
        return vec2<f32>(1.0, distance);
    }
    let n = surface.normal;
    let helper = select(vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(1.0, 0.0, 0.0), abs(n.y) > 0.9);
    let t = normalize(cross(helper, n));
    let b = cross(n, t);
    // A fixed rotation per pixel (no flicker); the upscale's four taps smooth the pattern.
    let spin = veio_hash2(floor(input.pixel)) * 6.2831853;
    let samples = select(8, 12, quality >= 2u);
    let radius = 0.85;
    var hidden = 0.0;
    for (var i = 0; i < 12; i++) {
        if i >= samples { break; }
        let k = (f32(i) + 0.5) / f32(samples);
        let angle = spin + f32(i) * 2.3999632;
        let spread = sqrt(k);
        // Cosine-weighted hemisphere, nearer samples weigh more.
        let local = vec3<f32>(cos(angle) * spread, sin(angle) * spread, sqrt(1.0 - k));
        let reach = radius * mix(0.25, 1.0, k * k);
        let p = position + (t * local.x + b * local.y + n * local.z) * reach + n * 0.03;
        let screen = veio_project(p);
        if screen.z == 0.0 || any(screen.xy < vec2<f32>(0.0)) || any(screen.xy > vec2<f32>(1.0)) {
            continue;
        }
        let scene_depth_here = depth_at(screen.xy);
        if scene_depth_here <= 0.000001 {
            continue;
        }
        let scene = view_position(screen.xy, scene_depth_here);
        let in_front = length(p) - length(scene);
        // Only nearby geometry hides the sky: a wall far behind does not darken a ledge.
        let near = 1.0 - smoothstep(radius * 0.8, radius * 2.0, length(scene - position));
        hidden += select(0.0, near, in_front > 0.02);
    }
    var open = 1.0 - hidden / f32(samples);
    // Leaves and grass shade themselves already; keep them light.
    open = mix(open, 1.0, surface.data * 0.6);
    let fade = smoothstep(48.0, 72.0, distance);
    return vec2<f32>(mix(open, 1.0, fade), distance);
}

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
fn veio_screen_rays(input: PostInput) -> f32 {
    let sun = veio_sun_screen();
    let up = smoothstep(-0.02, 0.10, frame.sun_direction.y);
    if sun.z == 0.0 || up <= 0.0 {
        return 0.0;
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
    return sum / f32(steps) * toward * up * (1.0 - frame.rain) * sun_rays * 0.55;
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
fn veio_rays(input: PostInput) -> f32 {
    if quality == 0u || sun_rays <= 0.0 || frame.dimension != 0 || frame.eye_in != 0u {
        return 0.0;
    }
    if !shadows || frame.has_shadow == 0u {
        return veio_screen_rays(input);
    }
    let depth = depth_at(input.uv);
    let far_point = view_position(input.uv, max(depth, 0.0000001));
    let reach = min(frame.shadow_distance * 0.95, 96.0);
    let length_to = select(min(length(far_point), reach), reach, depth <= 0.000001);
    // The view ray as the difference of two points on it: the view bob of walking and jumping
    // moves the eye inside the projection, so one point 2 near-planes away gave a ray off by tens
    // of degrees (clouds and light shafts jumped with every step).
    let direction = normalize(view_position(input.uv, 0.000001) - view_position(input.uv, 1.0));
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
    // Over terrain the shafts stay a hint: looking toward the sun they washed nearby hills white.
    let over_terrain = select(1.0, 0.55, depth > 0.000001);
    let strength = lit * phase * 0.020 * over_terrain * sun_rays * underground * (1.0 - 0.6 * frame.rain) * mix(0.25, 1.0, veio_daylight());
    // Thick mist saturates instead of washing the picture out.
    return 1.0 - exp(-strength);
}

fn post(input: PostInput) -> vec4<f32> {
    if quality == 0u {
        return vec4<f32>(1.0, 0.0, 0.0, 1.0);
    }
    let occlusion = veio_ao(input);
    return vec4<f32>(occlusion, veio_rays(input), 1.0);
}
