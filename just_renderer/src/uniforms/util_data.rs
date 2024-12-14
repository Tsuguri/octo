use glam::{Mat4, Vec4};

use crate::bind_group_layouts::BindGroupLayouts;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UtilData {
    pub view: Mat4,
    pub projection: Mat4,
    pub view_projection: Mat4,
    pub eye_position: Vec4,
    pub total_time: f32,
    pub delta_time: f32,
    _padding: [f32; 2],
}

impl UtilData {
    pub fn new() -> Self {
        Self {
            total_time: 0f32,
            delta_time: 0f32,
            view: glam::Mat4::IDENTITY,
            projection: glam::Mat4::IDENTITY,
            view_projection: glam::Mat4::IDENTITY,
            eye_position: glam::Vec4::ZERO,
            _padding: Default::default(),
        }
    }
}

impl super::SingleUniformGroup for UtilData {
    const BINDING: u32 = 0;
    fn layout<'a>(&self, layouts: &'a BindGroupLayouts) -> &'a wgpu::BindGroupLayout {
        &layouts.single_uniform_group
    }
}

pub type UtilDataUniform = super::Uniform<UtilData>;
