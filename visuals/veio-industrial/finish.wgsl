override edge_smoothing: bool = true;
override color_strength: f32 = 0.65;
override sharpen: f32 = 0.35;

// Ambient occlusion from the half-size pass, upscaled along the depth: of four nearby texels,
// those at this pixel's distance count, so no dark halo leaks around the edges of things.
fn veio_occlusion(uv: vec2<f32>, distance: f32) -> f32 {
    let texel = 1.0 / vec2<f32>(textureDimensions(ao));
    var sum = 0.0;
    var weights = 0.0;
    for (var i = 0; i < 4; i++) {
        let offset = vec2<f32>(f32(i & 1) - 0.5, f32(i >> 1u) - 0.5) * texel * 1.5;
        let tap = textureSampleLevel(ao, linear_sampler, uv + offset, 0.0);
        let w = 1.0 / (0.05 + abs(tap.g - distance) / max(distance * 0.04, 0.05));
        sum += tap.r * w;
        weights += w;
    }
    return sum / max(weights, 0.0001);
}

// The last pass: smooth edges, then air (distance haze and factory smog), light shafts and
// bloom, the eye's adaptation, a gentle film curve, the grade and a vignette.
fn post(input: PostInput) -> vec4<f32> {
    var color = input.color.rgb;
    if edge_smoothing {
        // Four neighbours plus the centre, reused to choose the less destructive filter axis.
        let texel = 1.0 / frame.resolution;
        let left = textureSampleLevel(previous, linear_sampler, input.uv - vec2<f32>(texel.x, 0.0), 0.0).rgb;
        let right = textureSampleLevel(previous, linear_sampler, input.uv + vec2<f32>(texel.x, 0.0), 0.0).rgb;
        let up = textureSampleLevel(previous, linear_sampler, input.uv - vec2<f32>(0.0, texel.y), 0.0).rgb;
        let down = textureSampleLevel(previous, linear_sampler, input.uv + vec2<f32>(0.0, texel.y), 0.0).rgb;
        let center_luma = veio_luma(color);
        let horizontal = vec2<f32>(veio_luma(left), veio_luma(right));
        let vertical = vec2<f32>(veio_luma(up), veio_luma(down));
        let lo = min(center_luma, min(min(horizontal.x, horizontal.y), min(vertical.x, vertical.y)));
        let hi = max(center_luma, max(max(horizontal.x, horizontal.y), max(vertical.x, vertical.y)));
        let along_x = abs(horizontal.x - horizontal.y) < abs(vertical.x - vertical.y);
        let filtered = select((up + down) * 0.5, (left + right) * 0.5, along_x);
        let average = (horizontal.x + horizontal.y + vertical.x + vertical.y) * 0.25;
        let subpixel = clamp(abs(center_luma - average) / max(hi - lo, 0.02), 0.0, 1.0);
        let blend = smoothstep(0.055, 0.20, hi - lo) * (0.08 + 0.24 * subpixel);
        color = mix(color, filtered, blend);
        if quality >= 1u && sharpen > 0.0 {
            // Contrast-aware sharpening: block textures stay crisp at a distance, edges the
            // smoothing just softened are left alone.
            let around = (left + right + up + down) * 0.25;
            let room = clamp(min(lo, 1.0 - hi) / max(hi, 0.05), 0.0, 1.0);
            color += (color - around) * sharpen * 0.9 * sqrt(room) * (1.0 - blend * 3.0);
        }
    }
    if quality >= 1u && input.depth > 0.000001 {
        let distance_here = length(view_position(input.uv, input.depth));
        if ambient_occlusion > 0.0 {
            let open = veio_occlusion(input.uv, distance_here);
            color *= mix(1.0, open, ambient_occlusion);
        }
        if screen_reflections {
            let mirrored = textureSampleLevel(ssr, linear_sampler, input.uv, 0.0);
            color = mix(color, mirrored.rgb, mirrored.a * 0.75);
        }
    }
    let overworld = frame.dimension == 0 && frame.eye_in == 0u;
    if overworld && input.depth > 0.000001 {
        // Distance haze and smog from depth already drawn: no volume, ray march or extra pass.
        let clip = vec4<f32>(input.uv.x * 2.0 - 1.0, 1.0 - input.uv.y * 2.0, input.depth, 1.0);
        let projected = frame.projection_inverse * clip;
        let distance = length(projected.xyz / max(abs(projected.w), 0.00001));
        let exposed = frame.eye_light_smooth.y * frame.eye_light_smooth.y;
        let effects = (1.0 - frame.blindness) * (1.0 - frame.darkness) * (1.0 - 0.6 * frame.night_vision);
        if atmosphere > 0.0 {
            let near = min(24.0, frame.render_distance * 0.25);
            let far = max(near + 16.0, frame.render_distance * 0.88);
            // Aerial perspective: far terrain takes the colour of the air toward it.
            let view_dir = normalize((frame.view_inverse * vec4<f32>(projected.xyz, 0.0)).xyz);
            let air = mix(frame.fog_color, veio_atmosphere(vec3<f32>(view_dir.x, max(view_dir.y, 0.02), view_dir.z)), 0.55);
            let haze = smoothstep(near, far, distance) * atmosphere * exposed * effects
                * (0.14 + 0.10 * frame.rain) * mix(0.30, 1.0, veio_daylight());
            color = mix(color, air, clamp(haze, 0.0, 0.30));
        }
        let smog = veio_smog();
        if smog > 0.0 {
            // Factory smog: a brownish grey that thickens fast with distance.
            let thickness = 1.0 - exp(-distance * 0.018 * smog);
            let smog_colour = vec3<f32>(0.46, 0.43, 0.37) * mix(0.18, 1.0, veio_daylight());
            color = mix(color, smog_colour, thickness * 0.65 * exposed * effects);
        }
    }
    if quality >= 1u {
        if overworld {
            color += textureSampleLevel(rays, linear_sampler, input.uv, 0.0).rgb;
        }
        let halo = textureSampleLevel(glow, linear_sampler, input.uv, 0.0).rgb;
        color += halo * bloom * 0.55;
    }
    // Eye adaptation: dark interiors open up a little, bright outdoors stays as it is.
    let indoor = 1.0 - frame.eye_light_smooth.y;
    let lit = max(frame.eye_light_smooth.x, frame.eye_light_smooth.y * veio_daylight());
    color *= 1.0 + 0.12 * indoor * (1.0 - lit);
    // Film shoulder: values above 0.8 roll off instead of clipping (the bloom adds light).
    let over = max(color - vec3<f32>(0.8), vec3<f32>(0.0));
    color = min(color, vec3<f32>(0.8)) + 0.2 * (vec3<f32>(1.0) - exp(-over / 0.2));
    // Fixed exposure and luminance-based contrast keep hues and dark interiors stable.
    // Gamma-space API: an HDR tone map here would double-correct the image.
    let bounded = clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));
    let luma = veio_luma(bounded);
    let curve = luma * luma * (3.0 - 2.0 * luma);
    let target_luma = mix(luma, curve, 0.12 * color_strength);
    var graded = bounded * select(1.0, target_luma / max(luma, 0.00001), luma > 0.00001);
    // Time of day: warm golden hours, cool blue nights; the seasons tint the light a little.
    if frame.dimension == 0 {
        let golden = veio_low_sun() * veio_daylight();
        let night = 1.0 - veio_daylight();
        let w = veio_season_weights();
        var tint = vec3<f32>(1.0) + vec3<f32>(0.045, 0.012, -0.040) * golden + vec3<f32>(-0.030, -0.008, 0.040) * night;
        tint += vec3<f32>(0.010, 0.006, -0.012) * w.y + vec3<f32>(0.018, 0.004, -0.018) * w.z + vec3<f32>(-0.016, -0.004, 0.020) * w.w;
        graded *= mix(vec3<f32>(1.0), tint, color_strength);
    }
    graded = mix(vec3<f32>(veio_luma(graded)), graded, 1.0 + 0.06 * color_strength);
    if vignette > 0.0 {
        let centred = input.uv - 0.5;
        let fall = dot(centred, centred) * 1.6;
        graded *= 1.0 - vignette * 0.35 * fall * fall;
    }
    return vec4<f32>(clamp(graded, vec3<f32>(0.0), vec3<f32>(1.0)), input.color.a);
}
