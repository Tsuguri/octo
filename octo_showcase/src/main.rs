use core::mem::{ManuallyDrop, size_of};

use gfx_backend_vulkan as back;
use gfx_hal::{
    Backend,
    DescriptorPool,
    device::Device,
    //memory::{Properties, Requirements},
    format::Format,
    pass::Subpass,
    Primitive,
    pso::{
        AttributeDesc, BakedStates, BasePipeline, BlendDesc, BlendOp, BlendState, ColorBlendDesc,
        ColorMask, DepthStencilDesc, DepthTest, DescriptorSetLayoutBinding, Element, ElemOffset,
        ElemStride, EntryPoint, Face, Factor, FrontFace, GraphicsPipelineDesc, GraphicsShaderSet,
        InputAssemblerDesc, LogicOp, PipelineCreationFlags, PolygonMode, Rasterizer,
        ShaderStageFlags, Specialization, StencilTest, VertexBufferDesc, Viewport,
    },
    window::Extent2D,
};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use serde_json;
use nalgebra_glm as glm;

use hal::HalState;
use input::UserInput;
use octo_runtime::OctoModule;
use state::LocalState;
use window::WinitState;

mod window;
mod input;
mod state;
mod hal;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub points: [[f32; 2]; 3],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vertex {
    xyz: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
}

impl Vertex {
    pub fn size() -> usize {
        size_of::<f32>() * 8
    }
    pub fn attributes() -> Vec<AttributeDesc> {
        let position_attribute = AttributeDesc {
            location: 0,
            binding: 0,
            element: Element {
                format: Format::Rgb32Float,
                offset: 0,
            },
        };
        let normal_attribute = AttributeDesc {
            location: 1,
            binding: 0,
            element: Element {
                format: Format::Rgb32Float,
                offset: size_of::<[f32; 3]>() as ElemOffset,

            },
        };
        let uv_attribute = AttributeDesc {
            location: 2,
            binding: 0,
            element: Element {
                format: Format::Rg32Float,
                offset: (size_of::<[f32; 3]>() * 2) as ElemOffset,
            },
        };
        vec![position_attribute, normal_attribute, uv_attribute]
    }
}

pub static CREATURE_BYTES: &[u8] = include_bytes!("creature.png");

#[derive(Debug, Clone, Copy)]
pub struct Quad {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Quad {
    pub fn vertex_attributes(self) -> [f32; 4 * (3 + 3 + 2)] {
        let x = self.x;
        let y = self.y;
        let w = self.w;
        let h = self.h;

        [
            x,
            y + h,
            0.0,
            0.0, 0.0, 0.0,
            0.0,
            1.0,
            x,
            y,
            0.0,
            0.0, 0.0, 0.0,
            0.0,
            0.0,
            x + w,
            y,
            0.0,
            0.0, 0.0, 0.0,
            1.0,
            0.0,
            x + w,
            y + h,
            0.0,
            0.0, 0.0, 0.0,
            1.0,
            1.0,
        ]
    }
}

impl Triangle {
    pub fn points_flat(self) -> [f32; 3 * (2 + 3)] {
        let [[a, b], [c, d], [e, f]] = self.points;
        [
            a, b, 1.0, 0.0, 0.0, c, d, 0.0, 1.0, 0.0, e, f, 0.0, 0.0, 1.0,
        ]
    }
}

use hal::pipeline::Pipeline;
use std::time::Instant;
use crate::state::Object;

impl HalState {
    fn create_pipeline(
        device: &mut back::Device,
        extent: Extent2D,
        render_pass: &<back::Backend as Backend>::RenderPass,
    ) -> Result<Pipeline, &'static str> {
        let f = include_str!("file.octo_bin");

        let module: OctoModule = serde_json::from_str(&f).unwrap();
        drop(f);
        println!("Loading octo module: {}", module.name);

        let vert = include_bytes!("vert.glsl.spirv");
        let frag = include_bytes!("frag.glsl.spirv");

