#version 450

layout(set = 0, binding = 0) uniform sampler colorsampler;
layout(set = 0, binding = 1) uniform texture2D[2] colormaps;

layout(location = 1) in vec2 uv;

layout(location = 0) out vec4 color;

void main() {
    vec4 kolorek = texture(sampler2D(colormaps[0], colorsampler), uv);
    color = kolorek;
}