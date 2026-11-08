override edge_smoothing: bool = true;
override color_strength: f32 = 0.65;

fn post(input: PostInput) -> vec4<f32> {
    var color = input.color.rgb;
    if edge_smoothing {
        // A bounded five-tap spatial filter in this same pass. HUD and hand are drawn afterwards.
        let texel = 1.0 / frame.resolution;
        let left = textureSampleLevel(previous, linear_sampler, input.uv - vec2<f32>(texel.x, 0.0), 0.0).rgb;
        let right = textureSampleLevel(previous, linear_sampler, input.uv + vec2<f32>(texel.x, 0.0), 0.0).rgb;
        let up = textureSampleLevel(previous, linear_sampler, input.uv - vec2<f32>(0.0, texel.y), 0.0).rgb;
        let down = textureSampleLevel(previous, linear_sampler, input.uv + vec2<f32>(0.0, texel.y), 0.0).rgb;
        let center_luma = veio_luma(color);
        let lo = min(center_luma, min(min(veio_luma(left), veio_luma(right)), min(veio_luma(up), veio_luma(down))));
        let hi = max(center_luma, max(max(veio_luma(left), veio_luma(right)), max(veio_luma(up), veio_luma(down))));
        let blend = smoothstep(0.06, 0.22, hi - lo) * 0.24;
        color = mix(color, (left + right + up + down) * 0.25, blend);
    }
    // Gentle contrast with fixed exposure: dark caves stay dark, and black/white stay black/white.
    // Gamma-space API 0.1: do not apply an HDR tone map here.
    let bounded = clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));
    let contrast = bounded * bounded * (3.0 - 2.0 * bounded);
    let graded = mix(bounded, contrast, 0.12 * color_strength);
    let saturation = mix(vec3<f32>(veio_luma(graded)), graded, 1.0 + 0.025 * color_strength);
    return vec4<f32>(clamp(saturation, vec3<f32>(0.0), vec3<f32>(1.0)), input.color.a);
}
