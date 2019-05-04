use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct OctoModule {
    pub name: String,
    pub version: u32,
    pub basic_vertex: String,
    pub basic_fragment: String,
}

impl OctoModule {
    pub fn new() -> Self {
        OctoModule{
            name: "test_module".to_owned(),
            version: 0u32,



            basic_vertex: "#version 450
layout (location = 0) in vec2 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec2 uv;

layout (location = 0) out gl_PerVertex {
  vec4 gl_Position;
};
layout (location = 1) out vec3 frag_color;
layout (location = 2) out vec2 frag_uv;

void main()
{
  gl_Position = vec4(position, 0.0, 1.0);
  frag_color = color;
  frag_uv = uv;
}".to_owned(),
            basic_fragment: "#version 450
layout (push_constant) uniform PushConsts {
    float time;
} push;


layout (location = 1) in vec3 frag_color;
layout (location = 2) in vec2 frag_uv;

layout(set = 0, binding = 0) uniform texture2D tex;
layout(set = 0, binding = 1) uniform sampler samp;

layout(location = 0) out vec4 color;
void main()
{
    float time01 = -0.9 * abs(sin(push.time * 0.9)) + 0.9;
    vec4 tex_color = texture(sampler2D(tex, samp), frag_uv);
  //color = vec4(frag_color, 1.0) * vec4(vec3(time01), 1.0);
    color = mix(tex_color, vec4(frag_color, 1.0), time01);
}".to_owned(),
        }
    }
}

pub fn lolz() {
    println!("lolz");
}
