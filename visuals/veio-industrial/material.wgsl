override metal_highlights: f32 = 0.65;
override water_detail: bool = true;
override wet_surfaces: bool = true;
override reflections: bool = false;
override colored_lights: bool = true;
override relief: f32 = 0.6;

// Relief: the texture's own detail turned into a surface normal near the camera, so the sun
// grazing a wall shows each stone and plank. LabPBR normal maps win when a resource pack has
// them. Two extra texture reads; off on Leve and past 32 blocks.
fn veio_relief_normal(input: MaterialInput, distance: f32, uv: vec2<f32>, albedo: vec3<f32>) -> vec3<f32> {
    let n = input.normal;
    if quality == 0u || relief <= 0.0 || input.translucent || distance > 32.0 || input.emission > 0.5 {
        return n;
    }
    let t = input.tangent.xyz;
    let b = cross(n, t) * input.tangent.w;
    let fade = relief * (1.0 - smoothstep(18.0, 32.0, distance));
    if block_pbr_maps() {
        let m = block_normal_map(uv).rg * 2.0 - 1.0;
        let z = sqrt(max(1.0 - dot(m, m), 0.0));
        let mapped = normalize(t * m.x + b * m.y + n * z);
        return normalize(mix(n, mapped, min(fade * 1.4, 1.0)));
    }
    // Brighter texels stand out, darker ones (mortar, grooves, knots) sink in.
    let lod = block_texture_lod();
    let texel = 1.0 / block_texture_size();
    let centre = veio_luma(albedo);
    let across = veio_luma(block_texture_level(fract(uv + vec2<f32>(texel.x, 0.0)), lod).rgb);
    let down = veio_luma(block_texture_level(fract(uv + vec2<f32>(0.0, texel.y)), lod).rgb);
    // Mip levels blur the steps: keep the slope the same in blocks at every distance.
    let gradient = vec2<f32>(across - centre, down - centre) * (2.2 / exp2(lod * 0.5));
    return normalize(n - (t * gradient.x + b * gradient.y) * fade);
}

// Colour of the light given by torches, lava, lanterns and Veio's working machines:
// the pack's [[lights]] colours and the frame's coloured lights (mods, held items).
fn veio_block_light(input: MaterialInput) -> vec3<f32> {
    if !colored_lights || quality == 0u {
        return vec3<f32>(1.0);
    }
    let tint = block_light_color(input.position);
    // Untinted sources stay a warm white rather than vanilla's neutral.
    let base = mix(vec3<f32>(1.10, 0.92, 0.72), tint.rgb, min(tint.a, 1.0));
    return base;
}

