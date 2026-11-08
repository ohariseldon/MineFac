

// Layered cloud density 0..1 at a point of the cloud plane (blocks).
fn veio_cloud_density(p: vec2<f32>) -> f32 {
    let wind = vec2<f32>(frame.time * 3.0, frame.time * 1.1) * (0.6 + veio_wind());
    // Far clouds grow larger so the horizon does not fill with specks.
    let far = 1.0 + length(p - veio_cloud_origin()) / 9000.0;
    var q = (p + wind) / (900.0 * far);
    var sum = 0.0;
    var amp = 0.55;
    // Two octaves on Leve, four on Equilibrado and five on Alto.
    let octaves = select(select(4, 5, quality >= 2u), 2, quality == 0u);
    for (var i = 0; i < 5; i++) {
        if i >= octaves { break; }
        sum += veio_noise(q) * amp;
        q = q * 2.07 + vec2<f32>(17.3, -9.1);
        amp *= 0.5;
    }
    let seasons_now = veio_season_weights();
    // Rain and the cold seasons cover more of the sky.
    let cover = 0.50 + 0.30 * frame.rain + 0.06 * seasons_now.w + 0.04 * seasons_now.z - 0.06 * seasons_now.y;
    return smoothstep(1.0 - cover, 1.0 - cover + 0.32, sum);
}

// ---------- Physical sky ----------
// Single scattering through a spherical atmosphere: Rayleigh (air), Mie (haze) and ozone, with
// the standard Earth coefficients of Hillaire, "A Scalable and Production Ready Sky and
// Atmosphere Rendering Technique" (EGSR 2020) and Bruneton's precomputed model. Kilometres.
// The light to the sun is marched too, so sunsets redden on their own and the Earth's shadow
// rises at dusk. Then a fixed exposure and the ACES film curve (Narkowicz fit) bring it to the
// screen, and the result is encoded to the game's gamma, as API 0.1 expects.
const VEIO_EARTH: f32 = 6360.0;
const VEIO_TOP: f32 = 6460.0;
const VEIO_RAYLEIGH: vec3<f32> = vec3<f32>(5.802e-3, 13.558e-3, 33.1e-3);
const VEIO_MIE_SCATTER: f32 = 3.996e-3;
const VEIO_MIE_EXTINCTION: f32 = 4.40e-3;
const VEIO_OZONE: vec3<f32> = vec3<f32>(0.650e-3, 1.881e-3, 0.085e-3);
const VEIO_SKY_EXPOSURE: f32 = 20.0;

// Distance along a ray from radius r (cosine mu with the vertical) to the sphere of radius R,
// written so that r*r - R*R does not lose its digits; negative when the ray misses.
fn veio_to_sphere(r: f32, mu: f32, radius: f32, far: bool) -> f32 {
    let c = (r - radius) * (r + radius);
    let d = r * r * mu * mu - c;
    if d < 0.0 {
        return -1.0;
    }
    return select(-r * mu - sqrt(d), -r * mu + sqrt(d), far);
}

// Rayleigh, Mie and ozone densities at a height (km).
fn veio_air(h: f32) -> vec3<f32> {
    return vec3<f32>(exp(-h / 8.0), exp(-h / 1.2), max(0.0, 1.0 - abs(h - 25.0) / 15.0));
}

fn veio_extinction(air: vec3<f32>) -> vec3<f32> {
    return VEIO_RAYLEIGH * air.x + vec3<f32>(VEIO_MIE_EXTINCTION * air.y) + VEIO_OZONE * air.z;
}

// How much sunlight reaches a point at radius r with the sun at cosine mu over its horizon.
fn veio_sun_transmittance(r: f32, mu: f32, steps: i32) -> vec3<f32> {
    if veio_to_sphere(r, mu, VEIO_EARTH, false) > 0.0 {
        return vec3<f32>(0.0);
    }
    let length_to = max(veio_to_sphere(r, mu, VEIO_TOP, true), 0.0);
    let step_length = length_to / f32(steps);
    var depth = vec3<f32>(0.0);
    for (var j = 0; j < 4; j++) {
        if j >= steps { break; }
        let s = (f32(j) + 0.5) * step_length;
        let h = sqrt(r * r + s * s + 2.0 * r * mu * s) - VEIO_EARTH;
        depth += veio_extinction(veio_air(h)) * step_length;
    }
    return exp(-depth);
}

