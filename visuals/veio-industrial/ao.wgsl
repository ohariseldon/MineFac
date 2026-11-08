
// Ambient occlusion (half size): corners, the ground under machines, the gaps between planks and
// the foot of walls darken where light from the sky can hardly reach. The terrain's normals come
// from the material stage (relief included). r: how open the point is, g: its distance, for the
// depth-aware upscale in the last pass. 8 samples on Equilibrado, 12 on Alto; none on Leve.
fn post(input: PostInput) -> vec4<f32> {
    if quality == 0u || ambient_occlusion <= 0.0 {
        return vec4<f32>(1.0, 0.0, 0.0, 1.0);
    }
    let depth = depth_at(input.uv);
    if depth <= 0.000001 {
        return vec4<f32>(1.0, 4096.0, 0.0, 1.0);
    }
    let position = view_position(input.uv, depth);
    let distance = length(position);
    let surface = surface_at(vec2<i32>(input.uv * vec2<f32>(textureDimensions(scene_surface))));
    if !surface.present || distance > 72.0 {
        return vec4<f32>(1.0, distance, 0.0, 1.0);
    }
    let n = surface.normal;
    let helper = select(vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(1.0, 0.0, 0.0), abs(n.y) > 0.9);
    let t = normalize(cross(helper, n));
    let b = cross(n, t);
    // A fixed rotation per pixel (no flicker); the upscale's four taps smooth the pattern.
    let spin = veio_hash2(floor(input.pixel)) * 6.2831853;
    let samples = select(8, 12, quality >= 2u);
    let radius = 0.85;
    var hidden = 0.0;
    for (var i = 0; i < 12; i++) {
        if i >= samples { break; }
        let k = (f32(i) + 0.5) / f32(samples);
        let angle = spin + f32(i) * 2.3999632;
        let spread = sqrt(k);
        // Cosine-weighted hemisphere, nearer samples weigh more.
        let local = vec3<f32>(cos(angle) * spread, sin(angle) * spread, sqrt(1.0 - k));
        let reach = radius * mix(0.25, 1.0, k * k);
        let p = position + (t * local.x + b * local.y + n * local.z) * reach + n * 0.03;
        let screen = veio_project(p);
        if screen.z == 0.0 || any(screen.xy < vec2<f32>(0.0)) || any(screen.xy > vec2<f32>(1.0)) {
            continue;
        }
        let scene_depth_here = depth_at(screen.xy);
        if scene_depth_here <= 0.000001 {
            continue;
        }
        let scene = view_position(screen.xy, scene_depth_here);
        let in_front = length(p) - length(scene);
        // Only nearby geometry hides the sky: a wall far behind does not darken a ledge.
        let near = 1.0 - smoothstep(radius * 0.8, radius * 2.0, length(scene - position));
        hidden += select(0.0, near, in_front > 0.02);
    }
    var open = 1.0 - hidden / f32(samples);
    // Leaves and grass shade themselves already; keep them light.
    open = mix(open, 1.0, surface.data * 0.6);
    let fade = smoothstep(48.0, 72.0, distance);
    return vec4<f32>(mix(open, 1.0, fade), distance, 0.0, 1.0);
}
