use std::borrow::Cow;

use crate::{
    bind_group_layouts::BindGroupLayouts,
    material_library::{MaterialId, MaterialLibrary},
    model_data::{ModelData, ModelVertex},
    model_instance::ModelInstance,
    renderer::gbuffer::GBuffer,
    shader_manager::ShaderManager,
    uniforms::UtilDataUniform,
};

pub(crate) struct SolidColorPass<'c, 'm: 'c>(wgpu::RenderPass<'c>, &'m MaterialLibrary);
pub(crate) struct SolidColorRenderer {
    render_pipeline: wgpu::RenderPipeline,
}

impl<'c, 'm> SolidColorPass<'c, 'm> {
    pub fn draw_single_object(
        &mut self,
        object: &'c ModelData,
        instance: &'c ModelInstance,
        material: MaterialId,
    ) {
        // TODO: material: texture / color
        // set per-object uniforms

        self.0.set_vertex_buffer(0, object.vertex_buffer.slice(..));
        self.0
            .set_index_buffer(object.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.0
            .set_bind_group(1, instance.model_uniform.bind_group(), &[]);
        self.0
            .set_bind_group(2, self.1.get_material_bindgroup(material), &[]);
        self.0.draw_indexed(0..object.num_elements, 0, 0..1);
    }
}

impl SolidColorRenderer {
    pub fn initialize(
        device: &wgpu::Device,
        gbuffer: &GBuffer,
        binding_layouts: &BindGroupLayouts,
        shaders: &ShaderManager,
    ) -> Result<Self, String> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("gbuffer fill shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Owned(shaders.get_shader("gbuffer_fill.wgsl")?)),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("SolidColor Pipeline"),
                bind_group_layouts: &[
                    &binding_layouts.single_uniform_group,
                    &binding_layouts.model,
                    &binding_layouts.material,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("SolidColor RP"),
            cache: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[ModelVertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                entry_point: Some("fs_main"),
                targets: &[
                    Some(wgpu::ColorTargetState {
                        format: gbuffer.albedo.texture.format(),
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                    Some(wgpu::ColorTargetState {
                        format: gbuffer.normal.texture.format(),
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                    Some(wgpu::ColorTargetState {
                        format: gbuffer.position.texture.format(),
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    }),
                ],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        Result::Ok(Self { render_pipeline })
    }

    pub fn render<'enc, 'a: 'enc>(
        &'a self,
        encoder: &'enc mut wgpu::CommandEncoder,
        materials: &'a MaterialLibrary,
        gbuffer: &'a GBuffer,
        util_data: &'a UtilDataUniform,
        clear_target: bool,
    ) -> SolidColorPass<'enc, 'a> {
        let color_clear_op = if clear_target {
            wgpu::LoadOp::Clear(wgpu::Color::BLACK)
        } else {
            wgpu::LoadOp::Load
        };

        let depth_clear_op = if clear_target {
            wgpu::LoadOp::Clear(1.0)
        } else {
            wgpu::LoadOp::Load
        };
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("SolidColor RP"),
            color_attachments: &[
                Some(wgpu::RenderPassColorAttachment {
                    view: &gbuffer.albedo.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: color_clear_op,
                        store: wgpu::StoreOp::Store,
                    },
                }),
                Some(wgpu::RenderPassColorAttachment {
                    view: &gbuffer.normal.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: color_clear_op,
                        store: wgpu::StoreOp::Store,
                    },
                }),
                Some(wgpu::RenderPassColorAttachment {
                    view: &gbuffer.position.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: color_clear_op,
                        store: wgpu::StoreOp::Store,
                    },
                }),
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &gbuffer.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: depth_clear_op,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, util_data.bind_group(), &[]);

        return SolidColorPass(render_pass, materials);
    }
}
