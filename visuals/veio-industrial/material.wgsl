override metal_highlights: f32 = 0.65;
override water_detail: bool = true;
override wet_surfaces: bool = true;

fn veio_visibility(position: vec3<f32>, normal: vec3<f32>, light: vec3<f32>) -> f32 {
    if !shadows || frame.has_shadow == 0u {
        return 1.0;
    }
    // One bilinear comparison, also at night. Bias scales with map size and surface slope.
    let slope = 1.0 - max(dot(normal, light), 0.0);
    let offset = max(0.025, frame.shadow_center_texel * (0.85 + 0.65 * slope));
    let p = shadow_position(position + normal * offset);
    if any(p.xy <= vec2<f32>(0.002)) || any(p.xy >= vec2<f32>(0.998)) || p.z <= 0.0 || p.z >= 1.0 {
        return 1.0;
    }
    let lit = textureSampleCompareLevel(shadow_map, shadow_sampler, p.xy, p.z - (0.00015 + 0.0002 * slope));
    let fade = smoothstep(frame.shadow_distance * 0.72, frame.shadow_distance, length(position.xz));
    return mix(lit, 1.0, fade);
}

fn veio_water_thickness(input: MaterialInput, view: vec3<f32>) -> f32 {
    // Horizon already copies this depth for native materials. Read only on translucent water.
    let size = textureDimensions(opaque_depth);
    let pixel = clamp(vec2<i32>(input.screen * vec2<f32>(size)), vec2<i32>(0), vec2<i32>(size) - vec2<i32>(1));
    let depth = textureLoad(opaque_depth, pixel, 0);
    if depth <= 0.000001 {
        return 12.0;
    }
    let uv = (vec2<f32>(pixel) + 0.5) / vec2<f32>(size);
    let clip = vec4<f32>(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0, depth, 1.0);
    let projected = frame.projection_inverse * clip;
    let local = projected.xyz / max(abs(projected.w), 0.00001);
    let behind = (frame.view_inverse * vec4<f32>(local, 0.0)).xyz;
    return clamp(dot(behind - input.position, -view), 0.0, 12.0);
}

