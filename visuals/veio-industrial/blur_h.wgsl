// Bloom, step 1 (quarter size): the bright parts of the picture, read straight from the scene,
// spread by a wide horizontal Gaussian. Lamps, lava, the sun and lit windows pass. Each tap
// averages the 4 by 4 pixels under it, so small lights do not flicker as the camera moves.
fn veio_bright(uv: vec2<f32>, texel: vec2<f32>) -> vec3<f32> {
    let a = textureSampleLevel(scene_color, linear_sampler, uv + texel * vec2<f32>(-1.0, -1.0), 0.0).rgb;
    let b = textureSampleLevel(scene_color, linear_sampler, uv + texel * vec2<f32>(1.0, -1.0), 0.0).rgb;
    let c = textureSampleLevel(scene_color, linear_sampler, uv + texel * vec2<f32>(-1.0, 1.0), 0.0).rgb;
    let d = textureSampleLevel(scene_color, linear_sampler, uv + texel * vec2<f32>(1.0, 1.0), 0.0).rgb;
    let color = (a + b + c + d) * 0.25;
    // A soft knee: nothing below 0.62, all of it above 0.92.
    return color * smoothstep(0.62, 0.92, veio_luma(color));
}

fn post(input: PostInput) -> vec4<f32> {
    if quality == 0u || bloom <= 0.0 {
        return vec4<f32>(0.0);
    }
    let full = 1.0 / frame.resolution;
    // Nine taps 2 quarter texels apart (8 pixels), folded into five pairs of bilinear reads.
    let step = vec2<f32>(8.0 * full.x, 0.0);
    var sum = veio_bright(input.uv, full) * 0.2270;
    sum += veio_bright(input.uv + step * 1.3846, full) * 0.3162;
    sum += veio_bright(input.uv - step * 1.3846, full) * 0.3162;
    sum += veio_bright(input.uv + step * 3.2308, full) * 0.0703;
    sum += veio_bright(input.uv - step * 3.2308, full) * 0.0703;
    // Dark scenes glow more around their lights (night, caves).
    let night = 1.0 - 0.5 * veio_daylight() * frame.eye_light_smooth.y;
    return vec4<f32>(sum * night, 1.0);
}
