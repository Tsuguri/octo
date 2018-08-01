use glsl_to_spirv;
use hal;
use hal::adapter::MemoryType;
use hal::buffer;
use hal::format;
use hal::image;
use hal::memory;
use hal::pass;
use hal::pass::Subpass;
use hal::pso::ShaderStageFlags;
use hal::{pso, DescriptorPool, Device};
use std;
use std::io::Read;
use types::Vertex;

pub trait OctoPipeline<B: hal::Backend> {
    fn fill_command_buffer(
        &self,
        cmd_buffer: &mut hal::command::CommandBuffer<B, hal::Graphics>,
        framebuffer: &B::Framebuffer,
        rect: hal::pso::Rect,
    );
}

const QUAD: [Vertex; 6] = [
    Vertex {
        pos: [-0.5, 0.33],
        uv: [0.0, 1.0],
    },
    Vertex {
        pos: [0.5, 0.33],
        uv: [1.0, 1.0],
    },
    Vertex {
        pos: [0.5, -0.33],
        uv: [1.0, 0.0],
    },
    Vertex {
        pos: [-0.5, 0.33],
        uv: [0.0, 1.0],
    },
    Vertex {
        pos: [0.5, -0.33],
        uv: [1.0, 0.0],
    },
    Vertex {
        pos: [-0.5, -0.33],
        uv: [0.0, 0.0],
    },
];
const ENTRY_NAME: &str = "main";

pub struct Pipeline<B: hal::Backend> {
    descriptors_pool: B::DescriptorPool,
    descriptors_set: B::DescriptorSet, // does not have to be freed/destroyed
    image_sampler: B::Sampler,
    vertex_buffer_memory: B::Memory,
    vertex_buffer: B::Buffer,
    render_pass: B::RenderPass,
    pipeline_layout: B::PipelineLayout,
    pipeline: B::GraphicsPipeline,
}

impl<B: hal::Backend> Pipeline<B> {
    pub fn new(
        device: &B::Device,
        memory_types: &[MemoryType],
        format: format::Format,
    ) -> Pipeline<B> {
        let mut descriptors_pool = device.create_descriptor_pool(
            1, // sets
            &[
                pso::DescriptorRangeDesc {
                    ty: pso::DescriptorType::SampledImage,
                    count: 1,
                },
                pso::DescriptorRangeDesc {
                    ty: pso::DescriptorType::Sampler,
                    count: 1,
                },
            ],
        );
        let set_layout = device.create_descriptor_set_layout(
            &[
                pso::DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: pso::DescriptorType::SampledImage,
                    count: 1,
                    stage_flags: ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                },
                pso::DescriptorSetLayoutBinding {
                    binding: 1,
                    ty: pso::DescriptorType::Sampler,
                    count: 1,
                    stage_flags: ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                },
            ],
            &[],
        );
        let descriptors_set = descriptors_pool.allocate_set(&set_layout).unwrap();
        let image_sampler = device.create_sampler(image::SamplerInfo::new(
            image::Filter::Linear,
            image::WrapMode::Clamp,
        ));
        let vertex_buffer_size = std::mem::size_of::<Vertex>() as u64 * QUAD.len() as u64;
        let unbound_vertex_buffer = device
            .create_buffer(vertex_buffer_size, buffer::Usage::VERTEX)
            .unwrap();

        let buffer_requirements = device.get_buffer_requirements(&unbound_vertex_buffer);
        let upload_type = memory_types
            .iter()
            .enumerate()
            .position(|(id, mem_type)| {
                buffer_requirements.type_mask & (1 << id) != 0
                    && mem_type
                        .properties
                        .contains(memory::Properties::CPU_VISIBLE)
            })
            .unwrap()
            .into();
        let vertex_buffer_memory = device
            .allocate_memory(upload_type, buffer_requirements.size)
            .unwrap();
        let vertex_buffer = device
            .bind_buffer_memory(&vertex_buffer_memory, 0, unbound_vertex_buffer)
            .unwrap();

