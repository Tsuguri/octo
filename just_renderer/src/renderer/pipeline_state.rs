use crate::{
    bind_group_layouts::BindGroupLayouts,
    passes::{PostprocessingPass, SolidColorRenderer},
    shader_manager::ShaderManager,
    texture::TextureData,
};

use super::{blit::BlitPass, configuration::Configuration, gbuffer::GBuffer};

pub(crate) struct PipelineState {
    pub gbuffer: GBuffer,
    pub scene_color: TextureData,
    pub blit_pass: BlitPass,
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
        let scene_color = TextureData::create_render_target(
            device,
            gbuffer_size,
            wgpu::TextureFormat::Rgba16Float,
            "scene color",
        );
        let blit_pass = BlitPass::initialize(device, config.format);

        let postprocessing = PostprocessingPass::initialize(
            &device,
            wgpu::TextureFormat::Rgba16Float,
            &bind_group_layouts,
            shaders,
            &gbuffer,
        )?;
        let solid_color_pass =
            SolidColorRenderer::initialize(&device, &gbuffer, &bind_group_layouts, shaders)?;

        Result::Ok(Self {
            gbuffer,
            scene_color,
            blit_pass,
            solid_color_pass,
            postprocessing,
        })
    }

    pub fn clear_scene_color(&self, encoder: &mut wgpu::CommandEncoder) {
        let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("scene color clear pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.scene_color.view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    }
}
