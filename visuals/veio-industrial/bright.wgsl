// Bloom, step 1 (half size): the bright parts of the picture, averaged over four texels so the
// smaller targets after it do not flicker. Lamps, lava, the sun and lit windows pass.
fn post(input: PostInput) -> vec4<f32> {
    if quality == 0u || bloom <= 0.0 {
        return vec4<f32>(0.0);
    }
    let texel = 1.0 / frame.resolution;
    let a = textureSampleLevel(scene_color, linear_sampler, input.uv + texel * vec2<f32>(-0.75, -0.75), 0.0).rgb;
    let b = textureSampleLevel(scene_color, linear_sampler, input.uv + texel * vec2<f32>(0.75, -0.75), 0.0).rgb;
    let c = textureSampleLevel(scene_color, linear_sampler, input.uv + texel * vec2<f32>(-0.75, 0.75), 0.0).rgb;
    let d = textureSampleLevel(scene_color, linear_sampler, input.uv + texel * vec2<f32>(0.75, 0.75), 0.0).rgb;
    let color = (a + b + c + d) * 0.25;
    // A soft knee: nothing below 0.62, all of it above 0.92.
    let luma = veio_luma(color);
    let knee = smoothstep(0.62, 0.92, luma);
    // Dark scenes glow more around their lights (night, caves).
    let night = 1.0 - 0.5 * veio_daylight() * frame.eye_light_smooth.y;
    return vec4<f32>(color * knee * night, 1.0);
}
