use std::rc::Rc;

use glam::Vec4;

use crate::{
    bind_group_layouts::BindGroupLayouts,
    texture::TextureData,
    uniforms::{Material, MaterialUniform},
};

#[derive(Clone, Copy)]
pub struct MaterialId(usize);

impl MaterialId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

pub struct MaterialLibrary {
    materials: Vec<MaterialUniform>,
}

impl MaterialLibrary {
    pub fn new() -> Self {
        Self {
            materials: Vec::new(),
        }
    }
    pub fn add_material(
        &mut self,
        device: &wgpu::Device,
        layouts: &BindGroupLayouts,
        color: Vec4,
        texture: Rc<TextureData>,
    ) -> MaterialId {
        let id = MaterialId::new(self.materials.len());

        self.materials.push(MaterialUniform::new(
            Material { color },
            device,
            layouts,
            texture,
        ));

        id
    }
    pub fn get_material_bindgroup(&self, id: MaterialId) -> &wgpu::BindGroup {
        self.materials[id.0].get_bindgroup()
    }
}
