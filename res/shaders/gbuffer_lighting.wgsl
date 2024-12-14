struct UtilDataUniform {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
    view_projection: mat4x4<f32>,
    eye_position: vec3<f32>,
    total_time: f32,
    delta_time: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

struct LightStrength {
    ambient: vec3<f32>,
    diffuse: vec3<f32>,
    specular: vec3<f32>,
};

struct DirectionalLight {
    direction: vec3<f32>, 
    colors: LightStrength,
};

struct PointLight {
    position: vec3<f32>,
    colors: LightStrength,
    attenuation: vec3<f32>,
};

struct DirectionalLightsUniform {
    count: u32,
    lights: array<DirectionalLight>,
};

struct PointLightsUniform {
    count: u32,
    lights: array<PointLight>,
}



@group(0) @binding(0)
var<uniform> util: UtilDataUniform;

@group(1) @binding(0)
var gbuffer_albedo: texture_2d<f32>;
@group(1) @binding(1)
var gbuffer_normal: texture_2d<f32>;
@group(1) @binding(2)
var gbuffer_position: texture_2d<f32>;
@group(1) @binding(3)
var albedo_sampler: sampler;
@group(1) @binding(4)
var normal_sampler: sampler;
@group(1) @binding(5)
var position_sampler: sampler;

@group(2) @binding(0)
var<storage> directional_lights: DirectionalLightsUniform;
@group(2) @binding(1)
var<storage> point_lights: PointLightsUniform;

fn number_of_lights() -> u32 {
  //return arrayLength(&directional_lights.lights) + arrayLength(&point_lights.lights);
  return directional_lights.count + point_lights.count;
}

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32
) -> VertexOutput {
    var out: VertexOutput;
    let x: f32 = -1.0 + f32(i32(in_vertex_index)%2) * 4.0;
    let y: f32 = f32(i32(in_vertex_index)/2) * 4.0 - 1.0;

    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.tex_coords = 0.5 * vec2<f32>(x, -y)  + vec2<f32>(0.5, 0.5);
    return out;
}

fn calculate_point_lights(position: vec3<f32>, normal: vec3<f32>, albedo: vec3<f32>) -> vec3<f32> {
    var output = vec3<f32>(0.0, 0.0, 0.0);

    for (var i: u32 = 0u; i<point_lights.count; i++) {
        let light = point_lights.lights[i];
        let distance = length(light.position - position);
        let to_light = normalize(light.position - position);

        let attenuation = 1.0 / (light.attenuation.x + distance * light.attenuation.y + distance * distance * light.attenuation.z);

        // light settings
        let specular_pow = 32.0;

        //diffuse
        let diffuse_strength = max(dot(normal, to_light), 0.0);

        //specular
        let to_camera = normalize(util.eye_position - position);
        let half_dir = normalize(to_light + to_camera);
        let specular_strength = pow(max(dot(normal, half_dir), 0.0), specular_pow);

        output += (light.colors.ambient * albedo + light.colors.diffuse * diffuse_strength * albedo + light.colors.specular * specular_strength) * attenuation;
    }

    return output;
}


// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let lights = number_of_lights();
    // pixel data
    let albedo = textureSample(gbuffer_albedo, albedo_sampler, in.tex_coords).xyz;
    let normal = textureSample(gbuffer_normal, normal_sampler, in.tex_coords).xyz;
    let position = textureSample(gbuffer_position, position_sampler, in.tex_coords).xyz;

    let point_lights_strength = calculate_point_lights(position, normal, albedo);

    let pixel_color = point_lights_strength;

    return vec4<f32>(pixel_color, 1.0);
}
