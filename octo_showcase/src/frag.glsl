#version 450
/*layout (push_constant) uniform PushConsts {
    float time;
} push;
*/


layout (location = 1) in vec2 frag_uv;

layout(set = 0, binding = 0) uniform texture2D tex;
layout(set = 0, binding = 1) uniform sampler samp;

layout(location = 0) out vec4 color;
void main()
{
    //float time01 = -0.9 * abs(sin( 0.9)) + 0.9;
    color = texture(sampler2D(tex, samp), frag_uv);
    //color = mix(tex_color, vec4(1.0, 0.5, 1.0, 1.0), time01);
}