// Seasons on foliage: autumn colours vary block by block, winter dulls and frosts.
fn veio_seasonal_foliage(color: vec3<f32>, input: MaterialInput, leaves: bool) -> vec3<f32> {
    if !seasons || !veio_has_mod() {
        return color;
    }
    let w = veio_season_weights();
    let world = floor(input.position + frame.camera_position - floor(frame.camera_position / 1024.0) * 1024.0 + 0.0001);
    let h = veio_hash3(world);
    let luma = veio_luma(color);
    // Autumn: leaves turn yellow, orange or red; grass dries to straw.
    let autumn_leaf = mix(mix(vec3<f32>(1.05, 0.78, 0.22), vec3<f32>(0.98, 0.45, 0.12), smoothstep(0.30, 0.65, h)),
        vec3<f32>(0.70, 0.16, 0.08), smoothstep(0.80, 0.95, h)) * luma * 1.9;
    let autumn_grass = mix(color, vec3<f32>(0.78, 0.70, 0.40) * luma * 1.6, 0.45);
    // With the mod the biome colours already carry the season (the game recolours grass and
    // leaves): here only the variety from tree to tree is added, not the season again.
    let native = select(1.0, 0.35, frame.mods[1].x > 0.5);
    let autumn = select(mix(color, autumn_grass, native),
        mix(color, autumn_leaf, (0.35 + 0.55 * smoothstep(0.10, 0.40, h)) * mix(0.55, 1.0, native)), leaves);
    // Winter: dull and frosty, colder biomes and heights more so.
    let cold = clamp(0.6 - frame.temperature * 0.5, 0.2, 1.0);
    let winter = mix(color, vec3<f32>(luma) * vec3<f32>(0.92, 0.96, 1.02) * 1.05, 0.55 * cold * native);
    // Spring: fresher, slightly brighter green.
    let spring = color * mix(vec3<f32>(1.0), vec3<f32>(0.97, 1.06, 0.95), native);
    return spring * w.x + color * w.y + autumn * w.z + winter * w.w;
}

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
    let reference = p.z - (0.00015 + 0.0002 * slope);
    var lit = 0.0;
    if quality == 0u {
        lit = textureSampleCompareLevel(shadow_map, shadow_sampler, p.xy, reference);
    } else {
        // Soft shadows: a rotated Poisson disc of bilinear comparisons. On Alto the disc widens
        // with the distance to whatever casts the shadow (a branch near the ground stays sharp,
        // a tower's shadow far below goes soft), like the sun's real penumbra.
        let texel = 1.0 / max(frame.shadow_resolution, 256.0);
        let spin = veio_hash2(floor(position.xz * 16.0) + floor(position.y * 16.0)) * 6.2831853;
        let rotation = mat2x2<f32>(cos(spin), sin(spin), -sin(spin), cos(spin));
        var radius = 1.6 * texel;
        if quality >= 2u {
            var blocker = 0.0;
            var found = 0.0;
            let size = vec2<f32>(textureDimensions(shadow_map));
            for (var b = 0; b < 4; b++) {
                let o = rotation * vec2<f32>(f32(b & 1) * 2.0 - 1.0, f32(b >> 1u) * 2.0 - 1.0) * 3.0 * texel;
                let d = textureLoad(shadow_map, vec2<i32>(clamp((p.xy + o) * size, vec2<f32>(0.0), size - 1.0)), 0);
                let blocks = select(0.0, 1.0, d < reference);
                blocker += d * blocks;
                found += blocks;
            }
            if found > 0.0 {
                let gap = reference - blocker / found;
                radius = texel * clamp(1.2 + gap * 900.0, 1.2, 6.0);
            }
        }
        let taps = select(6, 12, quality >= 2u);
        for (var i = 0; i < 12; i++) {
            if i >= taps { break; }
            let k = (f32(i) + 0.5) / f32(taps);
            let angle = f32(i) * 2.3999632;
            let o = rotation * vec2<f32>(cos(angle), sin(angle)) * sqrt(k) * radius;
            lit += textureSampleCompareLevel(shadow_map, shadow_sampler, p.xy + o, reference);
        }
        lit /= f32(taps);
    }
    let fade = smoothstep(frame.shadow_distance * 0.72, frame.shadow_distance, length(position.xz));
    return mix(lit, 1.0, fade);
}

fn veio_water_thickness(input: MaterialInput, view: vec3<f32>, screen: vec2<f32>) -> f32 {
    // Horizon already copies this depth for native materials. Read only on translucent water.
    let size = textureDimensions(opaque_depth);
    let pixel = clamp(vec2<i32>(screen * vec2<f32>(size)), vec2<i32>(0), vec2<i32>(size) - vec2<i32>(1));
    let depth = textureLoad(opaque_depth, pixel, 0);
    if depth <= 0.000001 {
        return 12.0;
    }
    let uv = (vec2<f32>(pixel) + 0.5) / vec2<f32>(size);
    let clip = vec4<f32>(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0, depth, 1.0);
    let projected = frame.projection_inverse * clip;
    let local = projected.xyz / max(abs(projected.w), 0.00001);
    let behind = (frame.view_inverse * vec4<f32>(local, 0.0)).xyz;
    return clamp(dot(behind - input.position, -view), -1.0, 12.0);
}

override parallax: f32 = 0.7;