struct VeioSky {
    light: vec3<f32>,
    // What is left of the light behind (the sun's disc), 0 where the ground is.
    through: vec3<f32>,
}

// The sky's light along a view direction, linear, before exposure.
fn veio_physical_sky(direction: vec3<f32>, sun: vec3<f32>) -> VeioSky {
    let r = VEIO_EARTH + 0.1 + clamp((frame.camera_position.y - 64.0) * 0.001, 0.0, 0.3);
    // Keep the view a hair above the horizon: below it the game draws terrain, not the void.
    let d = normalize(vec3<f32>(direction.x, max(direction.y, 0.012), direction.z));
    let mu = d.y;
    let length_to = min(veio_to_sphere(r, mu, VEIO_TOP, true), 800.0);
    let cos_theta = dot(d, sun);
    let rayleigh_phase = 0.0596831 * (1.0 + cos_theta * cos_theta);
    let g = 0.8;
    let mie_phase = 0.1193662 * ((1.0 - g * g) * (1.0 + cos_theta * cos_theta))
        / ((2.0 + g * g) * pow(max(1.0 + g * g - 2.0 * g * cos_theta, 0.0001), 1.5));
    let steps = select(select(12, 16, quality >= 2u), 8, quality == 0u);
    let light_steps = select(4, 2, quality == 0u);
    var light = vec3<f32>(0.0);
    var through = vec3<f32>(1.0);
    for (var i = 0; i < 16; i++) {
        if i >= steps { break; }
        // Steps grow with distance: the dense low air gets most of them.
        let a = f32(i) / f32(steps);
        let b = f32(i + 1) / f32(steps);
        let t = length_to * (a * a + b * b) * 0.5;
        let dt = length_to * (b * b - a * a);
        let p = vec3<f32>(d.x * t, r + d.y * t, d.z * t);
        let pr = length(p);
        let air = veio_air(pr - VEIO_EARTH);
        let extinction = veio_extinction(air);
        let sun_here = veio_sun_transmittance(pr, dot(p / pr, sun), light_steps);
        let scattered = (VEIO_RAYLEIGH * air.x * rayleigh_phase + vec3<f32>(VEIO_MIE_SCATTER * air.y * mie_phase)) * sun_here;
        let step_through = exp(-extinction * dt);
        // Integrated exactly over the step (Hillaire's energy-conserving form).
        light += through * scattered * (vec3<f32>(1.0) - step_through) / max(extinction, vec3<f32>(1e-7));
        through *= step_through;
    }
    return VeioSky(light, through);
}

// The ACES filmic curve, Narkowicz's fit.
fn veio_aces(x: vec3<f32>) -> vec3<f32> {
    return clamp((x * (2.51 * x + 0.03)) / (x * (2.43 * x + 0.59) + 0.14), vec3<f32>(0.0), vec3<f32>(1.0));
}

fn veio_stars(direction: vec3<f32>) -> f32 {
    // A fixed star field: one candidate per cell of the direction cube, a few lit.
    let a = abs(direction);
    let m = max(a.x, max(a.y, a.z));
    let cube = direction / m;
    let cell = floor(cube * 180.0);
    let h = veio_hash3(cell + select(vec3<f32>(0.0), vec3<f32>(311.0), direction < vec3<f32>(0.0)));
    if h < 0.985 {
        return 0.0;
    }
    let centre = (cell + 0.5) / 180.0;
    let d = length(cube - centre) * 180.0;
    let twinkle = 0.75 + 0.25 * sin(frame.time * 2.3 + h * 600.0);
    return (1.0 - smoothstep(0.0, 0.55, d)) * mix(0.35, 1.2, (h - 0.985) / 0.015) * twinkle;
}

