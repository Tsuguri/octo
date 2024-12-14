use glam::Mat4;

use crate::bind_group_layouts::BindGroupLayouts;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Model {
    pub model_matrix: Mat4,
    pub normal_matrix: Mat4,
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}

impl Model {
    pub fn new() -> Self {
        Self {
            model_matrix: glam::Mat4::IDENTITY,
            normal_matrix: glam::Mat4::IDENTITY,
        }
    }
}

impl super::SingleUniformGroup for Model {
    const BINDING: u32 = 0;
    fn layout<'a>(&self, layouts: &'a BindGroupLayouts) -> &'a wgpu::BindGroupLayout {
        &layouts.model
    }
}

pub type ModelUniform = super::Uniform<Model>;
