override edge_smoothing: bool = true;
override color_strength: f32 = 0.65;

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
    }
    if atmosphere > 0.0 && frame.dimension == 0 && frame.eye_in == 0u && input.depth > 0.000001 {
        // Distance haze for the whole scene, including native animated machinery.
        // Depth already exists: no volume, ray march or additional pass.
        let clip = vec4<f32>(input.uv.x * 2.0 - 1.0, 1.0 - input.uv.y * 2.0, input.depth, 1.0);
        let projected = frame.projection_inverse * clip;
        let distance = length(projected.xyz / max(abs(projected.w), 0.00001));
        let near = min(24.0, frame.render_distance * 0.25);
        let far = max(near + 16.0, frame.render_distance * 0.88);
        let exposed = frame.eye_light_smooth.y * frame.eye_light_smooth.y;
        let effects = (1.0 - frame.blindness) * (1.0 - frame.darkness) * (1.0 - 0.6 * frame.night_vision);
        let haze = smoothstep(near, far, distance) * atmosphere * exposed * effects
            * (0.11 + 0.09 * frame.rain) * mix(0.30, 1.0, veio_daylight());
        let air = mix(frame.fog_color, veio_environment(vec3<f32>(0.0, 0.0, 1.0)), 0.22);
        color = mix(color, air, clamp(haze, 0.0, 0.24));
    }
    // Fixed exposure and luminance-based contrast keep hues and dark interiors stable.
    // Gamma-space API: an HDR tone map here would double-correct the image.
    let bounded = clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));
    let luma = veio_luma(bounded);
    let curve = luma * luma * (3.0 - 2.0 * luma);
    let target_luma = mix(luma, curve, 0.10 * color_strength);
    let graded = bounded * select(1.0, target_luma / max(luma, 0.00001), luma > 0.00001);
    let saturation = mix(vec3<f32>(veio_luma(graded)), graded, 1.0 + 0.025 * color_strength);
    return vec4<f32>(clamp(saturation, vec3<f32>(0.0), vec3<f32>(1.0)), input.color.a);
}
