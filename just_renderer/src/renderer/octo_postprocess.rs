use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::num::NonZeroU32;

use octo_runtime::{InputType, OctoModule, OutputType, TextureId, TextureSize, TextureType};

use crate::texture::TextureData;

#[derive(Debug, Clone)]
pub struct OctoPostprocessError {
    message: String,
}

impl OctoPostprocessError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for OctoPostprocessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for OctoPostprocessError {}

#[derive(Debug, Clone)]
pub enum OctoModuleStatus {
    Empty,
    Ready { name: String },
    Error { name: String, message: String },
}

enum OctoPassOutput {
    Result,
    Textures(Vec<TextureId>),
}

struct OctoPassRuntime {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: Option<wgpu::BindGroupLayout>,
    inputs: Vec<InputType>,
    output: OctoPassOutput,
}

pub(crate) struct OctoPostprocessState {
    name: String,
    uniform_bytes: Vec<u8>,
    sampler: wgpu::Sampler,
    pipeline_textures: HashMap<TextureId, TextureData>,
    passes: Vec<OctoPassRuntime>,
}

impl OctoPostprocessState {
    pub(crate) fn create(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        surface_size: winit::dpi::PhysicalSize<u32>,
        module: &OctoModule,
    ) -> Result<Self, OctoPostprocessError> {
        Self::validate_device(device, module)?;
        Self::validate_graph(module, device.limits().max_color_attachments as usize)?;

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("octo postprocess sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let pipeline_textures = Self::create_pipeline_textures(device, surface_size, module)?;
        let vertex_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("octo fullscreen vertex shader"),
            source: wgpu::ShaderSource::SpirV(Cow::Borrowed(&module.basic_vertex_spirv)),
        });

