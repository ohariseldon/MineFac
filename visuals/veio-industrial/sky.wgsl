override clouds: bool = true;

// The camera's ground position wrapped to keep precision far from the origin.
fn veio_cloud_origin() -> vec2<f32> {
    return frame.camera_position.xz - floor(frame.camera_position.xz / 8192.0) * 8192.0;
}

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
    return (1.0 - smoothstep(0.0, 0.45, d)) * (h - 0.985) / 0.015 * twinkle;
}

fn sky(input: SkyInput) -> vec4<f32> {
    let direction = input.direction;
    let sun = frame.sun_direction;
    let day = veio_daylight();
    let storm = clamp(frame.rain * 0.80 + frame.thunder * 0.15, 0.0, 0.92);
    var color = mix(input.color.rgb, veio_atmosphere(direction), atmosphere);
    // Industrial smog yellows and greys the sky near polluted factories.
    let smog = veio_smog();
    color = mix(color, vec3<f32>(0.52, 0.48, 0.40) * mix(0.25, 1.0, day), smog * 0.45 * (1.0 - smoothstep(0.0, 0.6, direction.y)));
    color = mix(color, frame.fog_color, storm);

    // The sun's disc, with limb darkening, a glow and the colour of the air it crosses.
    let cos_sun = dot(direction, sun);
    let sun_light = exp(-vec3<f32>(0.20, 0.46, 1.0) * 0.30 / (max(sun.y, 0.0) + 0.12));
    let disc = smoothstep(0.99935, 0.99965, cos_sun);
    let limb = sqrt(clamp((cos_sun - 0.99935) / 0.00065, 0.0, 1.0));
    let glow = pow(max(cos_sun, 0.0), 900.0) * 0.6 + pow(max(cos_sun, 0.0), 40.0) * 0.10;
    let above = smoothstep(-0.02, 0.02, direction.y);
    let sun_visible = (1.0 - storm) * above;
    color += sun_light * (disc * mix(0.65, 1.0, limb) * 4.0 + glow) * sun_visible;

    // Stars at night, above the clouds' gaps.
    var star = 0.0;
    if day < 0.6 && direction.y > 0.0 {
        star = veio_stars(direction) * (1.0 - day / 0.6) * (1.0 - storm);
    }

    if clouds && direction.y > 0.012 {
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
