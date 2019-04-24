use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct OctoModule {
    pub name: String,
    pub version: u32,
    pub basic_vertex: String,
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
        }
    }
}

pub fn lolz() {
    println!("lolz");
}