        let vertex_shader_module = unsafe {
            device
                .create_shader_module(vert)
                .map_err(|_| "Couldn't make the vertex module")?
        };
        let fragment_shader_module = unsafe {
            device
                .create_shader_module(frag)
                .map_err(|_| "Couldn't make the fragment module")?
        };
        let (
            descriptor_set_layouts,
            descriptor_pool,
            descriptor_set,
            pipeline_layout,
            gfx_pipeline,
        ) = {
            let (vs_entry, fs_entry) = (
                EntryPoint {
                    entry: "main",
                    module: &vertex_shader_module,
                    specialization: Specialization {
                        constants: &[],
                        data: &[],
                    },
                },
                EntryPoint {
                    entry: "main",
                    module: &fragment_shader_module,
                    specialization: Specialization {
                        constants: &[],
                        data: &[],
                    },
                },
            );
            let shaders = GraphicsShaderSet {
                vertex: vs_entry,
                hull: None,
                domain: None,
                geometry: None,
                fragment: Some(fs_entry),
            };

            let input_assembler = InputAssemblerDesc::new(Primitive::TriangleList);

            let vertex_buffers: Vec<VertexBufferDesc> = vec![VertexBufferDesc {
                binding: 0,
                stride: (Vertex::size()) as ElemStride,
                rate: 0,
            }];
            let attributes = Vertex::attributes();

            let rasterizer = Rasterizer {
                depth_clamping: false,
                polygon_mode: PolygonMode::Fill,
                cull_face: Face::BACK,
                front_face: FrontFace::Clockwise,
                depth_bias: None,
                conservative: false,
            };
            let depth_stencil = DepthStencilDesc {
                depth: DepthTest::On {
                    fun: gfx_hal::pso::Comparison::LessEqual,
                    write: true,
                },
                depth_bounds: false,
                stencil: StencilTest::Off,
            };

            let blender = {
                let blend_state = BlendState::On {
                    color: BlendOp::Add {
                        src: Factor::One,
                        dst: Factor::Zero,
                    },
                    alpha: BlendOp::Add {
                        src: Factor::One,
                        dst: Factor::Zero,
                    },
                };
                BlendDesc {
                    logic_op: Some(LogicOp::Copy),
                    targets: vec![ColorBlendDesc(ColorMask::ALL, blend_state)],
                }
            };

            let baked_states = BakedStates {
                viewport: Some(Viewport {
                    rect: extent.to_extent().rect(),
                    depth: (0.0..1.0),
                }),
                scissor: Some(extent.to_extent().rect()),
                blend_color: None,
                depth_bounds: None,
            };

            let bindings = vec![
                DescriptorSetLayoutBinding {
                    binding: 0,
                    ty: gfx_hal::pso::DescriptorType::SampledImage,
                    count: 1,
                    stage_flags: ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                },
                DescriptorSetLayoutBinding {
                    binding: 1,
                    ty: gfx_hal::pso::DescriptorType::Sampler,
                    count: 1,
                    stage_flags: ShaderStageFlags::FRAGMENT,
                    immutable_samplers: false,
                },
            ];

            let descriptor_set_layouts: Vec<<back::Backend as Backend>::DescriptorSetLayout> =
                vec![unsafe {
                    device
                        .create_descriptor_set_layout(bindings, std::iter::empty::<<back::Backend as Backend>::Sampler>())
                        .map_err(|_| "Couldn't make a DescriptorSetLayout")?
                }];

            let mut descriptor_pool = unsafe {
                device
                    .create_descriptor_pool(
                        1,
                        &[
                            gfx_hal::pso::DescriptorRangeDesc {
                                ty: gfx_hal::pso::DescriptorType::SampledImage,
                                count: 1,
                            },
                            gfx_hal::pso::DescriptorRangeDesc {
                                ty: gfx_hal::pso::DescriptorType::Sampler,
                                count: 1,
                            },
                        ],
                    )
                    .map_err(|_| "Couldn't create a descriptor pool!")?
            };

            let descriptor_set = unsafe {
                descriptor_pool
                    .allocate_set(&descriptor_set_layouts[0])
                    .map_err(|_| "Couldn't make a Descriptor Set")?
            };

            let push_constants = vec![(ShaderStageFlags::VERTEX, 0..48)];
            let layout = unsafe {
                device
                    .create_pipeline_layout(&descriptor_set_layouts, push_constants)
                    .map_err(|_| "Couldn't create a pipeline layout")?
            };

            let gfx_pipeline = {
                let desc = GraphicsPipelineDesc {
                    shaders,
                    rasterizer,
                    vertex_buffers,
                    attributes,
                    input_assembler,
                    blender,
                    depth_stencil,
                    multisampling: None,
                    baked_states,
                    layout: &layout,
                    subpass: Subpass {
                        index: 0,
                        main_pass: render_pass,
                    },
                    flags: PipelineCreationFlags::empty(),
                    parent: BasePipeline::None,
                };

                unsafe {
                    device
                        .create_graphics_pipeline(&desc, None)
                        .map_err(|_| "Couldn't create a graphics pipeline!")?
                }
            };

            (
                descriptor_set_layouts,
                descriptor_pool,
                descriptor_set,
                layout,
                gfx_pipeline,
            )
        };
        unsafe {
            device.destroy_shader_module(vertex_shader_module);
            device.destroy_shader_module(fragment_shader_module);
        }

