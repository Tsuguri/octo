use backend as back;
use glsl_to_spirv;
use hal;
use image;
use winit;

use hal::format::{AsFormat, ChannelType, Rgba8Srgb as ColorFormat, Swizzle};
use hal::pass::Subpass;
use hal::pso::{PipelineStage, ShaderStageFlags, Specialization};
use hal::queue::Submission;
use hal::Backend;
use hal::{buffer, command, format as f, image as i, memory as m, pass, pool, pso, window::Extent2D};
use hal::{Backbuffer, DescriptorPool, FrameSync, Primitive, SwapchainConfig};
use hal::{Device, PhysicalDevice, Surface, Swapchain};

use data_loading::load_image;
use generated_stub::*;
use types::Vertex;
use window::*;

use std;
use std::io::{Read, Write};

const DIMS: Extent2D = Extent2D {
    width: 1024,
    height: 768,
};

const ENTRY_NAME: &str = "main";

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

const COLOR_RANGE: i::SubresourceRange = i::SubresourceRange {
    aspects: f::Aspects::COLOR,
    levels: 0..1,
    layers: 0..1,
};

#[cfg(any(feature = "vulkan", feature = "dx12", feature = "gl"))]
pub fn main_function(events_loop: &mut winit::EventsLoop) {
    let wb = window_builder("quad".to_owned(), DIMS.width.into(), DIMS.height.into());
    // instantiate backend
    let (window, _instance, mut adapters, mut surface) = build_window("gfx", wb, events_loop);

    for adapter in &adapters {
        println!("{:#?}", adapter.info);
    }

    let mut adapter = adapters.remove(0);
    drop(adapters);
    let memory_types = adapter.physical_device.memory_properties().memory_types;
    let limits = adapter.physical_device.limits();

    // Build a new device and associated command queues
    let (mut device, mut queue_group) = adapter
        .open_with::<_, hal::Graphics>(1, |family| surface.supports_queue_family(family)) // checking if device provides swapchains
        .unwrap();

    let mut command_pool =
        device.create_command_pool_typed(&queue_group, pool::CommandPoolCreateFlags::empty(), 16);

    // Setup renderpass and pipeline
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

    // Descriptors
    let mut desc_pool = device.create_descriptor_pool(
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
    let desc_set = desc_pool.allocate_set(&set_layout).unwrap();

    // Buffer allocations

    let buffer_stride = std::mem::size_of::<Vertex>() as u64;
    let buffer_len = QUAD.len() as u64 * buffer_stride;

    let buffer_unbound = device
        .create_buffer(buffer_len, buffer::Usage::VERTEX)
        .unwrap();
    let buffer_req = device.get_buffer_requirements(&buffer_unbound);
    println!("Memory types: {:#?}", memory_types);

    let upload_type = memory_types
        .iter()
        .enumerate()
        .position(|(id, mem_type)| {
            buffer_req.type_mask & (1 << id) != 0
                && mem_type.properties.contains(m::Properties::CPU_VISIBLE)
        })
        .unwrap()
        .into();

    let buffer_memory = device
        .allocate_memory(upload_type, buffer_req.size)
        .unwrap();
    let vertex_buffer = device
        .bind_buffer_memory(&buffer_memory, 0, buffer_unbound)
        .unwrap();

    // TODO: check transitions: read/write mapping and vertex buffer read
    {
        let mut vertices = device
            .acquire_mapping_writer::<Vertex>(&buffer_memory, 0..buffer_len)
            .unwrap();
        vertices.copy_from_slice(&QUAD);
        device.release_mapping_writer(vertices);
    }

    // Image
    let img_data = include_bytes!("data/logo.png");

    let image_data = load_image(img_data, &device, &limits, upload_type, image::PNG);

    let image_unbound = device
        .create_image(
            image_data.kind,
            1,
            ColorFormat::SELF,
            i::Tiling::Optimal,
            i::Usage::TRANSFER_DST | i::Usage::SAMPLED,
            i::StorageFlags::empty(),
        )
        .unwrap(); // TODO: usage
    let image_req = device.get_image_requirements(&image_unbound);

    let device_type = memory_types
        .iter()
        .enumerate()
        .position(|(id, memory_type)| {
            image_req.type_mask & (1 << id) != 0
                && memory_type.properties.contains(m::Properties::DEVICE_LOCAL)
        })
        .unwrap()
        .into();
    let image_memory = device.allocate_memory(device_type, image_req.size).unwrap();

    let image_logo = device
        .bind_image_memory(&image_memory, 0, image_unbound)
        .unwrap();

    let image_srv = device
        .create_image_view(
            &image_logo,
            i::ViewKind::D2,
            ColorFormat::SELF,
            Swizzle::NO,
            COLOR_RANGE.clone(),
        )
        .unwrap();

    let sampler = device.create_sampler(i::SamplerInfo::new(i::Filter::Linear, i::WrapMode::Clamp));

    device.write_descriptor_sets(vec![
        pso::DescriptorSetWrite {
            set: &desc_set,
            binding: 0,
            array_offset: 0,
            descriptors: Some(pso::Descriptor::Image(&image_srv, i::Layout::Undefined)),
        },
        pso::DescriptorSetWrite {
            set: &desc_set,
            binding: 1,
            array_offset: 0,
            descriptors: Some(pso::Descriptor::Sampler(&sampler)),
        },
    ]);

    let mut frame_semaphore = device.create_semaphore();
    let mut frame_fence = device.create_fence(false); // TODO: remove

    // copy buffer to texture
    {
        let submit = {
            let mut cmd_buffer = command_pool.acquire_command_buffer(false);

            let image_barrier = m::Barrier::Image {
                states: (i::Access::empty(), i::Layout::Undefined)
                    ..(i::Access::TRANSFER_WRITE, i::Layout::TransferDstOptimal),
                target: &image_logo,
                range: COLOR_RANGE.clone(),
            };

            cmd_buffer.pipeline_barrier(
                PipelineStage::TOP_OF_PIPE..PipelineStage::TRANSFER,
                m::Dependencies::empty(),
                &[image_barrier],
            );

            image_data.copy_to_image(&mut cmd_buffer, &image_logo);

            let image_barrier = m::Barrier::Image {
                states: (i::Access::TRANSFER_WRITE, i::Layout::TransferDstOptimal)
                    ..(i::Access::SHADER_READ, i::Layout::ShaderReadOnlyOptimal),
                target: &image_logo,
                range: COLOR_RANGE.clone(),
            };
            cmd_buffer.pipeline_barrier(
                PipelineStage::TRANSFER..PipelineStage::FRAGMENT_SHADER,
                m::Dependencies::empty(),
                &[image_barrier],
            );

            cmd_buffer.finish()
        };

        let submission = Submission::new().submit(Some(submit));
        queue_group.queues[0].submit(submission, Some(&mut frame_fence));

        device.wait_for_fence(&frame_fence, !0);
    }
    //////////////////////////////
    image_data.destroy(&device);

    // shaders

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
    let (caps, formats, _present_modes) = surface.compatibility(&adapter.physical_device);

    println!("formats: {:?}", formats);
    let format = formats.map_or(f::Format::Rgba8Srgb, |formats| {
        formats
            .iter()
            .find(|format| format.base_format().1 == ChannelType::Srgb)
            .map(|format| *format)
            .unwrap_or(formats[0])
    });
    let octo_pipeline = Pipeline::<back::Backend>::new(&device, &memory_types, format);

    let render_pass = {
        let attachment = pass::Attachment {
            format: Some(format),
            samples: 1,
            ops: pass::AttachmentOps::new(
                pass::AttachmentLoadOp::Clear,
                pass::AttachmentStoreOp::Store,
            ),
            stencil_ops: pass::AttachmentOps::DONT_CARE,
            layouts: i::Layout::Undefined..i::Layout::Present,
        };

        let subpass = pass::SubpassDesc {
            colors: &[(0, i::Layout::ColorAttachmentOptimal)],
            depth_stencil: None,
            inputs: &[],
            resolves: &[],
            preserves: &[],
        };

        let dependency = pass::SubpassDependency {
            passes: pass::SubpassRef::External..pass::SubpassRef::Pass(0),
            stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT..PipelineStage::COLOR_ATTACHMENT_OUTPUT,
            accesses: i::Access::empty()
                ..(i::Access::COLOR_ATTACHMENT_READ | i::Access::COLOR_ATTACHMENT_WRITE),
        };

        device.create_render_pass(&[attachment], &[subpass], &[dependency])
    };
    let pipeline_layout =
        device.create_pipeline_layout(Some(set_layout), &[(pso::ShaderStageFlags::VERTEX, 0..8)]);
    let pipeline = {
        let pipeline = {
            let subpass = Subpass {
                index: 0,
                main_pass: &render_pass,
            };
            let (vs_entry, fs_entry) = (
                pso::EntryPoint::<back::Backend> {
                    entry: ENTRY_NAME,
                    module: &vs_module,
                    specialization: &[Specialization {
                        id: 0,
                        value: pso::Constant::F32(0.8),
                    }],
                },
                pso::EntryPoint::<back::Backend> {
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
                Primitive::TriangleList,
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
                    format: f::Format::Rg32Float,
                    offset: 0,
                },
            });
            pipeline_desc.attributes.push(pso::AttributeDesc {
                location: 1,
                binding: 0,
                element: pso::Element {
                    format: f::Format::Rg32Float,
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

    let mut swap_chain;
    let mut framebuffers;
    let mut frame_images;
    let mut extent: Extent2D;

    {
        let window = &window;
        let (new_swap_chain, new_framebuffers, new_frame_images, new_extent) = swapchain_stuff(
            &mut surface,
            &mut device,
            window,
            &render_pass,
            &caps,
            format,
        );

        swap_chain = new_swap_chain;
        framebuffers = new_framebuffers;
        frame_images = new_frame_images;
        extent = new_extent;
    }

    // Rendering setup
    let mut viewport = pso::Viewport {
        rect: pso::Rect {
            x: 0,
            y: 0,
            w: extent.width as _,
            h: extent.height as _,
        },
        depth: 0.0..1.0,
    };

    //
    let mut running = true;
    let mut recreate_swapchain = false;
    let mut frames = 0u32;
    let mut now = std::time::Instant::now();
    println!("");
    while running {
        events_loop.poll_events(|event| {
            if let winit::Event::WindowEvent { event, .. } = event {
                #[allow(unused_variables)]
                match event {
                    winit::WindowEvent::KeyboardInput {
                        input:
                            winit::KeyboardInput {
                                virtual_keycode: Some(winit::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    }
                    | winit::WindowEvent::CloseRequested => running = false,
                    winit::WindowEvent::Resized(dims) => {
                        // Couldn't compile due to missing trait impl use.
                        // #[cfg(feature = "gl")]
                        // surface
                        //     .get_window()
                        //     .resize(dims.to_physical(surface.get_window().get_hidpi_factor()));
                        recreate_swapchain = true;
                    }
                    _ => (),
                }
            }
        });

        if recreate_swapchain {
            device.wait_idle().unwrap();

            command_pool.reset();

            for framebuffer in framebuffers {
                device.destroy_framebuffer(framebuffer);
            }

            for (_, rtv) in frame_images {
                device.destroy_image_view(rtv);
            }
            device.destroy_swapchain(swap_chain);

            #[cfg(not(feature = "gl"))]
            let window = &window;
            #[cfg(feature = "gl")]
            let window = &0;
            let (new_swap_chain, new_framebuffers, new_frame_images, new_extent) = swapchain_stuff(
                &mut surface,
                &mut device,
                window,
                &render_pass,
                &caps,
                format,
            );
            swap_chain = new_swap_chain;
            framebuffers = new_framebuffers;
            frame_images = new_frame_images;
            extent = new_extent;
            viewport.rect.w = extent.width as _;
            viewport.rect.h = extent.height as _;
            recreate_swapchain = false;
        }

        device.reset_fence(&frame_fence);
        command_pool.reset();
        let frame: hal::SwapImageIndex = {
            match swap_chain.acquire_image(FrameSync::Semaphore(&mut frame_semaphore)) {
                Ok(i) => i,
                Err(_) => {
                    recreate_swapchain = true;
                    continue;
                }
            }
        };

        // Rendering
        let submit = {
            let mut cmd_buffer = command_pool.acquire_command_buffer(false);

            cmd_buffer.set_viewports(0, &[viewport.clone()]);
            cmd_buffer.set_scissors(0, &[viewport.rect]);
            cmd_buffer.bind_graphics_pipeline(&pipeline);
            cmd_buffer.bind_vertex_buffers(0, Some((&vertex_buffer, 0)));
            cmd_buffer.bind_graphics_descriptor_sets(&pipeline_layout, 0, Some(&desc_set), &[]); //TODO

            {
                let mut encoder = cmd_buffer.begin_render_pass_inline(
                    &render_pass,
                    &framebuffers[frame as usize],
                    viewport.rect,
                    &[command::ClearValue::Color(command::ClearColor::Float([
                        0.8, 0.8, 0.8, 1.0,
                    ]))],
                );
                encoder.draw(0..6, 0..1);
            }

            octo_pipeline.fill_command_buffer(
                &mut cmd_buffer,
                &framebuffers[frame as usize],
                hal::pso::Rect {
                    x: 0,
                    y: viewport.rect.h / 2,
                    w: viewport.rect.w,
                    h: viewport.rect.h / 2,
                },
            );

            cmd_buffer.finish()
        };

        let submission = Submission::new()
            .wait_on(&[(&frame_semaphore, PipelineStage::BOTTOM_OF_PIPE)])
            .submit(Some(submit));
        queue_group.queues[0].submit(submission, Some(&mut frame_fence));

        // TODO: replace with semaphore
        device.wait_for_fence(&frame_fence, !0);

        // present frame
        if let Err(_) = swap_chain.present(&mut queue_group.queues[0], frame, &[]) {
            recreate_swapchain = true;
        }
        frames += 1;
        if now.elapsed().as_secs() >= 1 {
            print!("\rframes in last sec: {}", frames);
            std::io::stdout().flush();
            frames = 0;
            now = std::time::Instant::now();
        }
    }

    // cleanup!
    device.destroy_command_pool(command_pool.into_raw());
    device.destroy_descriptor_pool(desc_pool);

    device.destroy_buffer(vertex_buffer);
    device.destroy_image(image_logo);
    device.destroy_image_view(image_srv);
    device.destroy_sampler(sampler);
    device.destroy_fence(frame_fence);
    device.destroy_semaphore(frame_semaphore);
    device.destroy_render_pass(render_pass);
    device.free_memory(buffer_memory);
    device.free_memory(image_memory);
    device.destroy_graphics_pipeline(pipeline);
    device.destroy_pipeline_layout(pipeline_layout);
    for framebuffer in framebuffers {
        device.destroy_framebuffer(framebuffer);
    }
    for (_, rtv) in frame_images {
        device.destroy_image_view(rtv);
    }

    device.destroy_swapchain(swap_chain);
}

#[cfg(not(any(feature = "vulkan", feature = "dx12", feature = "gl")))]
fn main_function() {
    println!("You need to enable the native API feature (vulkan/dx12) in order to test the LL");
}

#[cfg(any(feature = "vulkan", feature = "dx12", feature = "gl"))]
fn swapchain_stuff(
    surface: &mut <back::Backend as hal::Backend>::Surface,
    device: &mut back::Device,
    window: &WindowType,
    render_pass: &<back::Backend as hal::Backend>::RenderPass,
    caps: &hal::SurfaceCapabilities,
    format: hal::format::Format,
) -> (
    <back::Backend as hal::Backend>::Swapchain,
    std::vec::Vec<<back::Backend as hal::Backend>::Framebuffer>,
    std::vec::Vec<(
        <back::Backend as hal::Backend>::Image,
        <back::Backend as hal::Backend>::ImageView,
    )>,
    Extent2D,
) {
    let extent = match caps.current_extent {
        Some(e) => e,
        None => {
            #[cfg(feature = "gl")]
            let _window = window;
            #[cfg(feature = "gl")]
            let window = surface.get_window();

            let window_size = window
                .get_inner_size()
                .unwrap()
                .to_physical(window.get_hidpi_factor());
            let mut extent = hal::window::Extent2D {
                width: window_size.width as _,
                height: window_size.height as _,
            };

            extent.width = extent
                .width
                .max(caps.extents.start.width)
                .min(caps.extents.end.width);
            extent.height = extent
                .height
                .max(caps.extents.start.height)
                .min(caps.extents.end.height);

            extent
        }
    };

    println!("Surface format: {:?}", format);
    let swap_config = SwapchainConfig::new()
        .with_color(format)
        .with_image_count(caps.image_count.start)
        .with_image_usage(i::Usage::COLOR_ATTACHMENT);
    let (swap_chain, backbuffer) = device.create_swapchain(surface, swap_config, None, &extent);

    let (frame_images, framebuffers) = match backbuffer {
        Backbuffer::Images(images) => {
            let extent = i::Extent {
                width: extent.width as _,
                height: extent.height as _,
                depth: 1,
            };
            let pairs = images
                .into_iter()
                .map(|image| {
                    let rtv = device
                        .create_image_view(
                            &image,
                            i::ViewKind::D2,
                            format,
                            Swizzle::NO,
                            COLOR_RANGE.clone(),
                        )
                        .unwrap();
                    (image, rtv)
                })
                .collect::<Vec<_>>();
            let fbos = pairs
                .iter()
                .map(|&(_, ref rtv)| {
                    device
                        .create_framebuffer(render_pass, Some(rtv), extent)
                        .unwrap()
                })
                .collect();
            (pairs, fbos)
        }
        Backbuffer::Framebuffer(fbo) => (Vec::new(), vec![fbo]),
    };

    (swap_chain, framebuffers, frame_images, extent)
}
