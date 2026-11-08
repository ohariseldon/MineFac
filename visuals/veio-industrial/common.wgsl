// Original Veio shader. MIT; no code or assets from Satisfactory or other packs.
// Keep all work bounded per visible fragment; there is no world scan or mod tick.
override shadows: bool = true;
// Choices are WGSL indices. Horizon uses the numeric labels to size the shadow pass.
override shadow_resolution: u32 = 1u;
override shadow_distance: u32 = 1u;
override atmosphere: f32 = 0.55;
// 0 Leve (8 GB), 1 Equilibrado, 2 Alto. Constant per pipeline: the GPU skips what is off.
override quality: u32 = 1u;
// What the Veio mod tells the pack each frame (frame.mods), 0 without the mod.
override seasons: bool = true;
override bloom: f32 = 0.6;
override sun_rays: f32 = 0.5;
override vignette: f32 = 0.2;
override ambient_occlusion: f32 = 0.7;
override screen_reflections: bool = true;

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
    let sun = mix(vec3<f32>(1.04, 1.025, 0.97), vec3<f32>(1.16, 0.90, 0.68), veio_low_sun() * 0.75);
    return mix(vec3<f32>(0.80, 0.86, 1.0), sun, veio_daylight());
}

// ---------- Veio's world state (frame.mods[0]) ----------
// x: season phase 0..4 (0 spring, 1 summer, 2 autumn, 3 winter), y: local air temperature / 50,
// z: industrial smog at the camera 0..1, w: 1 while the Veio mod is sending values.
fn veio_has_mod() -> bool {
    return frame.mods[0].w > 0.5;
}

// How much of each season shows now, eased across the borders: (spring, summer, autumn, winter).
fn veio_season_weights() -> vec4<f32> {
    if !seasons || !veio_has_mod() {
        return vec4<f32>(0.0, 1.0, 0.0, 0.0);
    }
    let phase = fract(frame.mods[0].x / 4.0) * 4.0;
    var w = vec4<f32>(0.0);
    for (var i = 0; i < 4; i++) {
        let d = abs(phase - (f32(i) + 0.5));
        let wrapped = min(d, 4.0 - d);
        w[i] = 1.0 - smoothstep(0.35, 0.65, wrapped);
    }
    return w / max(w.x + w.y + w.z + w.w, 0.001);
}

fn veio_smog() -> f32 {
    return select(0.0, clamp(frame.mods[0].z, 0.0, 1.0), veio_has_mod());
}

// Projects a camera-relative point (blocks) to the screen: xy uv from the top left, z 1 if in front.
fn veio_project(p: vec3<f32>) -> vec3<f32> {
    let view = frame.view * vec4<f32>(p, 0.0);
    let clip = frame.projection * vec4<f32>(view.xyz, 1.0);
    if clip.w <= 0.0001 {
        return vec3<f32>(0.0);
    }
    let ndc = clip.xy / clip.w;
    return vec3<f32>(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5, 1.0);
}

// ---------- Hash noise, no textures ----------
fn veio_hash2(p: vec2<f32>) -> f32 {
    let q = fract(p * vec2<f32>(0.1031, 0.1030));
    let r = q + dot(q, q.yx + 33.33);
    return fract((r.x + r.y) * r.x);
}

fn veio_hash3(p: vec3<f32>) -> f32 {
    var q = fract(p * 0.1031);
    q += dot(q, q.zyx + 31.32);
    return fract((q.x + q.y) * q.z);
}

fn veio_noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    let a = veio_hash2(i);
    let b = veio_hash2(i + vec2<f32>(1.0, 0.0));
    let c = veio_hash2(i + vec2<f32>(0.0, 1.0));
    let d = veio_hash2(i + vec2<f32>(1.0, 1.0));
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

// ---------- Atmosphere ----------
// A cheap single-scattering fit: Rayleigh blue overhead, warm Mie glow around a low sun,
// optical depth growing toward the horizon. No ray march.
fn veio_atmosphere(direction: vec3<f32>) -> vec3<f32> {
    let sun = frame.sun_direction;
    let up = max(direction.y, 0.0);
    let cos_theta = dot(direction, sun);
    let air = 1.0 / (up + 0.08);
    let sun_air = 1.0 / (max(sun.y, 0.0) + 0.12);
    let rayleigh = vec3<f32>(0.20, 0.46, 1.0);
    // Light reaching the sky along the sun's path loses its blue first: warm sunsets.
    let extinction = exp(-rayleigh * sun_air * 0.30);
    let phase_r = 0.75 * (1.0 + cos_theta * cos_theta);
    let g = 0.76;
    let phase_m = (1.0 - g * g) / pow(max(1.0 + g * g - 2.0 * g * cos_theta, 0.0001), 1.5) * 0.08;
    let scatter = rayleigh * phase_r * (1.0 - exp(-air * 0.18)) + vec3<f32>(phase_m) * (1.0 - exp(-air * 0.05));
    let day = smoothstep(-0.12, 0.10, sun.y);
    var color = (scatter * 1.45 + vec3<f32>(0.04)) * extinction * day;
    // Horizon haze, pale by day and orange toward a low sun.
    let toward = pow(0.5 + 0.5 * cos_theta, 3.0) * veio_low_sun();
    let haze = mix(vec3<f32>(0.62, 0.68, 0.78), vec3<f32>(1.0, 0.50, 0.20), toward);
    color += haze * exp(-up * 5.0) * 0.42 * day;
    // Night: dark blue with airglow at the horizon.
    let night = vec3<f32>(0.010, 0.016, 0.034) + vec3<f32>(0.010, 0.012, 0.020) * exp(-up * 4.0);
    return color + night * (1.0 - day);
}