fn material(input: MaterialInput) -> Surface {
    let surface = input.albedo * input.tint;
    let vanilla = surface.rgb * input.lightmap;
    // Other dimensions retain their own palette, fog, lighting and visibility rules.
    if frame.dimension != 0 {
        return Surface(vec4<f32>(vanilla, surface.a));
    }
    let day = veio_daylight();
    let sky_light = input.light.y * input.light.y;
    let key_strength = mix(0.16, 1.0, day) * (1.0 - 0.80 * frame.rain);
    let light = veio_key_direction();
    let normal = input.normal;
    let view = -input.position / max(length(input.position), 0.001);
    let facing = max(dot(normal, light), 0.0);
    var visibility = 1.0;
    if sky_light > 0.01 {
        visibility = veio_visibility(input.position, normal, light);
    }
    let sky_ambient = mix(vec3<f32>(0.91, 0.96, 1.06), vec3<f32>(0.97, 1.005, 1.035), day);
    let hemisphere = 0.76 + 0.12 * max(normal.y, 0.0);
    let lighting = sky_ambient * hemisphere + veio_key_color() * (0.23 * facing * visibility * key_strength);
    var color = vanilla * mix(vec3<f32>(1.0), lighting, sky_light * (1.0 - input.emission));
    // Warm only light already supplied by the game. No gathered or newly created light sources.
    let local_light = clamp(input.light.x - input.light.y * day, 0.0, 1.0) * (1.0 - input.emission);
    color *= mix(vec3<f32>(1.0), vec3<f32>(1.035, 0.995, 0.94), local_light);

    let metal = input.material == 1u || input.material == 2u;
    let painted = input.material == 3u;
    let glass = input.material == 4u;
    let foliage = input.material == 5u;
    let patina = input.material == 6u;
    var wet = 0.0;
    if wet_surfaces && !input.translucent && !foliage && input.emission < 0.1 {
        // Direct sky exposure and upward faces avoid wet floors inside buildings.
        wet = frame.wetness * smoothstep(0.98, 1.0, input.light.y) * smoothstep(0.45, 0.95, normal.y);
        color *= 1.0 - wet * select(0.14, 0.035, metal || painted);
    }
    if metal || painted || glass || patina || wet > 0.01 {
        let half_raw = view + light;
        let half_light = half_raw / max(length(half_raw), 0.001);
        var roughness = 0.82;
        var f0 = vec3<f32>(0.035);
        var amount = 0.055;
        if metal {
            roughness = select(0.38, 0.43, input.material == 2u);
            f0 = select(vec3<f32>(0.56, 0.62, 0.68), vec3<f32>(0.95, 0.64, 0.40), input.material == 2u);
            amount = 0.22;
        } else if painted {
            roughness = 0.57;
            f0 = vec3<f32>(0.045);
            amount = 0.10;
        } else if glass {
            roughness = 0.16;
            f0 = vec3<f32>(0.04);
            amount = 0.10;
        }
        // Oxidation stays a matte coating instead of returning green copper to orange metal.
        if patina { roughness = 0.86; }
        roughness = mix(roughness, min(roughness, 0.24), wet * 0.72);
        let exponent = mix(80.0, 10.0, roughness);
        let highlight = pow(max(dot(normal, half_light), 0.0), exponent);
        let fresnel = veio_fresnel(f0, max(dot(normal, view), 0.0));
        let environment = veio_environment(reflect(-view, normal));
        let exposure = sky_light * (1.0 - input.emission);
        color += environment * fresnel * (1.0 - roughness) * exposure * metal_highlights * 0.16;
        color += veio_key_color() * mix(vec3<f32>(1.0), f0, 0.55)
            * highlight * amount * exposure * visibility * facing * key_strength * metal_highlights;
    }
    if foliage {
        let backlight = max(dot(-normal, light), 0.0) * sky_light * key_strength;
        color += vanilla * vec3<f32>(0.075, 0.065, 0.025) * backlight * visibility;
    }

    var alpha = surface.a;
    if water_detail && input.fluid == 1u && input.translucent && frame.eye_in == 0u {
        var water_normal = normal;
        let distance = length(input.position);
        if input.face == 1u && distance < 72.0 {
            // Periodic coordinates retain detail far from the origin. Far water stays stable.
            let camera = frame.camera_position.xz - floor(frame.camera_position.xz / 256.0) * 256.0;
            let p = input.position.xz + camera;
            let phase = frame.time * 0.65;
            let ripple = vec2<f32>(sin(p.x * 0.7363108 + p.y * 0.49087385 + phase), sin(p.y * 0.6135923 - p.x * 0.3681554 - phase));
            let amplitude = 0.035 * (1.0 - smoothstep(16.0, 72.0, distance));
            water_normal = normalize(normal + vec3<f32>(ripple.x, 0.0, ripple.y) * amplitude);
        }
        let thickness = veio_water_thickness(input, view);
        let absorption = 1.0 - exp(-thickness * 0.16);
        let deep = mix(vanilla, vec3<f32>(0.025, 0.15, 0.18) * max(veio_luma(input.lightmap), 0.02), 0.55);
        color = mix(color, deep, absorption * 0.62);
        alpha = mix(surface.a * 0.52, max(surface.a, 0.84), absorption);
        let fresnel = 0.02 + 0.68 * veio_pow5(1.0 - clamp(dot(water_normal, view), 0.0, 1.0));
        let sky = veio_environment(reflect(-view, water_normal));
        color = mix(color, sky, fresnel * sky_light);
        alpha = max(alpha, fresnel * sky_light * 0.85);
        let half_raw = view + light;
        let half_light = half_raw / max(length(half_raw), 0.001);
        let sparkle = pow(max(dot(water_normal, half_light), 0.0), 64.0);
        color += veio_key_color() * sparkle * 0.18 * sky_light * key_strength * visibility;
    }
    // Horizon applies its original alpha rules and biome fog after this material.
    return Surface(vec4<f32>(clamp(color, vec3<f32>(0.0), vec3<f32>(1.0)), clamp(alpha, 0.0, 1.0)));
}
