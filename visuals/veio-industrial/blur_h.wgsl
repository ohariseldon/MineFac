// Bloom, step 2 (quarter size): a wide horizontal Gaussian of the bright parts.
fn veio_blur(source: texture_2d<f32>, uv: vec2<f32>, axis: vec2<f32>) -> vec3<f32> {
    let texel = axis / vec2<f32>(textureDimensions(source));
    // Nine taps folded into five bilinear reads (weights of a sigma-2 Gaussian).
    var sum = textureSampleLevel(source, linear_sampler, uv, 0.0).rgb * 0.2270;
    sum += textureSampleLevel(source, linear_sampler, uv + texel * 1.3846, 0.0).rgb * 0.3162;
    sum += textureSampleLevel(source, linear_sampler, uv - texel * 1.3846, 0.0).rgb * 0.3162;
    sum += textureSampleLevel(source, linear_sampler, uv + texel * 3.2308, 0.0).rgb * 0.0703;
    sum += textureSampleLevel(source, linear_sampler, uv - texel * 3.2308, 0.0).rgb * 0.0703;
    return sum;
}

fn post(input: PostInput) -> vec4<f32> {
    if quality == 0u {
        return vec4<f32>(0.0);
    }
    return vec4<f32>(veio_blur(bright, input.uv, vec2<f32>(2.0, 0.0)), 1.0);
}
