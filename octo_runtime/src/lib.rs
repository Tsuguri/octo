use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum TextureType {
    Float,
    Vec2,
    Vec3,
    Vec4,
}

#[derive(Serialize, Deserialize)]
pub struct ShaderPass {
    pub input: Vec<(String, TextureType)>,
    pub output: Vec<(String, TextureType)>,
    pub code: String,
    pub spirv: Vec<u8>

}


#[derive(Serialize, Deserialize)]
pub struct OctoModule {
    pub name: String,
    pub version: u32,
    pub basic_vertex: String,
    pub basic_vertex_spirv: Vec<u8>,
    pub fragment_shaders: HashMap<String, (String, Vec<u8>)>,
    pub passes: HashMap<String, ShaderPass>,
}

impl OctoModule {
    pub fn new() -> Self {
        OctoModule{
            name: "test_module".to_owned(),
            version: 0u32,



            basic_vertex: "#version 450
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
  frag_uv = uv;
}".to_owned(),
            fragment_shaders: HashMap::new(),
            basic_vertex_spirv: vec![],
            passes: HashMap::new(),
        }
    }
}
