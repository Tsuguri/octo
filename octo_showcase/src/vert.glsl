#version 450
layout (push_constant) uniform PushConsts {
  mat4 view;
  mat4 projection;
  mat4 model;

} push;
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 uv;

layout (location = 0) out gl_PerVertex {
  vec4 gl_Position;
};
layout (location = 1) out vec2 frag_uv;

mat4 mvp = push.projection * push.view * push.model;

void main()
{
  gl_Position = mvp * vec4(position, 1.0);
  frag_uv = (normal.xy/2.0 + 0.5);
}