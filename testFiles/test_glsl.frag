#version 450

layout(location = 1) in vec2 uv;
layout(location = 0) out vec4 color;

void main() {
    vec3 on = vec3(1.0);
    vec3 two = vec3(0.0);
    float wut = dot(on, two);

}