        Ok(Pipeline {
            descriptor_set_layouts,
            descriptor_pool: ManuallyDrop::new(descriptor_pool),
            descriptor_set: ManuallyDrop::new(descriptor_set),
            pipeline_layout: ManuallyDrop::new(pipeline_layout),
            graphics_pipeline: ManuallyDrop::new(gfx_pipeline),
        })
    }
}


pub fn do_the_render(hal: &mut HalState, hardware: &mut hal::hardware::Hardware, local_state: &LocalState) -> Result<(), &'static str> {
    hal.draw_quad_frame(local_state, hardware)
}

fn main() {
    //simple_logger::init().unwrap();


    let mut winit_state = WinitState::default();
    let mut keyboard_state = input::keyboard_state::KeyboardState::default();
    let mut mouse_state = input::mouse_state::MouseState::default();
    let mut hardware = hal::hardware::Hardware::new(&winit_state.window).unwrap();
    let mut hal_state = HalState::new(&winit_state.window, &mut hardware).unwrap();

    let monkey_id = hardware.add_object("monkey.obj").unwrap();
    let teapot_id = hardware.add_object("teapot3.obj").unwrap();
    let mut local_state = LocalState::default();

    local_state.add_object(
        Object::new(monkey_id)
            .with_pos(glm::vec3(0.0f32, 0.0, 0.0))
            .with_rotation(glm::quat_identity())
    );
    local_state.add_object(Object::new(teapot_id).with_pos(glm::vec3(2.0f32, 0.0, 0.0)));

    local_state.camera.position = glm::vec3(0f32, 0.1f32, -3.0f32);
    let mut reinitialize = false;
    let mut prev = Instant::now();
    //return;
    loop {
        if reinitialize {
            reinitialize = false;
            hal_state.drop_stuff(&mut hardware);
            hal_state = match HalState::new(&winit_state.window, &mut hardware) {
                Ok(state) => state,
                Err(e) => panic!(e),
            };
        }
        let inputs = UserInput::poll_events_loop(&mut winit_state.events_loop, &mut keyboard_state, &mut mouse_state);
        if inputs.end_requested {
            break;
        }

        if inputs.new_frame_size.is_some() {
            debug!("Window changed size, restarting HalState");
            reinitialize = true;
        }
        let now = Instant::now();

        let dt = now - prev;
        let duration = dt.as_secs() as f32 + dt.subsec_nanos() as f32 / 1_000_000_000.0;
        prev = now;
        local_state.update(inputs, &keyboard_state, &mouse_state, duration);
        if let Err(e) = do_the_render(&mut hal_state, &mut hardware, &local_state) {
            error!("Rendering error: {:?}", e);
            debug!("trying to restart hal");
            reinitialize = true;
            continue;
        }
    }
    hal_state.drop_stuff(&mut hardware);
}
