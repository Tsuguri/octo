use std::borrow::Cow;

use wgpu::TextureView;

use crate::bind_group_layouts::BindGroupLayouts;
use crate::dynamic_buffer::DynamicBuffer;
use crate::renderer::gbuffer::GBuffer;
use crate::renderer::LightsCollection;
use crate::shader_manager::ShaderManager;
use crate::uniforms::UtilDataUniform;
use crate::{DirectionalLightBuffer, Light, PointLightBuffer};

pub(crate) struct PostprocessingPass {
    render_pipeline: wgpu::RenderPipeline,
    source_bind_group: wgpu::BindGroup,
    directional_lights_data: DirectionalLightBuffer,
    directional_lights_buffer: DynamicBuffer<DirectionalLightBuffer>,
    point_lights_data: PointLightBuffer,
    point_lights_buffer: DynamicBuffer<PointLightBuffer>,
    lights_bind_group: wgpu::BindGroup,
}

impl PostprocessingPass {
    fn create_lights_bind_group(
        device: &wgpu::Device,
        binding_layouts: &BindGroupLayouts,
        directional: &DynamicBuffer<DirectionalLightBuffer>,
        point: &DynamicBuffer<PointLightBuffer>,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("gbuffer lights bind group"),
            layout: &binding_layouts.lights,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: directional.inner().as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: point.inner().as_entire_binding(),
                },
            ],
        })
    }
    pub fn initialize(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
        binding_layouts: &BindGroupLayouts,
        shaders: &ShaderManager,
        gbuffer: &GBuffer,
    ) -> Result<Self, String> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("gbuffer lighting shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Owned(
                shaders.get_shader("gbuffer_lighting.wgsl")?,
            )),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Standard Pipeline"),
                bind_group_layouts: &[
                    &binding_layouts.single_uniform_group,
                    &binding_layouts.postprocessing_sources,
                    &binding_layouts.lights,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("PP RP"),
            cache: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let source_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("pp source BG"),
            layout: &binding_layouts.postprocessing_sources,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&gbuffer.albedo.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&gbuffer.normal.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&gbuffer.position.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&gbuffer.albedo.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Sampler(&gbuffer.normal.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: wgpu::BindingResource::Sampler(&gbuffer.position.sampler),
                },
            ],
        });
        let directional_lights_data = Default::default();
        let directional_lights_buffer = DynamicBuffer::new(device, wgpu::BufferUsages::STORAGE);
        let point_lights_data = Default::default();
        let point_lights_buffer = DynamicBuffer::new(device, wgpu::BufferUsages::STORAGE);

        let lights_bind_group = Self::create_lights_bind_group(
            device,
            binding_layouts,
            &directional_lights_buffer,
            &point_lights_buffer,
        );

        Result::Ok(Self {
            render_pipeline,
            source_bind_group,
            directional_lights_data,
            directional_lights_buffer,
            point_lights_data,
            point_lights_buffer,
            lights_bind_group,
        })
    }

    pub fn update_buffers(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        binding_layouts: &BindGroupLayouts,
        lights: &LightsCollection,
    ) {
        let mut point_lights = Vec::new();
        let mut directional_lights = Vec::new();

        for val in lights.values() {
            match val {
                Light::Directional(data) => directional_lights.push(*data),
                Light::Point(data) => point_lights.push(*data),
            }
        }

        self.directional_lights_data.array = directional_lights;
        self.point_lights_data.array = point_lights;

        let changed =
            self.directional_lights_buffer
                .update(device, queue, &self.directional_lights_data)
                || self
                    .point_lights_buffer
                    .update(device, queue, &self.point_lights_data);

        if changed {
            self.lights_bind_group = Self::create_lights_bind_group(
                device,
                binding_layouts,
                &self.directional_lights_buffer,
                &self.point_lights_buffer,
            );
        }
    }
    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        color_target: &TextureView,
        util_data: &UtilDataUniform,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("postprocessing render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: color_target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, util_data.bind_group(), &[]);
        render_pass.set_bind_group(1, &self.source_bind_group, &[]);
        render_pass.set_bind_group(2, &self.lights_bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}
