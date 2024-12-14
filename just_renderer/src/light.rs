use encase::{ArrayLength, ShaderType};
use glam::Vec3;

#[derive(bytemuck::Pod, bytemuck::Zeroable, Debug, Clone, Copy, ShaderType)]
#[repr(C)]
pub struct LightStrength {
    pub ambient: Vec3,
    pub diffuse: Vec3,
    pub specular: Vec3,
}

impl LightStrength {
    pub fn splat_floats(
        r: f32,
        g: f32,
        b: f32,
        diffuse_multiplier: f32,
        ambient_multiplier: f32,
    ) -> Self {
        let color = Vec3::new(r, g, b);
        return Self::splat_vec(color, diffuse_multiplier, ambient_multiplier);
    }
    pub fn splat_vec(color: Vec3, diffuse_multiplier: f32, ambient_multiplier: f32) -> Self {
        Self {
            ambient: color * ambient_multiplier,
            diffuse: color * diffuse_multiplier,
            specular: color,
        }
    }
}

#[derive(Debug, Clone, ShaderType, Copy)]
pub struct PointLightParameters {
    pub pos: Vec3,
    pub colors: LightStrength,
    pub attenuation: Vec3, // contant, linear, quadratic
}

impl PointLightParameters {
    pub fn attenuation_for_range(range: f32) -> Vec3 {
        Vec3::new(1.0, 4.5 / range, 75.0 / (range * range))
    }
}

#[derive(Debug, Clone, ShaderType, Copy)]
pub struct DirectionalLightParameters {
    pub direction: Vec3,
    pub colors: LightStrength,
}
pub enum Light {
    Point(PointLightParameters),
    Directional(DirectionalLightParameters),
}

#[derive(Debug, Clone, ShaderType, Default)]
pub(crate) struct DirectionalLightBuffer {
    count: ArrayLength,
    #[size(runtime)]
    pub array: Vec<DirectionalLightParameters>,
}

#[derive(Debug, Clone, ShaderType, Default)]
pub(crate) struct PointLightBuffer {
    count: ArrayLength,
    #[size(runtime)]
    pub array: Vec<PointLightParameters>,
}
