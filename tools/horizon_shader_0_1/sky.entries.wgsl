// Generated from Horizon (MIT) crates/horizon-shader-api, shader API 0.1. Used only by tools/check_shaders.py.
struct HorizonSkyIn {
    @builtin(position) position: vec4<f32>,
    @location(0) direction: vec3<f32>,
    @location(1) color: vec4<f32>,
}
@fragment fn horizon_fs_sky(in: HorizonSkyIn) -> @location(0) vec4<f32> {
    return sky(SkyInput(normalize(in.direction), in.color));
}
