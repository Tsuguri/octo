//#include gbuffer_format

struct UtilDataUniform {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
    view_projection: mat4x4<f32>,
    eye_position: vec3<f32>,
    total_time: f32,
    delta_time: f32,
};

struct ModelUniform {
    model_matrix: mat4x4<f32>,
    normal_matrix: mat4x4<f32>,
};

struct SolidColorUniform {
    color: vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) position: vec3<f32>,
};


@group(0) @binding(0)
var<uniform> util: UtilDataUniform;
@group(1) @binding(0)
var<uniform> model: ModelUniform;

@group(2) @binding(0)
var<uniform> color_data: SolidColorUniform;
@group(2) @binding(1)
var diffuse: texture_2d<f32>;
@group(2) @binding(2)
var diffuse_sampler: sampler;

@vertex
fn vs_main(
    mesh: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    out.uv = mesh.uv;
    out.normal = (model.normal_matrix * vec4<f32>(mesh.normal, 0.0)).xyz;
    // out.normal = mesh.normal;
    var pos = model.model_matrix * vec4<f32>(mesh.position, 1.0);
    out.clip_position = util.view_projection * pos;
    out.position = pos.xyz;
    return out;
}


// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> GBufferOutput {
    return pack_gbuffer(in.position, in.normal, textureSample(diffuse, diffuse_sampler, in.uv).xyz);
}
