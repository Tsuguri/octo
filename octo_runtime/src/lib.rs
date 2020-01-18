use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type TextureId = usize;
pub type PassId = usize;
pub type ShaderId = usize;

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum TextureType {
    Float,
    Vec2,
    Vec3,
    Vec4,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum ValueType {
    Float,
    Vec2,
    Vec3,
    Vec4,
    Mat3,
    Mat4,
    Int,
    Bool,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum OutputType {
    Result,
    Textures(Vec<TextureId>),
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum InputType {
    ProvidedTexture(TextureId),
    PipelineTexture(TextureId),
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum TextureSize {
    Original,
    Scaled(f32),
    Custom(u32, u32),
}

#[derive(Serialize, Deserialize)]
pub struct ShaderPass {
    pub id: PassId,
    pub input: Vec<InputType>,
    pub output: OutputType,
    pub shader: ShaderId,
    pub dependencies: Option<Vec<PassId>>,
}

#[derive(Serialize, Deserialize)]
pub struct OctoModule {
    pub name: String,
    pub version: u32,
    pub basic_vertex_spirv: Vec<u32>,
    pub fragment_shaders: HashMap<ShaderId, Vec<u32>>,
    pub passes: Vec<ShaderPass>,

    pub uniform_block: Vec<(String, ValueType)>,
    pub uniform_block_size: usize,
    pub required_input: Vec<(String, ValueType)>,
    pub textures: Vec<(TextureId, TextureType, TextureSize)>,
}

impl OctoModule {
    pub fn new() -> Self {
        OctoModule {
            name: "test_module".to_owned(),
            version: 0u32,
            fragment_shaders: HashMap::new(),
            basic_vertex_spirv: vec![],
            passes: vec![],
            uniform_block: vec![],
            uniform_block_size: 0,
            required_input: vec![],
            textures: vec![],
        }
    }
}
