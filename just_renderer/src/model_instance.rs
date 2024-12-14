use strum::EnumCount;

use crate::{material_library::MaterialId, uniforms::ModelUniform};

// Used as an index in built in models array
#[derive(Debug, EnumCount, Clone, Copy)]
#[repr(u8)]
pub enum BuiltInModel {
    Quad = 0,
}

#[derive(Debug, Clone, Copy)]
pub enum ModelId {
    BuiltIn(BuiltInModel),
    Game(u32),
}

pub struct ModelInstance {
    pub positon: glam::Vec3,
    pub rotation: glam::Quat,
    pub scale: glam::Vec3,

    pub model_id: ModelId,
    pub model_uniform: ModelUniform,
    pub material: MaterialId,
}
