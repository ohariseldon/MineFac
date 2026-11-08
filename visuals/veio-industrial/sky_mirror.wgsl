// Sky and mirrors (half size), one target for two effects that never share a pixel, so the
// pack stays within the four post outputs a pack may write: where the sky shows, the
// volumetric clouds (alpha 0..1); on terrain, the screen reflections (alpha 0..-1). The last
// pass reads it with veio_sky_mirror, which keeps the two apart at the edges.

// Volumetric clouds (half size): a cumulus layer 560 to 1,150 blocks up, ray marched through a
// 2D field of towers eroded by 3D noise and lit by the sun through themselves (Beer's law with
// the powder effect, a forward and a back scattering lobe). rgb is the light the clouds send
// toward the camera, a how much of the sky behind they hide. Only where the sky shows.
// 16 steps on Equilibrado, 28 on Alto; Leve keeps the flat clouds of the sky stage.

fn veio_cloud_density(p: vec3<f32>, coverage: f32, detail: bool) -> f32 {
    let h = clamp((p.y - VEIO_CLOUD_BASE) / (VEIO_CLOUD_TOP - VEIO_CLOUD_BASE), 0.0, 1.0);
    // Flat dark bases, rounded towering tops: the column narrows as it rises.
    let profile = smoothstep(0.0, 0.16, h) * (1.0 - smoothstep(0.25 + 0.65 * coverage, 1.0, h));
    var d = coverage * profile;
    if detail && d > 0.0 {
        let drift = vec3<f32>(veio_cloud_wind().x, frame.time * 0.8, veio_cloud_wind().y);
        let q = (p + drift * 1.4) / 150.0;
        let erosion = veio_noise3(q) * 0.6 + veio_noise3(q * 2.9 + vec3<f32>(3.1, 1.7, -2.3)) * 0.4;
        // The edges fray into cauliflower lumps, the cores stay solid.
        d = d - erosion * (0.75 - 0.35 * h) * (1.0 - d * 0.4);
    }
    return clamp(d * 1.6 - 0.12, 0.0, 1.0);
}

fn veio_volumetric_clouds(input: PostInput) -> vec4<f32> {
    if quality == 0u || !clouds || !volumetric_clouds || frame.dimension != 0 {
        return vec4<f32>(0.0);
    }
    let direction = normalize(view_position(input.uv, 0.5));
    if direction.y < 0.015 {
        return vec4<f32>(0.0);
    }
    let camera_y = frame.camera_position.y;
    let t0 = (VEIO_CLOUD_BASE - camera_y) / direction.y;
    let t1 = (VEIO_CLOUD_TOP - camera_y) / direction.y;
    let far = 16000.0;
    if t0 > far {
        return vec4<f32>(0.0);
    }
    let steps = select(16, 28, quality >= 2u);
    // Long slanting rays near the horizon march only the first 4,200 blocks of the layer.
    let span = min(min(t1, far) - t0, 4200.0);
    let step_length = span / f32(steps);
    let jitter = fract(52.9829189 * fract(dot(floor(input.pixel), vec2<f32>(0.06711056, 0.00583715))));
    let origin = vec3<f32>(veio_cloud_origin().x, camera_y, veio_cloud_origin().y);

    let sun = veio_key_direction();
    let day = veio_daylight();
    let cos_theta = dot(direction, sun);
    let g1 = 0.65;
    let g2 = -0.25;
    let forward = (1.0 - g1 * g1) / pow(max(1.0 + g1 * g1 - 2.0 * g1 * cos_theta, 0.0001), 1.5);
    let back = (1.0 - g2 * g2) / pow(max(1.0 + g2 * g2 - 2.0 * g2 * cos_theta, 0.0001), 1.5);
    let phase = mix(back, forward, 0.7) * 0.0796;
    // Sunlight after crossing the air to the clouds: white high, orange low.
    let sun_air = exp(-vec3<f32>(0.20, 0.46, 1.0) * 0.30 / (max(sun.y, 0.0) + 0.10));
    let moon = vec3<f32>(0.10, 0.13, 0.20);
    let sun_colour = mix(moon, vec3<f32>(1.0, 0.98, 0.94) * sun_air * 2.6, day);
    let sky_top = veio_atmosphere(vec3<f32>(0.0, 1.0, 0.0)) * 1.15 + vec3<f32>(0.03);
    let storm = clamp(frame.rain * 0.7 + frame.thunder * 0.2, 0.0, 0.9);

    var transmittance = 1.0;
    var light = vec3<f32>(0.0);
    let sigma = 0.035;
    let light_steps = select(3, 5, quality >= 2u);
    for (var i = 0; i < 28; i++) {
        if i >= steps || transmittance < 0.03 { break; }
        let t = t0 + (f32(i) + jitter) * step_length;
        let p = origin + direction * t;
        let coverage = veio_cloud_coverage(p.xz);
        if coverage <= 0.001 { continue; }
        let density = veio_cloud_density(p, coverage, true);
        if density <= 0.001 { continue; }
        // Toward the sun: how much cloud the light crosses to reach this point.
        var optical = 0.0;
        for (var j = 0; j < 5; j++) {
            if j >= light_steps { break; }
            let s = p + sun * (40.0 + 70.0 * f32(j) * f32(j + 1) * 0.5);
            optical += veio_cloud_density(s, veio_cloud_coverage(s.xz), false) * (40.0 + 70.0 * f32(j));
        }
        // A second, softer extinction stands for light scattered many times inside the cloud:
        // undersides stay grey instead of black.
        let to_sun = exp(-optical * sigma * 0.9) + 0.35 * exp(-optical * sigma * 0.15);
        let powder = 1.0 - exp(-density * step_length * sigma * 2.0);
        let h = clamp((p.y - VEIO_CLOUD_BASE) / (VEIO_CLOUD_TOP - VEIO_CLOUD_BASE), 0.0, 1.0);
        let direct = sun_colour * to_sun * mix(1.0, powder * 2.0, 0.5) * phase * 6.0;
        let ambient = sky_top * mix(0.8, 1.3, h);
        var sample = direct * (1.0 - storm * 0.8) + ambient;
        sample = mix(sample, frame.fog_color * 0.75, storm * 0.6);
        let absorb = exp(-density * sigma * step_length);
        light += transmittance * (1.0 - absorb) * sample;
        transmittance *= absorb;
    }
    // Far clouds sink into the haze of the horizon.
    let haze = smoothstep(0.015, 0.20, direction.y);
    let air = veio_atmosphere(direction);
    let alpha = (1.0 - transmittance) * haze;
    let colour = mix(air * (1.0 - transmittance), light, haze * 0.9 + 0.1);
    let smog = veio_smog();
    let tinted = mix(colour, vec3<f32>(0.45, 0.42, 0.36) * alpha, smog * 0.4);
    return vec4<f32>(tinted, alpha);
}