        {
            let mut vertices = device
                .acquire_mapping_writer::<Vertex>(&vertex_buffer_memory, 0..vertex_buffer_size)
                .unwrap();
            vertices.copy_from_slice(&QUAD);
            device.release_mapping_writer(vertices);
        }
        let render_pass = {
            let attachment = pass::Attachment {
                format: Some(format),
                samples: 1,
                ops: pass::AttachmentOps::new(
                    pass::AttachmentLoadOp::Clear,
                    pass::AttachmentStoreOp::Store,
                ),
                stencil_ops: pass::AttachmentOps::DONT_CARE,
                layouts: image::Layout::Undefined..image::Layout::Present,
            };

            let subpass = pass::SubpassDesc {
                colors: &[(0, image::Layout::ColorAttachmentOptimal)],
                depth_stencil: None,
                inputs: &[],
                resolves: &[],
                preserves: &[],
            };

            let dependency = pass::SubpassDependency {
                passes: pass::SubpassRef::External..pass::SubpassRef::Pass(0),
                stages: pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT
                    ..pso::PipelineStage::COLOR_ATTACHMENT_OUTPUT,
                accesses: image::Access::empty()
                    ..(image::Access::COLOR_ATTACHMENT_READ
                        | image::Access::COLOR_ATTACHMENT_WRITE),
            };

            device.create_render_pass(&[attachment], &[subpass], &[dependency])
        };
        let pipeline_layout = device
            .create_pipeline_layout(Some(set_layout), &[(pso::ShaderStageFlags::VERTEX, 0..8)]);
        let vs_module = {
            let glsl = include_str!("data/quad.vert");
            let spirv: Vec<u8> = glsl_to_spirv::compile(&glsl, glsl_to_spirv::ShaderType::Vertex)
                .unwrap()
                .bytes()
                .map(|b| b.unwrap())
                .collect();
            device.create_shader_module(&spirv).unwrap()
        };
        let fs_module = {
            let glsl = include_str!("data/quad.frag");
            let spirv: Vec<u8> = glsl_to_spirv::compile(&glsl, glsl_to_spirv::ShaderType::Fragment)
                .unwrap()
                .bytes()
                .map(|b| b.unwrap())
                .collect();
            device.create_shader_module(&spirv).unwrap()
        };
        let pipeline = {
            let pipeline = {
                let subpass = Subpass {
                    index: 0,
                    main_pass: &render_pass,
                };
                let (vs_entry, fs_entry) = (
                    pso::EntryPoint {
                        entry: ENTRY_NAME,
                        module: &vs_module,
                        specialization: &[hal::pso::Specialization {
                            id: 0,
                            value: pso::Constant::F32(0.8),
                        }],
                    },
                    pso::EntryPoint::<B> {
                        entry: ENTRY_NAME,
                        module: &fs_module,
                        specialization: &[],
                    },
                );

                let shader_entries = pso::GraphicsShaderSet {
                    vertex: vs_entry,
                    hull: None,
                    domain: None,
                    geometry: None,
                    fragment: Some(fs_entry),
                };
                let mut pipeline_desc = pso::GraphicsPipelineDesc::new(
                    shader_entries,
                    hal::Primitive::TriangleList,
                    pso::Rasterizer::FILL,
                    &pipeline_layout,
                    subpass,
                );
                pipeline_desc.blender.targets.push(pso::ColorBlendDesc(
                    pso::ColorMask::ALL,
                    pso::BlendState::ALPHA,
                ));
                pipeline_desc.vertex_buffers.push(pso::VertexBufferDesc {
                    binding: 0,
                    stride: std::mem::size_of::<Vertex>() as u32,
                    rate: 0,
                });

                pipeline_desc.attributes.push(pso::AttributeDesc {
                    location: 0,
                    binding: 0,
                    element: pso::Element {
                        format: format::Format::Rg32Float,
                        offset: 0,
                    },
                });
                pipeline_desc.attributes.push(pso::AttributeDesc {
                    location: 1,
                    binding: 0,
                    element: pso::Element {
                        format: format::Format::Rg32Float,
                        offset: 8,
                    },
                });

                let p = device.create_graphics_pipeline(&pipeline_desc);
                // drop(pipeline_desc);
                p
            };

            pipeline.unwrap()
        };
        device.destroy_shader_module(vs_module);
        device.destroy_shader_module(fs_module);
        Pipeline {
            descriptors_pool,
            descriptors_set,
            image_sampler,
            vertex_buffer_memory,
            vertex_buffer,
            render_pass,
            pipeline_layout,
            pipeline,
        }
    }

    pub fn destroy(self, device: &B::Device) {
        device.destroy_descriptor_pool(self.descriptors_pool);
        device.destroy_render_pass(self.render_pass);
        device.destroy_sampler(self.image_sampler);
        device.destroy_buffer(self.vertex_buffer);
        device.free_memory(self.vertex_buffer_memory);
        device.destroy_graphics_pipeline(self.pipeline);
        device.destroy_pipeline_layout(self.pipeline_layout);
    }
}

impl<B: hal::Backend> OctoPipeline<B> for Pipeline<B> {
    fn fill_command_buffer(
        &self,
        cmd_buffer: &mut hal::command::CommandBuffer<B, hal::Graphics>,
        framebuffer: &B::Framebuffer,
        rect: hal::pso::Rect,
    ) {
        cmd_buffer.bind_graphics_pipeline(&self.pipeline);
        cmd_buffer.bind_vertex_buffers(0, Some((&self.vertex_buffer, 0)));
        cmd_buffer.bind_graphics_descriptor_sets(
            &self.pipeline_layout,
            0,
            Some(&self.descriptors_set),
            &[],
        );
        {
            let mut encoder = cmd_buffer.begin_render_pass_inline(
                &self.render_pass,
                framebuffer,
                rect,
                &[hal::command::ClearValue::Color(
                    hal::command::ClearColor::Float([0.3, 0.8, 0.8, 1.0]),
                )],
            );
            encoder.draw(0..6, 0..1);
        }
    }
}
