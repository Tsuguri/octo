use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OctoModule {
    pub name: String,
    pub version: u32,
    pub basic_vertex: String,
    pub basic_vertex_spirv: Vec<u8>,
    pub fragment_shaders: HashMap<String, (String, Vec<u8>)>,
}

impl OctoModule {
    pub fn new() -> Self {
        OctoModule{
            name: "test_module".to_owned(),
            version: 0u32,



            basic_vertex: "#version 450
layout (location = 0) in vec2 position;
layout (location = 2) in vec2 uv;

layout (location = 0) out gl_PerVertex {
  vec4 gl_Position;
};
layout (location = 2) out vec2 frag_uv;

void main()
{
  gl_Position = vec4(position, 0.0, 1.0);
  frag_uv = uv;
}".to_owned(),
            fragment_shaders: HashMap::new(),
            basic_vertex_spirv: vec![],
        }
    }
}

pub fn lolz() {
    println!("lolz");
}