// Parallax: the texture's darker texels (mortar, the gaps between cobbles and planks) are
// read as lower ground, and the view ray is followed into them so each stone stands out with
// real depth as the camera moves. 8 steps within 12 blocks on Equilibrado, 14 within 20 on Alto.
// Returns the texture coordinate where the ray meets the relief.
fn veio_parallax(input: MaterialInput, view: vec3<f32>, distance: f32, foliage: bool) -> vec2<f32> {
    let reach = select(12.0, 20.0, quality >= 2u);
    if quality == 0u || parallax <= 0.0 || input.translucent || foliage || distance > reach || input.emission > 0.5 || input.displaced > 0.5 {
        return input.uv;
    }
    let n = input.normal;
    let t = input.tangent.xyz;
    let b = cross(n, t) * input.tangent.w;
    let local = vec3<f32>(dot(view, t), dot(view, b), dot(view, n));
    if local.z < 0.12 {
        return input.uv;
    }
    let fade = 1.0 - smoothstep(reach * 0.6, reach, distance);
    let depth = 0.055 * parallax * fade;
    let steps = select(8, 14, quality >= 2u);
    let stride = -local.xy / max(local.z, 0.25) * depth / f32(steps);
    let lod = block_texture_lod() + 0.5;
    var uv = input.uv;
    var layer = 0.0;
    let layer_step = 1.0 / f32(steps);
    var height = 1.0 - veio_luma(block_texture_level(fract(uv), lod).rgb) * 1.6;
    var previous_uv = uv;
    var previous_gap = 0.0;
    for (var i = 0; i < 14; i++) {
        if i >= steps || layer >= height { break; }
        previous_uv = uv;
        previous_gap = height - layer;
        uv += stride;
        layer += layer_step;
        height = 1.0 - veio_luma(block_texture_level(fract(uv), lod).rgb) * 1.6;
    }
    // Between the last step above the relief and the first below it.
    let after = layer - height;
    let w = previous_gap / max(previous_gap + after, 0.0001);
    return fract(mix(previous_uv, uv, clamp(w, 0.0, 1.0)));
}

// Shadows of the cumulus layer drifting over the land: the sun's ray from this point up to
// the cloud base, through the same field the cloud pass draws.
fn veio_cloud_shadow(position: vec3<f32>, light: vec3<f32>) -> f32 {
    if quality == 0u || !clouds || !volumetric_clouds || light.y < 0.05 {
        return 1.0;
    }
    let world_y = position.y + frame.camera_position.y;
    let up = (VEIO_CLOUD_BASE + 120.0 - world_y) / light.y;
    let xz = veio_cloud_origin() + position.xz + light.xz * up;
    let cover = veio_cloud_coverage(xz);
    return 1.0 - smoothstep(0.15, 0.75, cover) * 0.72;
}

