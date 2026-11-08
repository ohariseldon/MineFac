// Bloom, step 2 (quarter size): the vertical half of the Gaussian.
fn post(input: PostInput) -> vec4<f32> {
    if quality == 0u || bloom <= 0.0 {
        return vec4<f32>(0.0);
    }
    let texel = vec2<f32>(0.0, 2.0) / vec2<f32>(textureDimensions(glow_h));
    var sum = textureSampleLevel(glow_h, linear_sampler, input.uv, 0.0).rgb * 0.2270;
    sum += textureSampleLevel(glow_h, linear_sampler, input.uv + texel * 1.3846, 0.0).rgb * 0.3162;
    sum += textureSampleLevel(glow_h, linear_sampler, input.uv - texel * 1.3846, 0.0).rgb * 0.3162;
    sum += textureSampleLevel(glow_h, linear_sampler, input.uv + texel * 3.2308, 0.0).rgb * 0.0703;
    sum += textureSampleLevel(glow_h, linear_sampler, input.uv - texel * 3.2308, 0.0).rgb * 0.0703;
    return vec4<f32>(sum, 1.0);
}
