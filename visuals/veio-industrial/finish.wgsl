override edge_smoothing: bool = true;
override color_strength: f32 = 0.65;
override sharpen: f32 = 0.35;

// Ambient occlusion from the half-size pass, upscaled along the depth: of four nearby texels,
// those at this pixel's distance count, so no dark halo leaks around the edges of things.
fn veio_occlusion(uv: vec2<f32>, distance: f32) -> f32 {
    let texel = 1.0 / vec2<f32>(textureDimensions(light));
    var sum = 0.0;
    var weights = 0.0;
    for (var i = 0; i < 4; i++) {
        let offset = vec2<f32>(f32(i & 1) - 0.5, f32(i >> 1u) - 0.5) * texel * 1.5;
        let tap = textureSampleLevel(light, linear_sampler, uv + offset, 0.0);
        let w = 1.0 / (0.05 + abs(tap.g - distance) / max(distance * 0.04, 0.05));
        sum += tap.r * w;
        weights += w;
    }
    return sum / max(weights, 0.0001);
}

// The colour of the light shafts the light pass measured (its b channel).
fn veio_rays_colour() -> vec3<f32> {
    if !shadows || frame.has_shadow == 0u {
        return mix(vec3<f32>(1.0, 0.94, 0.82), vec3<f32>(1.0, 0.62, 0.32), veio_low_sun());
    }
    return veio_key_color() * 0.8;
}

