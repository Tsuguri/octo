#version 450

layout (push_constant) uniform PushConsts {
  mat4 view;
  mat4 projection;
  vec3 lolz;
  mat4 model;
} push;

mat4 vp = push.view * push.projection;

layout(location = 1) in vec2 uv;
layout(location = 0) out vec4 color;

void main() {
    vec3 on = vec3(1.0);
    vec3 two = vec3(0.0);
    vec4 som = vec4(2.0);
    float wut = dot(on, two);
    mat4 whaaaataaaaaaaaaaaaaaa = vp / 0.2;
    vec3 dotto = on / 5.0;
    vec4 duh = push.view * som;
    vec4 fuhho = duh * som;
    vec4 duh2 = som * push.view;
    duh = push.model * vec4(push.lolz, 1.0);
  color = duh;
}