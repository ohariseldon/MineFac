
// Screen reflections (half size): wet streets after rain, polished metal and glass mirror the
// lamps, machines and sky already on screen. Only surfaces the material marked smooth or wet.
// rgb the reflected picture, a how much of it to show. 12 steps on Equilibrado, 24 on Alto.
fn post(input: PostInput) -> vec4<f32> {
    if quality == 0u || !screen_reflections || frame.eye_in != 0u {
        return vec4<f32>(0.0);
    }
    let surface = surface_at(vec2<i32>(input.uv * vec2<f32>(textureDimensions(scene_surface))));
    if !surface.present || surface.smoothness < 0.12 || surface.data > 0.5 {
        return vec4<f32>(0.0);
    }
    let depth = depth_at(input.uv);
    let position = view_position(input.uv, depth);
    let distance = length(position);
    if distance > 64.0 {
        return vec4<f32>(0.0);
    }
    let view = position / max(distance, 0.001);
    let n = surface.normal;
    let ray = reflect(view, n);
    let toward_camera = dot(ray, view);
    if toward_camera < -0.2 {
        return vec4<f32>(0.0);
    }
    let steps = select(12, 24, quality >= 2u);
    let jitter = veio_hash2(floor(input.pixel) + vec2<f32>(3.1, 7.7));
    var step_length = select(0.55, 0.32, quality >= 2u) * (1.0 + distance * 0.02);
    var p = position + n * 0.05 + ray * step_length * jitter;
    var hit = vec2<f32>(-1.0);
    var last = p;
    for (var i = 0; i < 24; i++) {
        if i >= steps { break; }
        last = p;
        p += ray * step_length;
        step_length *= 1.12;
        let screen = veio_project(p);
        if screen.z == 0.0 || any(screen.xy < vec2<f32>(0.0)) || any(screen.xy > vec2<f32>(1.0)) {
            break;
        }
        let d = depth_at(screen.xy);
        if d <= 0.000001 {
            continue;
        }
        let behind = length(p) - length(view_position(screen.xy, d));
        if behind > 0.0 && behind < step_length * 2.5 + 0.3 {
            // Three halvings between the last free point and this one.
            var a = last;
            var c = p;
            for (var j = 0; j < 3; j++) {
                let m = (a + c) * 0.5;
                let sm = veio_project(m);
                let dm = depth_at(sm.xy);
                if length(m) - length(view_position(sm.xy, dm)) > 0.0 { c = m; } else { a = m; }
            }
            hit = veio_project(c).xy;
            break;
        }
    }
    if hit.x < 0.0 {
        return vec4<f32>(0.0);
    }
    let reflected = textureSampleLevel(scene_color, linear_sampler, hit, 0.0).rgb;
    let edge = min(min(hit.x, 1.0 - hit.x), min(hit.y, 1.0 - hit.y));
    let fresnel = 0.04 + 0.96 * veio_pow5(1.0 - clamp(-dot(view, n), 0.0, 1.0));
    let weight = smoothstep(0.0, 0.08, edge) * (1.0 - smoothstep(40.0, 64.0, distance))
        * surface.smoothness * mix(0.35, 1.0, fresnel);
    return vec4<f32>(reflected, clamp(weight, 0.0, 1.0));
}
