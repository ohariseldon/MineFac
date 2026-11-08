override waving: bool = true;

// Plants sway in the wind and water rolls. Displacements depend only on the world position
// and time, so corners shared by two blocks move together and nothing tears apart.
fn veio_sway(world: vec3<f32>, strength: f32) -> vec3<f32> {
    let t = frame.time;
    let wind = veio_wind();
    let phase = world.x * 0.37 + world.z * 0.23 + world.y * 0.11;
    let gust = 0.6 + 0.4 * sin(t * 0.35 + world.x * 0.021 + world.z * 0.017);
    let primary = sin(t * 1.9 + phase) * gust;
    let flutter = sin(t * 4.7 + phase * 2.3) * 0.25;
    let along = vec3<f32>(0.80, 0.0, 0.45) * (primary + flutter);
    let across = vec3<f32>(-0.45, 0.0, 0.80) * sin(t * 1.3 + phase * 1.7) * 0.35;
    return (along + across) * strength * wind;
}

fn vertex(input: VertexInput) -> vec3<f32> {
    if !waving || quality == 0u || frame.dimension != 0 {
        return input.position;
    }
    let distance = length(input.position);
    if distance > 48.0 {
        return input.position;
    }
    let near = 1.0 - smoothstep(32.0, 48.0, distance);
    // World coordinates wrapped to keep the phase precise far from the origin.
    let camera = frame.camera_position - floor(frame.camera_position / 1024.0) * 1024.0;
    let world = input.position + camera;
    // Sheltered plants (no sky above) barely move.
    let open = 0.25 + 0.75 * input.light.y;
    var position = input.position;
    if input.fluid == 1u && input.face == 1u {
        // Rolling water: two long swells and a short chop, a few centimetres.
        let t = frame.time;
        let swell = sin(world.x * 0.55 + t * 1.3) * cos(world.z * 0.47 - t * 1.1);
        let chop = sin((world.x + world.z) * 1.7 + t * 2.6) * 0.3;
        position.y += (swell + chop) * 0.035 * (0.6 + 0.6 * veio_wind()) * near * open;
        return position;
    }
    var weight = 0.0;
    switch input.material {
        // Grass, flowers and crops: the top of the sprite moves, the roots stay.
        case 5u: { weight = (1.0 - input.uv.y) * 0.11; }
        // Leaves: the whole block breathes a little.
        case 7u: { weight = 0.035; }
        // Double plants: the bottom half bends to the middle, the top half continues it.
        case 8u: { weight = (1.0 - input.uv.y) * 0.06; }
        case 9u: { weight = 0.06 + (1.0 - input.uv.y) * 0.08; }
        default: { return position; }
    }
    position += veio_sway(world, weight * near * open);
    return position;
}