        let mut passes = Vec::with_capacity(module.passes.len());
        for pass in &module.passes {
            let fragment_words = module.fragment_shaders.get(&pass.shader).ok_or_else(|| {
                OctoPostprocessError::new(format!(
                    "pass {} references missing fragment shader {}",
                    pass.id, pass.shader
                ))
            })?;
            let fragment_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("octo fragment shader"),
                source: wgpu::ShaderSource::SpirV(Cow::Borrowed(fragment_words)),
            });

            let bind_group_layout = Self::create_pass_bind_group_layout(device, pass.input.len())?;
            let bind_group_layout_refs: Vec<Option<&wgpu::BindGroupLayout>> = bind_group_layout
                .as_ref()
                .map_or_else(Vec::new, |layout| vec![Some(layout)]);
            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("octo postprocess pipeline layout"),
                bind_group_layouts: &bind_group_layout_refs,
                immediate_size: module.uniform_block_size as u32,
            });

            let target_states = Self::pass_target_states(surface_format, module, &pass.output)?;
            let targets: Vec<_> = target_states
                .into_iter()
                .map(|(format, write_mask)| {
                    Some(wgpu::ColorTargetState {
                        format,
                        blend: None,
                        write_mask,
                    })
                })
                .collect();

            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("octo postprocess pipeline"),
                cache: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vertex_shader,
                    entry_point: Some("main"),
                    buffers: &[],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &fragment_shader,
                    entry_point: Some("main"),
                    targets: &targets,
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
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
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
            });

            passes.push(OctoPassRuntime {
                pipeline,
                bind_group_layout,
                inputs: pass.input.clone(),
                output: match &pass.output {
                    OutputType::Result => OctoPassOutput::Result,
                    OutputType::Textures(ids) => OctoPassOutput::Textures(ids.clone()),
                },
            });
        }

        Ok(Self {
            name: module.name.clone(),
            uniform_bytes: vec![0; module.uniform_block_size],
            sampler,
            pipeline_textures,
            passes,
        })
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn set_uniform_bytes(&mut self, bytes: Vec<u8>) -> Result<(), OctoPostprocessError> {
        if bytes.len() != self.uniform_bytes.len() {
            return Err(OctoPostprocessError::new(format!(
                "uniform block size mismatch: got {} bytes, expected {}",
                bytes.len(),
                self.uniform_bytes.len()
            )));
        }

        self.uniform_bytes = bytes;
        Ok(())
    }

    pub(crate) fn render(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        scene_color: &TextureData,
        target: &wgpu::TextureView,
    ) {
        for pass in &self.passes {
            let bind_group = pass.bind_group_layout.as_ref().map(|layout| {
                let input_views: Vec<&wgpu::TextureView> = pass
                    .inputs
                    .iter()
                    .map(|input| match input {
                        InputType::ProvidedTexture(_) => &scene_color.view,
                        InputType::PipelineTexture(id) => &self.pipeline_textures[id].view,
                    })
                    .collect();

                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("octo postprocess bind group"),
                    layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Sampler(&self.sampler),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureViewArray(&input_views),
                        },
                    ],
                })
            });

            let mut color_attachments = Vec::new();
            match &pass.output {
                OctoPassOutput::Result => {
                    color_attachments.push(Some(wgpu::RenderPassColorAttachment {
                        view: target,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    }));
                }
                OctoPassOutput::Textures(ids) => {
                    for id in ids {
                        color_attachments.push(Some(wgpu::RenderPassColorAttachment {
                            view: &self.pipeline_textures[id].view,
                            depth_slice: None,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: wgpu::StoreOp::Store,
                            },
                        }));
                    }
                }
            }

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("octo postprocess render pass"),
                color_attachments: &color_attachments,
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(&pass.pipeline);
            if let Some(bind_group) = &bind_group {
                render_pass.set_bind_group(0, bind_group, &[]);
            }
            if !self.uniform_bytes.is_empty() {
                render_pass.set_immediates(0, &self.uniform_bytes);
            }
            render_pass.draw(0..3, 0..1);
        }
    }

    fn validate_device(
        device: &wgpu::Device,
        module: &OctoModule,
    ) -> Result<(), OctoPostprocessError> {
        if module.basic_vertex_spirv.is_empty() {
            return Err(OctoPostprocessError::new(
                "module has no fullscreen vertex SPIR-V",
            ));
        }

        if module.passes.iter().any(|pass| !pass.input.is_empty())
            && !device
                .features()
                .contains(wgpu::Features::TEXTURE_BINDING_ARRAY)
        {
            return Err(OctoPostprocessError::new(
                "device does not support texture binding arrays required by octo inputs",
            ));
        }

        let max_input_count = module
            .passes
            .iter()
            .map(|pass| pass.input.len())
            .max()
            .unwrap_or(0);
        if max_input_count as u32 > device.limits().max_binding_array_elements_per_shader_stage {
            return Err(OctoPostprocessError::new(format!(
                "module needs {} texture array elements but device supports {}",
                max_input_count,
                device.limits().max_binding_array_elements_per_shader_stage
            )));
        }

        if module.uniform_block_size > 0 {
            if !device.features().contains(wgpu::Features::IMMEDIATES) {
                return Err(OctoPostprocessError::new(
                    "device does not support immediate data required by octo uniforms",
                ));
            }
            if module.uniform_block_size as u32 > device.limits().max_immediate_size {
                return Err(OctoPostprocessError::new(format!(
                    "uniform block is {} bytes but device supports only {} immediate bytes",
                    module.uniform_block_size,
                    device.limits().max_immediate_size
                )));
            }
        }

        Ok(())
    }

    fn validate_graph(
        module: &OctoModule,
        max_color_attachments: usize,
    ) -> Result<(), OctoPostprocessError> {
        let declared_textures: HashSet<TextureId> =
            module.textures.iter().map(|(id, _, _)| *id).collect();
        let mut produced_textures = HashSet::new();
        let mut seen_passes = HashSet::new();
        let mut result_passes = 0usize;

        for (pass_index, pass) in module.passes.iter().enumerate() {
            if !seen_passes.insert(pass.id) {
                return Err(OctoPostprocessError::new(format!(
                    "duplicate pass id {}",
                    pass.id
                )));
            }

            if !module.fragment_shaders.contains_key(&pass.shader) {
                return Err(OctoPostprocessError::new(format!(
                    "pass {} references missing fragment shader {}",
                    pass.id, pass.shader
                )));
            }

            if let Some(dependencies) = &pass.dependencies {
                for dependency in dependencies {
                    if !seen_passes.contains(dependency) {
                        return Err(OctoPostprocessError::new(format!(
                            "pass {} depends on pass {} before it has run",
                            pass.id, dependency
                        )));
                    }
                }
            }

            for input in &pass.input {
                if let InputType::PipelineTexture(id) = input {
                    if !produced_textures.contains(id) {
                        return Err(OctoPostprocessError::new(format!(
                            "pass {} reads pipeline texture {} before it is produced",
                            pass.id, id
                        )));
                    }
                }
            }

            match &pass.output {
                OutputType::Result => {
                    result_passes += 1;
                    if pass_index + 1 != module.passes.len() {
                        return Err(OctoPostprocessError::new(
                            "OutputType::Result must be the final octo pass",
                        ));
                    }
                }
                OutputType::Textures(ids) => {
                    if ids.is_empty() {
                        return Err(OctoPostprocessError::new(format!(
                            "pass {} has no output textures",
                            pass.id
                        )));
                    }
                    if ids.len() > max_color_attachments {
                        return Err(OctoPostprocessError::new(format!(
                            "pass {} writes {} color attachments but the device supports {}",
                            pass.id,
                            ids.len(),
                            max_color_attachments
                        )));
                    }
                    for id in ids {
                        if !declared_textures.contains(id) {
                            return Err(OctoPostprocessError::new(format!(
                                "pass {} writes undeclared pipeline texture {}",
                                pass.id, id
                            )));
                        }
                        if pass
                            .input
                            .iter()
                            .any(|input| matches!(input, InputType::PipelineTexture(input_id) if input_id == id))
                        {
                            return Err(OctoPostprocessError::new(format!(
                                "pass {} reads and writes pipeline texture {} in the same pass",
                                pass.id, id
                            )));
                        }
                        produced_textures.insert(*id);
                    }
                }
            }
        }

        if result_passes != 1 {
            return Err(OctoPostprocessError::new(format!(
                "expected exactly one final Result pass, found {}",
                result_passes
            )));
        }

        Ok(())
    }

    fn create_pipeline_textures(
        device: &wgpu::Device,
        surface_size: winit::dpi::PhysicalSize<u32>,
        module: &OctoModule,
    ) -> Result<HashMap<TextureId, TextureData>, OctoPostprocessError> {
        let mut textures = HashMap::new();
        for (id, texture_type, texture_size) in &module.textures {
            let size = Self::map_texture_size(surface_size, texture_size)?;
            let format = Self::map_texture_format(*texture_type);
            textures.insert(
                *id,
                TextureData::create_render_target(
                    device,
                    size,
                    format,
                    &format!("octo pipeline texture {}", id),
                ),
            );
        }

        Ok(textures)
    }

    fn map_texture_size(
        surface_size: winit::dpi::PhysicalSize<u32>,
        texture_size: &TextureSize,
    ) -> Result<wgpu::Extent3d, OctoPostprocessError> {
        let (width, height) = match texture_size {
            TextureSize::Original => (surface_size.width, surface_size.height),
            TextureSize::Scaled(scale) => {
                if !scale.is_finite() || *scale <= 0.0 {
                    return Err(OctoPostprocessError::new(format!(
                        "invalid scaled texture size {}",
                        scale
                    )));
                }
                (
                    ((surface_size.width as f32 * scale).round() as u32).max(1),
                    ((surface_size.height as f32 * scale).round() as u32).max(1),
                )
            }
            TextureSize::Custom(width, height) => ((*width).max(1), (*height).max(1)),
        };

        Ok(wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        })
    }

    fn create_pass_bind_group_layout(
        device: &wgpu::Device,
        input_count: usize,
    ) -> Result<Option<wgpu::BindGroupLayout>, OctoPostprocessError> {
        let Some(count) = NonZeroU32::new(input_count as u32) else {
            return Ok(None);
        };

        Ok(Some(device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("octo postprocess BGL"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        },
                        count: Some(count),
                    },
                ],
            },
        )))
    }

    fn pass_target_states(
        surface_format: wgpu::TextureFormat,
        module: &OctoModule,
        output: &OutputType,
    ) -> Result<Vec<(wgpu::TextureFormat, wgpu::ColorWrites)>, OctoPostprocessError> {
        match output {
            OutputType::Result => Ok(vec![(surface_format, wgpu::ColorWrites::ALL)]),
            OutputType::Textures(ids) => ids
                .iter()
                .map(|id| {
                    let (_, texture_type, _) = module
                        .textures
                        .iter()
                        .find(|(texture_id, _, _)| texture_id == id)
                        .ok_or_else(|| {
                            OctoPostprocessError::new(format!(
                                "output references undeclared pipeline texture {}",
                                id
                            ))
                        })?;
                    Ok((
                        Self::map_texture_format(*texture_type),
                        wgpu::ColorWrites::ALL,
                    ))
                })
                .collect(),
        }
    }

    fn map_texture_format(texture_type: TextureType) -> wgpu::TextureFormat {
        match texture_type {
            TextureType::Float => wgpu::TextureFormat::R32Float,
            TextureType::Vec2 => wgpu::TextureFormat::Rg32Float,
            TextureType::Vec4 => wgpu::TextureFormat::Rgba32Float,
        }
    }
}