// The half-size sky and mirror target, upscaled by hand: of its four nearest texels only those
// of the wanted kind count (clouds have alpha >= 0, reflections <= 0), so no cloud bleeds onto a
// roof's edge and no reflection onto the sky. Returns the alpha as a positive amount.
fn veio_sky_mirror(uv: vec2<f32>, want_reflection: bool) -> vec4<f32> {
    let size = vec2<f32>(textureDimensions(sky_mirror));
    let at = uv * size - 0.5;
    let base = floor(at);
    let f = at - base;
    var sum = vec4<f32>(0.0);
    var weights = 0.0;
    // A 3 by 3 tent (bilinear widened by one texel) smooths the half-size edges of the clouds.
    for (var i = 0; i < 16; i++) {
        let corner = vec2<f32>(f32(i & 3), f32(i >> 2u)) - 1.0;
        let texel = vec2<i32>(clamp(base + corner, vec2<f32>(0.0), size - 1.0));
        let tap = textureLoad(sky_mirror, texel, 0);
        let d = abs(corner - f);
        let w = max(1.5 - d.x, 0.0) * max(1.5 - d.y, 0.0);
        let kind = select(tap.a >= 0.0, tap.a <= 0.0, want_reflection);
        if kind {
            sum += vec4<f32>(tap.rgb, abs(tap.a)) * w;
            weights += w;
        }
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
            // 0.5 (was 0.9): stronger left a bright rim around every block and blade.
            color += (color - around) * sharpen * 0.5 * sqrt(room) * (1.0 - blend * 3.0);
        }
    }
    if quality >= 1u && input.depth > 0.000001 {
        let distance_here = length(view_position(input.uv, input.depth));
        if ambient_occlusion > 0.0 {
            let open = veio_occlusion(input.uv, distance_here);
            color *= mix(1.0, open, ambient_occlusion);
        }
        if screen_reflections {
            let mirrored = veio_sky_mirror(input.uv, true);
            color = mix(color, mirrored.rgb, mirrored.a * 0.55);
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
            let near = min(48.0, frame.render_distance * 0.35);
            let far = max(near + 16.0, frame.render_distance * 0.88);
            // Aerial perspective: far terrain takes the colour of the air toward it.
            let view_dir = normalize((frame.view_inverse * vec4<f32>(projected.xyz, 0.0)).xyz);
            let air = mix(frame.fog_color, veio_atmosphere(vec3<f32>(view_dir.x, max(view_dir.y, 0.02), view_dir.z)), 0.35);
            let haze = smoothstep(near, far, distance) * atmosphere * exposed * effects
                * (0.14 + 0.10 * frame.rain) * mix(0.30, 1.0, veio_daylight());
            color = mix(color, air, clamp(haze, 0.0, 0.22));
        }
        let smog = veio_smog();
        if smog > 0.0 {
            // Factory smog: a brownish grey that thickens fast with distance.
            let thickness = 1.0 - exp(-distance * 0.018 * smog);
            let smog_colour = vec3<f32>(0.46, 0.43, 0.37) * mix(0.18, 1.0, veio_daylight());
            color = mix(color, smog_colour, thickness * 0.65 * exposed * effects);
        }
    }
    if quality >= 1u && clouds && volumetric_clouds && frame.dimension == 0 && input.depth <= 0.000001 {
        // The cumulus pass over the sky: the light the clouds send, and the sky they hide.
        let sky_clouds = veio_sky_mirror(input.uv, false);
        color = color * (1.0 - sky_clouds.a) + sky_clouds.rgb;
    }
    if quality >= 1u {
        if overworld {
            color += veio_rays_colour() * textureSampleLevel(light, linear_sampler, input.uv, 0.0).b;
        }
        let halo = textureSampleLevel(glow, linear_sampler, input.uv, 0.0).rgb;
        color += halo * bloom * 0.45;
    }
    // Eye adaptation from the light around the eye, eased over about 2 seconds by the game
    // (eye_light_smooth), as BSL and Complementary do with eyeBrightnessSmooth: in a cave, a
    // dark room or outdoors at night the eye opens up to about twice the light; in daylight it
    // stays at 1. Leaving a cave into the sun the picture starts bright and settles.
    let sky_here = frame.eye_light_smooth.y * frame.eye_light_smooth.y * mix(0.10, 1.0, veio_daylight());
    let lamps_here = frame.eye_light_smooth.x * frame.eye_light_smooth.x * 0.55;
    let surround = max(sky_here, lamps_here);
    let opened = select(1.0 + 1.0 * (1.0 - smoothstep(0.02, 0.75, surround)), 1.0, frame.dimension != 0);
    let night_out = (1.0 - veio_daylight()) * frame.eye_light_smooth.y * (1.0 - frame.eye_light_smooth.x);
    // Exposure works on light, not on the encoded picture.
    color = pow(max(color, vec3<f32>(0.0)), vec3<f32>(2.2)) * opened;
    color = pow(color, vec3<f32>(1.0 / 2.2));
    // Night vision of the eye (Purkinje): colours fade at night outdoors.
    let grey = veio_luma(color);
    color = mix(color, vec3<f32>(grey) * vec3<f32>(0.96, 0.99, 1.06), 0.30 * night_out);
    // Film shoulder: values above 0.8 roll off instead of clipping (the bloom adds light).
    let over = max(color - vec3<f32>(0.8), vec3<f32>(0.0));
    color = min(color, vec3<f32>(0.8)) + 0.2 * (vec3<f32>(1.0) - exp(-over / 0.2));
    // Fixed exposure and luminance-based contrast keep hues and dark interiors stable.
    // Gamma-space API: an HDR tone map here would double-correct the image.
    // Levels as a photographer sets them (measured on Luiz's captures: blacks lifted to 25/255
    // by the haze, middle grey at 74/255, flat): a black point that lifts the veil, a midtone lift,
    // then an S curve on luminance. Hues stay; dark nights keep a softer black point.
    // The option's default (0.65) gives the full grade; 1.0 goes a bit further.
    let levels = min(color_strength / 0.65, 1.5);
    let black = 0.035 * mix(0.4, 1.0, veio_daylight()) * levels;
    let bounded = clamp((color - vec3<f32>(black)) / (1.0 - black), vec3<f32>(0.0), vec3<f32>(1.0));
    let luma = veio_luma(bounded);
    let lifted = pow(luma, mix(1.0, 0.80, levels));
    let curve = lifted * lifted * (3.0 - 2.0 * lifted);
    let target_luma = mix(lifted, curve, 0.35 * levels);
    var graded = bounded * select(1.0, target_luma / max(luma, 0.00001), luma > 0.00001);
    // Time of day: warm golden hours, cool blue nights; the seasons tint the light a little.
    if frame.dimension == 0 {
        let golden = veio_low_sun() * veio_daylight();
        let night = 1.0 - veio_daylight();
        let w = veio_season_weights();
        let warmth = 1.0;
        var tint = vec3<f32>(1.0) + vec3<f32>(0.045, 0.012, -0.040) * golden * warmth + vec3<f32>(-0.010, -0.002, 0.012) * night * warmth;
        tint += vec3<f32>(0.010, 0.006, -0.012) * w.y + vec3<f32>(0.018, 0.004, -0.018) * w.z + vec3<f32>(-0.006, -0.001, 0.008) * w.w;
        graded *= mix(vec3<f32>(1.0), tint, color_strength);
    }
    graded = mix(vec3<f32>(veio_luma(graded)), graded, 1.0 + 0.20 * levels);
    if vignette > 0.0 {
        let centred = input.uv - 0.5;
        let fall = dot(centred, centred) * 1.6;
        graded *= 1.0 - vignette * 0.35 * fall * fall;
    }
    // Dither of one step of 8 bits (triangular noise): dark gradients of the night sky no longer
    // break into visible bands.
    let n = veio_hash2(input.pixel) + veio_hash2(input.pixel + vec2<f32>(17.0, 59.0)) - 1.0;
    graded += vec3<f32>(n / 255.0);
    return vec4<f32>(clamp(graded, vec3<f32>(0.0), vec3<f32>(1.0)), input.color.a);
}
