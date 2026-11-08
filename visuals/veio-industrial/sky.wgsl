fn sky(input: SkyInput) -> vec4<f32> {
    // Keep the biome and the native sun, clouds, stars and moon. No atmospheric ray march.
    let day = veio_daylight() * (1.0 - 0.85 * frame.rain);
    let elevation = clamp(input.direction.y, 0.0, 1.0);
    let sun = max(dot(input.direction, frame.sun_direction), 0.0);
    let horizon = 1.0 - smoothstep(0.0, 0.45, elevation);
    let sun_band = sun * sun;
    let warm_air = vec3<f32>(0.18, 0.075, 0.014) * sun_band * horizon * veio_low_sun();
    let clear_air = veio_environment(input.direction);
    let color = mix(input.color.rgb, clear_air, atmosphere * (0.04 + 0.62 * day))
        + warm_air * day * atmosphere;
    return vec4<f32>(color, input.color.a);
}
