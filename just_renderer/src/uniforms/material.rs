use std::rc::Rc;

use crate::{bind_group_layouts::BindGroupLayouts, texture::TextureData};
use glam::Vec4;
use wgpu::util::DeviceExt as _;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Material {
    pub color: Vec4,
}

pub struct MaterialUniform {
    _data: Material,
    _buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    _texture: Rc<TextureData>,
    _modified: bool,
}

impl MaterialUniform {
    pub fn new(
        data: Material,
        device: &wgpu::Device,
        layouts: &BindGroupLayouts,
        texture: Rc<TextureData>,
    ) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Uniform for Material Uniform")),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            contents: bytemuck::cast_slice(&[data]),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("BindGroup for Material Uniform")),
            layout: &layouts.material,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        Self {
            _data: data,
            _buffer: buffer,
            bind_group,
            _modified: false,
            _texture: texture,
        }
    }

    pub fn get_bindgroup(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
