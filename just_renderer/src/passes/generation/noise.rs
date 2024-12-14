use std::num::NonZeroU8;

use glam::U64Vec2;
use glam::Vec2;
use strum::EnumCount;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::texture::TextureData;

#[allow(unused)]
#[repr(u8)]
#[derive(Debug, EnumCount, Clone, Copy)]
pub enum NoiseType {
    Perlin = 1,
    Worley,
    Cauliflower,
}

pub struct NoiseParameters {
    pub size: U64Vec2,
    pub frequency: Vec2, // scaling of the noise. First octave if multiple are used
    pub uv_offset: Vec2, // offset can be used to create tiling textures (in theory)
    pub ty: NoiseType,
    pub octaves: NonZeroU8,
}

impl Default for NoiseParameters {
    fn default() -> Self {
        Self {
            size: U64Vec2::new(512, 512),
            frequency: Vec2::new(16.0, 16.0),
            uv_offset: Vec2::ZERO,
            ty: NoiseType::Perlin,
            octaves: unsafe { NonZeroU8::new_unchecked(1) },
        }
    }
}

#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
#[repr(C)]
struct NoiseParametersUniform {
    size: Vec2,
    frequency: Vec2, // higher value -> denser,
    uv_offset: Vec2, // defaults to 0,0. Can be used to generate tiling textures
    ty: u32,
    octaves: u32,
}

impl From<&NoiseParameters> for NoiseParametersUniform {
    fn from(value: &NoiseParameters) -> Self {
        Self {
            size: Vec2::new(value.size.x as f32, value.size.y as f32),
            frequency: value.frequency,
            uv_offset: value.uv_offset,
            ty: value.ty as u32,
            octaves: value.octaves.get() as u32,
        }
    }
}

struct NoiseGenerationJob {
    bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
    size: U64Vec2,
}

pub struct NoiseGenerator {
    compute_pipeline: wgpu::ComputePipeline,
    pending_jobs: Vec<NoiseGenerationJob>,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl NoiseGenerator {
    pub fn initialize(device: &wgpu::Device) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("Noise.wgsl"));

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Noise bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    count: None,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Noise gen layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Noise gen RP"),
            cache: None,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            entry_point: Some("main"),
            layout: Some(&pipeline_layout),
            module: &shader,
        });

        Self {
            compute_pipeline,
            pending_jobs: Vec::new(),
            bind_group_layout,
        }
    }

    pub fn create_noise_texture(
        &mut self,
        device: &wgpu::Device,
        parameters: NoiseParameters,
    ) -> TextureData {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: parameters.size.x as u32,
                height: parameters.size.y as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let uniform_data = NoiseParametersUniform::from(&parameters);

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Noise constructrion data buffer"),
            contents: bytemuck::cast_slice(&[uniform_data]),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Noise parameters"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
            ],
        });

        self.pending_jobs.push(NoiseGenerationJob {
            buffer,
            bind_group,
            size: parameters.size,
        });

        TextureData {
            texture,
            view,
            sampler,
        }
    }

    pub fn submit_jobs<'enc, 'a: 'enc>(&'a mut self, encoder: &'enc mut wgpu::CommandEncoder) {
        for NoiseGenerationJob {
            buffer,
            bind_group,
            size,
        } in std::mem::replace(&mut self.pending_jobs, Vec::new())
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });

            pass.set_pipeline(&self.compute_pipeline);

            pass.set_bind_group(0, &bind_group, &[]);
            println!("dispatching {} by {} workgroups", size.x, size.y);
            pass.dispatch_workgroups(size.x as u32, size.y as u32, 1);
        }
    }
}
