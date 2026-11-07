override metal_highlights: f32 = 0.65;
override water_detail: bool = true;

fn veio_sun_visibility(position: vec3<f32>, normal: vec3<f32>) -> f32 {
    if !shadows || frame.has_shadow == 0u {
        return 1.0;
    }
    // One bilinear comparison (2x2 PCF); no sample loop, cascade or screen-space trace.
    let offset = max(0.025, frame.shadow_center_texel * 1.2);
    let p = shadow_position(position + normal * offset);
    if any(p.xy <= vec2<f32>(0.002)) || any(p.xy >= vec2<f32>(0.998)) || p.z <= 0.0 || p.z >= 1.0 {
        return 1.0;
    }
    let lit = textureSampleCompareLevel(shadow_map, shadow_sampler, p.xy, p.z - 0.0002);
    let distance_fade = smoothstep(frame.shadow_distance * 0.72, frame.shadow_distance, length(position.xz));
    return mix(lit, 1.0, distance_fade);
}

fn material(input: MaterialInput) -> Surface {
    let surface = input.albedo * input.tint;
    let vanilla = surface.rgb * input.lightmap;
    // Keep the Nether, End, torch lighting, night vision, biome tint and vertex AO usable.
    if frame.dimension != 0 {
        return Surface(vec4<f32>(vanilla, surface.a));
    }
    let daylight = veio_daylight() * (1.0 - 0.75 * frame.rain);
    let outdoor = input.light.y * input.light.y * daylight;
    let normal = input.normal;
    var visibility = 1.0;
    if outdoor > 0.01 {
        visibility = veio_sun_visibility(input.position, normal);
    }
    let facing = max(dot(normal, frame.sun_direction), 0.0);
    let direct = (0.86 + 0.22 * facing) * (0.62 + 0.38 * visibility);
    let diffuse = mix(vec3<f32>(0.94, 0.98, 1.035), vec3<f32>(1.035, 1.015, 0.965), visibility * facing);
    let shading = mix(vec3<f32>(1.0), diffuse * direct, outdoor * (1.0 - input.emission));
    var color = vanilla * shading;
    let view = -input.position / max(length(input.position), 0.001);
    let half_raw = view + frame.sun_direction;
    let half_light = half_raw / max(length(half_raw), 0.001);

    if input.material >= 1u && input.material <= 3u {
        // Broad, restrained highlights. Painted front panels stay readable at a distance.
        let painted = input.material == 3u;
        let copper = input.material == 2u;
        let exponent = select(24.0, 12.0, painted);
        let amount = select(0.16, 0.055, painted) * metal_highlights;
        let specular = pow(max(dot(normal, half_light), 0.0), exponent);
        let metal_color = select(vec3<f32>(0.88, 0.94, 1.0), vec3<f32>(1.0, 0.68, 0.40), copper);
        color += metal_color * specular * amount * outdoor * visibility * facing;
    }

    if water_detail && input.fluid == 1u && input.face == 1u {
        // Analytic surface normals and a sky tint; no reflected scene, extra geometry or depth reads.
        // Wrap camera position first, retaining sub-block precision far from the world origin.
        let camera = frame.camera_position.xz - floor(frame.camera_position.xz / 256.0) * 256.0;
        let p = input.position.xz + camera;
        let phase = frame.time * 0.65;
        let ripple = vec2<f32>(sin(p.x * 0.7363108 + p.y * 0.49087385 + phase), sin(p.y * 0.6135923 - p.x * 0.3681554 - phase));
        let water_normal = normalize(normal + vec3<f32>(ripple.x, 0.0, ripple.y) * 0.035);
        let fresnel = 0.025 + 0.30 * veio_pow5(1.0 - clamp(dot(water_normal, view), 0.0, 1.0));
        let sky_tint = vec3<f32>(0.35, 0.53, 0.65) * max(veio_luma(input.lightmap), 0.02);
        color = mix(color, sky_tint, fresnel * outdoor);
        let sparkle = pow(max(dot(water_normal, half_light), 0.0), 64.0);
        color += vec3<f32>(0.22, 0.23, 0.22) * sparkle * outdoor * visibility;
    }
    return Surface(vec4<f32>(max(color, vec3<f32>(0.0)), surface.a));
}