#[cfg(test)]
mod tests {
    use octo_runtime::{ShaderId, ShaderPass};

    use super::*;

    fn test_module(passes: Vec<ShaderPass>) -> OctoModule {
        let mut module = OctoModule::new();
        module.fragment_shaders.insert(0 as ShaderId, vec![1]);
        module
            .textures
            .push((0, TextureType::Vec4, TextureSize::Original));
        module.passes = passes;
        module
    }

    #[test]
    fn validates_simple_result_pass() {
        let module = test_module(vec![ShaderPass {
            id: 0,
            input: vec![InputType::ProvidedTexture(0)],
            output: OutputType::Result,
            shader: 0,
            dependencies: None,
        }]);

        assert!(OctoPostprocessState::validate_graph(&module, 8).is_ok());
    }

    #[test]
    fn rejects_pipeline_texture_before_it_is_produced() {
        let module = test_module(vec![ShaderPass {
            id: 0,
            input: vec![InputType::PipelineTexture(0)],
            output: OutputType::Result,
            shader: 0,
            dependencies: None,
        }]);

        assert!(OctoPostprocessState::validate_graph(&module, 8).is_err());
    }

    #[test]
    fn rejects_result_pass_that_is_not_final() {
        let module = test_module(vec![
            ShaderPass {
                id: 0,
                input: vec![InputType::ProvidedTexture(0)],
                output: OutputType::Result,
                shader: 0,
                dependencies: None,
            },
            ShaderPass {
                id: 1,
                input: vec![InputType::ProvidedTexture(0)],
                output: OutputType::Textures(vec![0]),
                shader: 0,
                dependencies: None,
            },
        ]);

        assert!(OctoPostprocessState::validate_graph(&module, 8).is_err());
    }
}
