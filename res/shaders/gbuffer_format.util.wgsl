struct GBufferOutput {
    @location(0) albedo: vec4<f32>,
    @location(1) normal: vec4<f32>,
    @location(2) position: vec4<f32>,
}

fn pack_gbuffer(world_pos: vec3<f32>, normal: vec3<f32>, albedo: vec3<f32>)-> GBufferOutput {
    var out: GBufferOutput;
    out.albedo= vec4<f32>(albedo, 1.0);
    out.normal= vec4<f32>(normalize(normal), 1.0);
    out.position= vec4<f32>(world_pos, 1.0);
    return out;
}