fn material(input: MaterialInput) -> Surface {
    let foliage_early = input.material == 5u || input.material >= 7u && input.material <= 9u;
    let view_early = -input.position / max(length(input.position), 0.001);
    let parallax_uv = veio_parallax(input, view_early, length(input.position), foliage_early);
    var albedo = input.albedo;
    if any(parallax_uv != input.uv) {
        albedo = vec4<f32>(block_texture(parallax_uv).rgb, input.albedo.a);
    }
    let surface = albedo * input.tint;
    let vanilla = surface.rgb * input.lightmap;
    // Other dimensions retain their own palette, fog, lighting and visibility rules.
    if frame.dimension != 0 {
        return Surface(vec4<f32>(vanilla, surface.a));
    }
    let day = veio_daylight();
    let sky_light = input.light.y * input.light.y;
    let key_strength = mix(0.16, 1.0, day) * (1.0 - 0.80 * frame.rain);
    let light = veio_key_direction();
    let distance_here = length(input.position);
    let geometric = input.normal;
    let normal = veio_relief_normal(input, distance_here, parallax_uv, albedo.rgb);
    let view = -input.position / max(distance_here, 0.001);
    // The face itself decides what the sun can reach; relief only shades inside it.
    let facing = max(dot(normal, light), 0.0) * smoothstep(-0.05, 0.10, dot(geometric, light));
    var visibility = 1.0;
    if sky_light > 0.01 {
        visibility = veio_visibility(input.position, geometric, light) * veio_cloud_shadow(input.position, light);
    }
    // Equilibrado and Alto: a stronger sun against a bluer sky-lit shade, like a real
    // afternoon. Leve keeps the soft 1.x balance.
    let contrast = select(0.0, 1.0, quality >= 1u);
    let sky_ambient = mix(vec3<f32>(0.91, 0.96, 1.06), mix(vec3<f32>(0.97, 1.005, 1.035), vec3<f32>(0.90, 0.97, 1.10), contrast), day);
    let hemisphere = mix(0.76, 0.66 + 0.10 * frame.rain, contrast) + 0.12 * max(normal.y, 0.0);
    let key = mix(0.23, 0.44, contrast);
    let lighting = sky_ambient * hemisphere + veio_key_color() * (key * facing * visibility * key_strength);
    var color = vanilla * mix(vec3<f32>(1.0), lighting, sky_light * (1.0 - input.emission));
    // Warm only light already supplied by the game. No gathered or newly created light sources.
    let local_light = clamp(input.light.x - input.light.y * day, 0.0, 1.0) * (1.0 - input.emission);
    color *= mix(vec3<f32>(1.0), veio_block_light(input), local_light);
    if colored_lights {
        // Mods' visual lights (Veio's furnaces) and held light sources, already coloured.
        color += surface.rgb * input.colored_light * 0.85 * (1.0 - input.emission);
    }

    let metal = input.material == 1u || input.material == 2u;
    let painted = input.material == 3u;
    let glass = input.material == 4u;
    let foliage = input.material == 5u || input.material >= 7u && input.material <= 9u;
    let patina = input.material == 6u;
    var wet = 0.0;
    if wet_surfaces && !input.translucent && !foliage && input.emission < 0.1 {
        // Direct sky exposure and upward faces avoid wet floors inside buildings.
        wet = frame.wetness * smoothstep(0.98, 1.0, input.light.y) * smoothstep(0.45, 0.95, normal.y);
        color *= 1.0 - wet * select(0.14, 0.035, metal || painted);
    }
    var smoothness = wet * 0.75;
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
        smoothness = max(smoothness, (1.0 - roughness) * select(0.6, 1.0, metal));
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
        color = veio_seasonal_foliage(color, input, input.material == 7u);
        let backlight = max(dot(-normal, light), 0.0) * sky_light * key_strength;
        color += vanilla * vec3<f32>(0.075, 0.065, 0.025) * backlight * visibility;
    }

    if !input.translucent {
        // For the post passes: the relief normal shades corners (AO) and smooth or wet
        // surfaces get screen reflections. data 1 marks foliage, which neither touches.
        set_surface(normal, clamp(smoothness, 0.0, 1.0), select(0.0, 1.0, foliage));
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
            var ripple = vec2<f32>(sin(p.x * 0.7363108 + p.y * 0.49087385 + phase), sin(p.y * 0.6135923 - p.x * 0.3681554 - phase));
            if quality >= 1u {
                // Smaller wind ripples over the swell: two noise octaves drifting with the wind.
                let drift = vec2<f32>(frame.time * 0.9, frame.time * 0.4) * (0.5 + veio_wind());
                let e = 0.35;
                let q = p * 2.3 + drift;
                let h = veio_noise(q) + 0.5 * veio_noise(q * 2.1 - drift);
                let hx = veio_noise(q + vec2<f32>(e, 0.0)) + 0.5 * veio_noise((q + vec2<f32>(e, 0.0)) * 2.1 - drift);
                let hz = veio_noise(q + vec2<f32>(0.0, e)) + 0.5 * veio_noise((q + vec2<f32>(0.0, e)) * 2.1 - drift);
                ripple = ripple * 0.7 + vec2<f32>(h - hx, h - hz) * (2.6 + 1.5 * frame.rain);
            }
            let amplitude = 0.035 * (1.0 - smoothstep(16.0, 72.0, distance));
            water_normal = normalize(normal + vec3<f32>(ripple.x, 0.0, ripple.y) * amplitude);
        }
        var thickness = max(veio_water_thickness(input, view, input.screen), 0.0);
        let absorption = 1.0 - exp(-thickness * 0.16);
        let deep = mix(vanilla, vec3<f32>(0.025, 0.15, 0.18) * max(veio_luma(input.lightmap), 0.02), 0.55);
        if quality >= 1u {
            // Refraction: what lies behind the water, bent by the waves, loses red first with
            // depth and takes the water's own blue-green. Caustics dance on a shallow bottom.
            let bend = water_normal.xz * 0.04 * (1.0 - smoothstep(6.0, 48.0, distance)) * min(thickness, 2.0) * 0.5;
            let bent_thickness = veio_water_thickness(input, view, input.screen + bend);
            // Never pull in what stands in front of the water.
            let bent = bent_thickness > 0.02;
            let screen_here = select(input.screen, input.screen + bend, bent);
            thickness = select(thickness, bent_thickness, bent);
            let size = textureDimensions(opaque_color);
            let pixel = clamp(vec2<i32>(screen_here * vec2<f32>(size)), vec2<i32>(0), vec2<i32>(size) - vec2<i32>(1));
            var behind = textureLoad(opaque_color, pixel, 0).rgb;
            let floor_point = input.position - view * thickness + frame.camera_position - floor(frame.camera_position / 256.0) * 256.0;
            let c1 = veio_noise(floor_point.xz * 1.1 + vec2<f32>(frame.time * 0.5, frame.time * 0.3));
            let c2 = veio_noise(floor_point.xz * 1.3 - vec2<f32>(frame.time * 0.4, -frame.time * 0.35));
            let caustic = pow(1.0 - abs(c1 - c2), 10.0) * (1.0 - smoothstep(0.5, 5.0, thickness));
            behind *= 1.0 + caustic * 0.55 * sky_light * key_strength * visibility;
            let transmit = exp(-thickness * vec3<f32>(0.42, 0.11, 0.085));
            let body = mix(vec3<f32>(0.020, 0.13, 0.16), surface.rgb * 0.30, 0.45) * max(veio_luma(input.lightmap), 0.025);
            color = behind * transmit + body * (vec3<f32>(1.0) - transmit);
            alpha = 1.0;
        } else {
            color = mix(color, deep, absorption * 0.62);
            alpha = mix(surface.a * 0.52, max(surface.a, 0.84), absorption);
        }
        let fresnel = 0.02 + 0.68 * veio_pow5(1.0 - clamp(dot(water_normal, view), 0.0, 1.0));
        var sky = veio_environment(reflect(-view, water_normal));
        if reflections && quality >= 1u && frame.has_reflection == 1u && input.face == 1u {
            // The mirrored world, nudged by the ripples; the analytic sky stays where it shows.
            let offset = water_normal.xz * 0.04 * (1.0 - smoothstep(8.0, 64.0, distance));
            let mirrored = textureSampleLevel(reflection, reflection_sampler, input.screen + offset, 0.0);
            sky = mix(sky, mirrored.rgb, 0.85);
        }
        color = mix(color, sky, fresnel * sky_light);
        alpha = max(alpha, fresnel * sky_light * 0.85);
        let half_raw = view + light;
        let half_light = half_raw / max(length(half_raw), 0.001);
        let sharp = select(64.0, 420.0, quality >= 1u);
        let sparkle = pow(max(dot(water_normal, half_light), 0.0), sharp) * select(1.0, 4.0, quality >= 1u);
        color += veio_key_color() * sparkle * 0.18 * sky_light * key_strength * visibility;
    }
    // Horizon applies its original alpha rules and biome fog after this material.
    return Surface(vec4<f32>(clamp(color, vec3<f32>(0.0), vec3<f32>(1.0)), clamp(alpha, 0.0, 1.0)));
}
