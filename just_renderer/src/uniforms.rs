mod material;
mod model;
mod util_data;

pub use material::{Material, MaterialUniform};
pub use model::{Model, ModelUniform};
pub use util_data::{UtilData, UtilDataUniform};

use wgpu::util::DeviceExt as _;

use crate::bind_group_layouts::BindGroupLayouts;

pub trait SingleUniformGroup: bytemuck::Pod {
    const BINDING: u32;
    fn layout<'a>(&self, layouts: &'a BindGroupLayouts) -> &'a wgpu::BindGroupLayout;
}

pub struct Uniform<T: SingleUniformGroup> {
    data: T,
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    modified: bool,
}

impl<T: SingleUniformGroup> Uniform<T> {
    pub fn new(data: T, device: &wgpu::Device, layouts: &BindGroupLayouts) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Uniform for {}", std::any::type_name::<T>())),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            contents: bytemuck::cast_slice(&[data]),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("BindGroup for {}", std::any::type_name::<T>())),
            layout: data.layout(layouts),
            entries: &[wgpu::BindGroupEntry {
                binding: T::BINDING,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            data,
            buffer,
            bind_group,
            modified: false,
        }
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn modify<F: FnOnce(&mut T)>(&mut self, modifier: F) {
        modifier(&mut self.data);
        self.modified = true;
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        if self.modified {
            queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.data]));
            self.modified = false;
        }
    }
}