fn sky(input: SkyInput) -> vec4<f32> {
    let direction = input.direction;
    let sun = frame.sun_direction;
    let day = veio_daylight();
    let storm = clamp(frame.rain * 0.80 + frame.thunder * 0.15, 0.0, 0.92);
    // The physical sky and the sun's disc in light, then the film curve, then gamma.
    let physical = veio_physical_sky(direction, sun);
    let cos_sun = dot(direction, sun);
    let disc = smoothstep(0.99935, 0.99965, cos_sun);
    let limb = sqrt(clamp((cos_sun - 0.99935) / 0.00065, 0.0, 1.0));
    let above = smoothstep(-0.02, 0.02, direction.y);
    let sun_disc = physical.through * disc * mix(0.6, 1.0, limb) * 40.0 * above * (1.0 - storm);
    let light = (physical.light + sun_disc) * VEIO_SKY_EXPOSURE;
    // Night: a dark blue base with airglow at the horizon, under the stars.
    let up = max(direction.y, 0.0);
    let night = vec3<f32>(0.024, 0.036, 0.070) + vec3<f32>(0.020, 0.026, 0.040) * exp(-up * 4.0);
    var color = pow(veio_aces(light), vec3<f32>(1.0 / 2.2)) + night * (1.0 - day);
    color = mix(input.color.rgb, color, clamp(atmosphere * 1.5, 0.0, 1.0));
    // The lowest band meets the game's fog, so far terrain melts into the sky without a seam.
    let low = 1.0 - smoothstep(0.0, 0.10, direction.y);
    color = mix(color, frame.fog_color, low * 0.45);
    // Industrial smog yellows and greys the sky near polluted factories.
    let smog = veio_smog();
    color = mix(color, vec3<f32>(0.52, 0.48, 0.40) * mix(0.25, 1.0, day), smog * 0.45 * day * (1.0 - smoothstep(0.0, 0.6, direction.y)));
    color = mix(color, frame.fog_color, storm);
    let sun_light = exp(-vec3<f32>(0.20, 0.46, 1.0) * 0.30 / (max(sun.y, 0.0) + 0.12));

    // Stars at night, above the clouds' gaps.
    var star = 0.0;
    if day < 0.6 && direction.y > 0.0 {
        star = veio_stars(direction) * (1.0 - day / 0.6) * (1.0 - storm);
    }

    if clouds && !(volumetric_clouds && quality >= 1u) && direction.y > 0.012 {
        // A cloud plane 1,200 blocks up: far away it meets the horizon in perspective.
        let t = 1200.0 / direction.y;
        let p = veio_cloud_origin() + direction.xz * t;
        let density = veio_cloud_density(p);
        // A second, offset tap toward the sun shades the clouds' far sides.
        let toward = veio_cloud_density(p + normalize(sun.xz + vec2<f32>(0.0001)) * 140.0);
        let lit = clamp(1.0 - (toward - density) * 1.0, 0.35, 1.0);
        let sun_colour = mix(vec3<f32>(1.0, 0.97, 0.92), vec3<f32>(1.0, 0.58, 0.32), veio_low_sun());
        let ambient = veio_atmosphere(vec3<f32>(0.0, 1.0, 0.0)) * 1.2 + vec3<f32>(0.05);
        var cloud = ambient * 0.6 + sun_colour * lit * 0.85 * day;
        // Silver lining toward the sun.
        cloud += sun_light * pow(max(cos_sun, 0.0), 12.0) * 0.6 * day * (1.0 - density);
        cloud = mix(cloud, vec3<f32>(0.012, 0.016, 0.026) + ambient * 0.25, 1.0 - max(day, 0.04));
        cloud = mix(cloud, frame.fog_color * 0.9, storm * 0.7);
        cloud = mix(cloud, vec3<f32>(0.45, 0.42, 0.36), smog * 0.4);
        let fade = smoothstep(0.02, 0.22, direction.y);
        let alpha = density * fade * (0.90 - 0.15 * storm);
        color = mix(color, cloud, alpha);
        star *= 1.0 - density;
    }
    color += vec3<f32>(star);
    return vec4<f32>(max(color, vec3<f32>(0.0)), input.color.a);
}
