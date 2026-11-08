// Rain and snow take the colour of the light around them: grey-blue under a stormy sky,
// warm near torches and furnaces, faint at night. Horizon applies no light here.
fn weather(input: WeatherInput) -> vec4<f32> {
    let day = veio_daylight();
    let sky = mix(vec3<f32>(0.10, 0.12, 0.17), mix(frame.fog_color, vec3<f32>(0.62, 0.68, 0.76), 0.5), day);
    let sky_part = sky * input.light.y;
    let block_part = vec3<f32>(1.0, 0.72, 0.42) * input.light.x * input.light.x * 0.9;
    let light = max(sky_part + block_part, vec3<f32>(0.035));
    // The Veio winter below freezing: the game still draws rain where its biome is mild, so
    // the streaks turn white and dense, sleet and snow, as the snow the mod lays on the ground.
    let freezing = select(0.0, 1.0 - smoothstep(-2.0, 1.0, frame.mods[0].y * 50.0), veio_has_mod() && !input.snow);
    let base = mix(input.color.rgb, vec3<f32>(0.95, 0.97, 1.0), freezing);
    var color = base * light * mix(select(1.15, 1.35, input.snow), 1.35, freezing);
    // Rain is mostly transparent, a streak of refraction; snow is opaque flakes.
    let alpha = input.color.a * mix(select(0.55, 0.95, input.snow), 0.9, freezing);
    color = mix(color, vec3<f32>(0.5, 0.47, 0.40) * light, veio_smog() * 0.3);
    return vec4<f32>(clamp(color, vec3<f32>(0.0), vec3<f32>(1.0)), alpha);
}
