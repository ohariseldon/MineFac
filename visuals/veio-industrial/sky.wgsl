fn sky(input: SkyInput) -> vec4<f32> {
    // The biome, weather and night keep their original colours. Only daytime air changes.
    let day = veio_daylight() * (1.0 - 0.85 * frame.rain);
    let elevation = clamp(input.direction.y, 0.0, 1.0);
    let sun = max(dot(input.direction, frame.sun_direction), 0.0);
    let low_sun = 1.0 - smoothstep(0.08, 0.55, frame.sun_direction.y);
    let horizon = 1.0 - smoothstep(0.0, 0.45, elevation);
    let clear_air = mix(vec3<f32>(0.68, 0.78, 0.82), vec3<f32>(0.19, 0.43, 0.68), sqrt(elevation));
    let warm_air = vec3<f32>(0.14, 0.065, 0.012) * sun * sun * horizon * low_sun;
    let color = mix(input.color.rgb, clear_air, 0.28 * day) + warm_air * day;
    return vec4<f32>(color, input.color.a);
}