fn veio_environment(direction: vec3<f32>) -> vec3<f32> {
    // Analytic sky colour, not a reflected camera or a world lookup.
    let sky = veio_atmosphere(direction);
    let ground = mix(vec3<f32>(0.03, 0.035, 0.04), vec3<f32>(0.20, 0.22, 0.20), veio_daylight());
    let below = smoothstep(0.0, -0.25, direction.y);
    let clear = mix(sky, ground, below);
    return mix(clear, frame.fog_color, clamp(frame.rain * 0.65 + frame.thunder * 0.20, 0.0, 0.85));
}

fn veio_fresnel(f0: vec3<f32>, cosine: f32) -> vec3<f32> {
    return f0 + (vec3<f32>(1.0) - f0) * veio_pow5(1.0 - clamp(cosine, 0.0, 1.0));
}

// Wind strength 0..1.5: a calm base, storms and the cold half of the year blow harder.
fn veio_wind() -> f32 {
    let seasons_now = veio_season_weights();
    let season = 0.85 * seasons_now.y + 1.0 * seasons_now.x + 1.15 * seasons_now.z + 1.2 * seasons_now.w;
    return (0.45 + 0.5 * frame.rain + 0.55 * frame.thunder) * season;
}

// 3D value noise (trilinear), for the volumetric clouds.
fn veio_noise3(p: vec3<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    let a = mix(veio_hash3(i), veio_hash3(i + vec3<f32>(1.0, 0.0, 0.0)), u.x);
    let b = mix(veio_hash3(i + vec3<f32>(0.0, 1.0, 0.0)), veio_hash3(i + vec3<f32>(1.0, 1.0, 0.0)), u.x);
    let c = mix(veio_hash3(i + vec3<f32>(0.0, 0.0, 1.0)), veio_hash3(i + vec3<f32>(1.0, 0.0, 1.0)), u.x);
    let d = mix(veio_hash3(i + vec3<f32>(0.0, 1.0, 1.0)), veio_hash3(i + vec3<f32>(1.0, 1.0, 1.0)), u.x);
    return mix(mix(a, b, u.y), mix(c, d, u.y), u.z);
}

// ---------- Cumulus layer, shared by the cloud pass and the cloud shadows on the ground ----------
override clouds: bool = true;
override volumetric_clouds: bool = true;
const VEIO_CLOUD_BASE: f32 = 560.0;
const VEIO_CLOUD_TOP: f32 = 1150.0;

// The camera's ground position wrapped to keep precision far from the origin.
fn veio_cloud_origin() -> vec2<f32> {
    return frame.camera_position.xz - floor(frame.camera_position.xz / 8192.0) * 8192.0;
}

// The wind's drift of the cloud field, in blocks.
fn veio_cloud_wind() -> vec2<f32> {
    return vec2<f32>(frame.time * 4.0, frame.time * 1.6) * (0.6 + veio_wind());
}

// How much of the sky the clouds cover: rain and the cold seasons more, summer less.
fn veio_cloud_cover() -> f32 {
    let w = veio_season_weights();
    return clamp(0.48 + 0.38 * frame.rain + 0.06 * w.w + 0.05 * w.z - 0.05 * w.y, 0.0, 0.95);
}

// The cloud field seen from above at a world point (xz, wrapped near the camera), 0..1: where
// the cumulus towers stand. Three octaves; the shape the 3D detail then erodes.
fn veio_cloud_coverage(xz: vec2<f32>) -> f32 {
    var q = (xz + veio_cloud_wind()) / 1150.0;
    var sum = 0.0;
    var amp = 0.55;
    for (var i = 0; i < 3; i++) {
        sum += veio_noise(q) * amp;
        q = q * 2.13 + vec2<f32>(5.2, -3.7);
        amp *= 0.5;
    }
    let cover = veio_cloud_cover();
    return smoothstep(1.0 - cover, 1.0 - cover + 0.28, sum / 0.9625);
}
