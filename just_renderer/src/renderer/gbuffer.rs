use crate::texture::TextureData;

pub(crate) struct GBuffer {
    pub albedo: TextureData,
    pub normal: TextureData,
    pub position: TextureData,
    pub depth_texture: TextureData,
}

impl GBuffer {
    pub fn create(device: &wgpu::Device, size: wgpu::Extent3d) -> Self {
        let position = TextureData::create_render_target(
            &device,
            size,
            wgpu::TextureFormat::Rgba16Float,
            "gbuffer position",
        );
        let albedo = TextureData::create_render_target(
            &device,
            size,
            wgpu::TextureFormat::Rgba16Float,
            "gbuffer albedo",
        );
        let normal = TextureData::create_render_target(
            &device,
            size,
            wgpu::TextureFormat::Rgba16Float,
            "gbuffer normal",
        );

        let depth_texture = TextureData::create_depth_texture(&device, size, "main depth");
        Self {
            albedo,
            normal,
            position,
            depth_texture,
        }
    }
}