// Screen reflections (half size): wet streets after rain, polished metal and glass mirror the
// lamps, machines and sky already on screen. Only surfaces the material marked smooth or wet.
// rgb the reflected picture, a how much of it to show. 12 steps on Equilibrado, 24 on Alto.
fn veio_screen_reflection(input: PostInput) -> vec4<f32> {
    if quality == 0u || !screen_reflections || frame.eye_in != 0u {
        return vec4<f32>(0.0);
    }
    let surface = surface_at(vec2<i32>(input.uv * vec2<f32>(textureDimensions(scene_surface))));
    if !surface.present || surface.smoothness < 0.12 || surface.data > 0.5 {
        return vec4<f32>(0.0);
    }
    let depth = depth_at(input.uv);
    let position = view_position(input.uv, depth);
    let distance = length(position);
    if distance > 64.0 {
        return vec4<f32>(0.0);
    }
    let view = position / max(distance, 0.001);
    let n = surface.normal;
    let ray = reflect(view, n);
    let toward_camera = dot(ray, view);
    if toward_camera < -0.2 {
        return vec4<f32>(0.0);
    }
    let steps = select(12, 24, quality >= 2u);
    let jitter = veio_hash2(floor(input.pixel) + vec2<f32>(3.1, 7.7));
    var step_length = select(0.55, 0.32, quality >= 2u) * (1.0 + distance * 0.02);
    var p = position + n * 0.05 + ray * step_length * jitter;
    var hit = vec2<f32>(-1.0);
    var last = p;
    for (var i = 0; i < 24; i++) {
        if i >= steps { break; }
        last = p;
        p += ray * step_length;
        step_length *= 1.12;
        let screen = veio_project(p);
        if screen.z == 0.0 || any(screen.xy < vec2<f32>(0.0)) || any(screen.xy > vec2<f32>(1.0)) {
            break;
        }
        let d = depth_at(screen.xy);
        if d <= 0.000001 {
            continue;
        }
        let behind = length(p) - length(view_position(screen.xy, d));
        if behind > 0.0 && behind < step_length * 2.5 + 0.3 {
            // Three halvings between the last free point and this one.
            var a = last;
            var c = p;
            for (var j = 0; j < 3; j++) {
                let m = (a + c) * 0.5;
                let sm = veio_project(m);
                let dm = depth_at(sm.xy);
                if length(m) - length(view_position(sm.xy, dm)) > 0.0 { c = m; } else { a = m; }
            }
            hit = veio_project(c).xy;
            break;
        }
    }
    if hit.x < 0.0 {
        return vec4<f32>(0.0);
    }
    let reflected = textureSampleLevel(scene_color, linear_sampler, hit, 0.0).rgb;
    let edge = min(min(hit.x, 1.0 - hit.x), min(hit.y, 1.0 - hit.y));
    let fresnel = 0.04 + 0.96 * veio_pow5(1.0 - clamp(-dot(view, n), 0.0, 1.0));
    let weight = smoothstep(0.0, 0.08, edge) * (1.0 - smoothstep(40.0, 64.0, distance))
        * surface.smoothness * mix(0.35, 1.0, fresnel);
    let shown = clamp(weight, 0.0, 1.0);
    if shown <= 0.0 {
        return vec4<f32>(0.0);
    }
    // A negative alpha marks a reflection, so the last pass never mixes it with the clouds.
    return vec4<f32>(reflected, -shown);
}

fn post(input: PostInput) -> vec4<f32> {
    if quality == 0u {
        return vec4<f32>(0.0);
    }
    if depth_at(input.uv) <= 0.000001 {
        return veio_volumetric_clouds(input);
    }
    return veio_screen_reflection(input);
}
