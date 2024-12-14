use crate::{
    bind_group_layouts::BindGroupLayouts,
    passes::{PostprocessingPass, SolidColorRenderer},
    shader_manager::ShaderManager,
    texture::TextureData,
};

use super::{configuration::Configuration, gbuffer::GBuffer};

pub(crate) struct PipelineState {
    pub gbuffer: GBuffer,
    pub solid_color_pass: SolidColorRenderer,
    pub postprocessing: PostprocessingPass,
}

impl PipelineState {
    pub fn create(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        bind_group_layouts: &BindGroupLayouts,
        shaders: &ShaderManager,
        _configuration: &Configuration,
    ) -> Result<Self, String> {
        let gbuffer_size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };

        let gbuffer = GBuffer::create(device, gbuffer_size);

        let postprocessing = PostprocessingPass::initialize(
            &device,
            config.format,
            &bind_group_layouts,
            shaders,
            &gbuffer,
        )?;
        let solid_color_pass =
            SolidColorRenderer::initialize(&device, &gbuffer, &bind_group_layouts, shaders)?;

        Result::Ok(Self {
            gbuffer,
            solid_color_pass,
            postprocessing,
        })
    }
}
