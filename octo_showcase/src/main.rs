use core::mem::size_of;

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

use hal::HalState;
use input::UserInput;
use octo_runtime::OctoModule;
use state::LocalState;
use window::WinitState;

mod images;
mod window;
mod input;
mod state;
mod hal;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub points: [[f32; 2]; 3],
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
    pub fn vertex_attributes(self) -> [f32; 4 * (2 + 3 + 2)] {
        let x = self.x;
        let y = self.y;
        let w = self.w;
        let h = self.h;

        [
            x,
            y + h,
            1.0,
            0.0,
            0.0,
            0.0,
            1.0,
            x,
            y,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            x + w,
            y,
            0.0,
            0.0,
            1.0,
            1.0,
            0.0,
            x + w,
            y + h,
            1.0,
            0.0,
            1.0,
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

impl HalState {
    fn create_pipeline(
        device: &mut back::Device,
        extent: Extent2D,
        render_pass: &<back::Backend as Backend>::RenderPass,
    ) -> Result<
        (
            Vec<<back::Backend as Backend>::DescriptorSetLayout>,
            <back::Backend as Backend>::DescriptorPool,
            <back::Backend as Backend>::DescriptorSet,
            <back::Backend as Backend>::PipelineLayout,
            <back::Backend as Backend>::GraphicsPipeline,
        ),
        &'static str,
    > {
        let f = include_str!("file.octo_bin");

        let module: OctoModule = serde_json::from_str(&f).unwrap();
        drop(f);
        println!("Loading octo module: {}", module.name);

        let vertex_shader_module = unsafe {
            device
                .create_shader_module(&module.basic_vertex_spirv)
                .map_err(|_| "Couldn't make the vertex module")?
        };
        let fragment_shader_module = unsafe {
            device
                .create_shader_module(&module.fragment_shaders["firstGPU"].1)
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
                stride: (size_of::<f32>() * (2 + 3 + 2)) as ElemStride,
                rate: 0,
            }];
            let attributes: Vec<AttributeDesc> = vec![
                AttributeDesc {
                    location: 0,
                    binding: 0,
                    element: Element {
                        format: Format::Rg32Float,
                        offset: 0,
                    },
                },
                AttributeDesc {
                    location: 1,
                    binding: 0,
                    element: Element {
                        format: Format::Rgb32Float,
                        offset: (size_of::<f32>() * 2) as ElemOffset,
                    },
                },
                AttributeDesc {
                    location: 2,
                    binding: 0,
                    element: Element {
                        format: Format::Rg32Float,
                        offset: (size_of::<f32>() * 5) as ElemOffset,
                    },
                },
            ];

            let rasterizer = Rasterizer {
                depth_clamping: false,
                polygon_mode: PolygonMode::Fill,
                cull_face: Face::NONE,
                front_face: FrontFace::Clockwise,
                depth_bias: None,
                conservative: false,
            };

            let depth_stencil = DepthStencilDesc {
                depth: DepthTest::Off,
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
            let immutable_samplers = Vec::<<back::Backend as Backend>::Sampler>::new();
            let descriptor_set_layouts: Vec<<back::Backend as Backend>::DescriptorSetLayout> =
                vec![unsafe {
                    device
                        .create_descriptor_set_layout(bindings, immutable_samplers)
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

            let push_constants = vec![(ShaderStageFlags::FRAGMENT, 0..1)];
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

        Ok((
            descriptor_set_layouts,
            descriptor_pool,
            descriptor_set,
            pipeline_layout,
            gfx_pipeline,
        ))
    }
}


pub fn do_the_render(hal: &mut HalState, local_state: &LocalState) -> Result<(), &'static str> {
    let quad = Quad {
        x: -0.5,
        y: -0.5,
        w: 1.0,
        h: 1.0,
    };
    hal.draw_quad_frame(quad)
}

fn main() {
    simple_logger::init().unwrap();


    let mut winit_state = WinitState::default();
    let mut hal_state = HalState::new(&winit_state.window).unwrap();
    let mut local_state = LocalState::default();

    loop {
        let inputs = UserInput::poll_events_loop(&mut winit_state.events_loop);
        if inputs.end_requested {
            break;
        }

        if inputs.new_frame_size.is_some() {
            debug!("Window changed size, restarting HalState");
            drop(hal_state);
            hal_state = match HalState::new(&winit_state.window) {
                Ok(state) => state,
                Err(e) => panic!(e),
            };
        }
        local_state.update_from_input(inputs);
        if let Err(e) = do_the_render(&mut hal_state, &local_state) {
            error!("Rendering error: {:?}", e);
            debug!("trying to restart hal");
            drop(hal_state);
            hal_state = match HalState::new(&winit_state.window) {
                Ok(state) => state,
                Err(_) => panic!(e),
            };
        }
    }
}
