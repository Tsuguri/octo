use crate::bind_group_layouts::BindGroupLayouts;

use self::noise::NoiseGenerator;

pub mod noise;

pub struct Generators {
    pub perlin: NoiseGenerator,
}

impl Generators {
    pub fn initialize(device: &wgpu::Device, _bind_group_layouts: &BindGroupLayouts) -> Self {
        Self {
            perlin: NoiseGenerator::initialize(device),
        }
    }
}
