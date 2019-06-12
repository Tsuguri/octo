use core::mem::{ManuallyDrop, size_of};
use std::ops::Deref;
use std::time::Instant;

use arrayvec::ArrayVec;
use gfx_hal::{
    adapter::{Adapter, /*MemoryTypeId,*/ PhysicalDevice},
    Backend,
    buffer::{IndexBufferView, Usage as BufferUsage},
    command::{ClearColor, ClearValue, CommandBuffer, MultiShot, Primary},
    device::Device,
    //memory::{Properties, Requirements},
    format::{Aspects, ChannelType, Format, Swizzle},
    Gpu,
    Graphics,
    image::{Extent, Layout, SubresourceRange, Usage, ViewKind},
    IndexType,
    Instance,
    memory::cast_slice,
    pass::{Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, SubpassDesc},
    pool::{CommandPool, CommandPoolCreateFlags},
    pso::{
        PipelineStage,
        Rect, ShaderStageFlags,
    },
    queue::{family::QueueGroup, Submission},
    QueueFamily,
    Surface,
    window::{Backbuffer, Extent2D, FrameSync, PresentMode, Swapchain, SwapchainConfig},
};
use log::info;
use winit::Window;

use buffers::BufferBundle;

use crate::back;
use crate::images::ImageData;
use crate::Quad;
use crate::LocalState;

//use winit::Window;
use nalgebra_glm as glm;

pub mod buffers;

pub struct Object {
    vertices: BufferBundle<back::Backend, back::Device>,
    indices: BufferBundle<back::Backend, back::Device>,
    mat: glm::TMat4<f32>,
}

pub struct HalState {
    objects: Vec<Object>,
    postprocessing_quad: Object,
    descriptor_set_layouts: Vec<<back::Backend as Backend>::DescriptorSetLayout>,
    descriptor_pool: ManuallyDrop<<back::Backend as Backend>::DescriptorPool>,
    descriptor_set: ManuallyDrop<<back::Backend as Backend>::DescriptorSet>,
    pipeline_layout: ManuallyDrop<<back::Backend as Backend>::PipelineLayout>,
    graphics_pipeline: ManuallyDrop<<back::Backend as Backend>::GraphicsPipeline>,
    current_frame: usize,
    frames_in_flight: usize,
    in_flight_fences: Vec<<back::Backend as Backend>::Fence>,
    render_finished_semaphores: Vec<<back::Backend as Backend>::Semaphore>,
    image_available_semaphores: Vec<<back::Backend as Backend>::Semaphore>,
    command_buffers: Vec<CommandBuffer<back::Backend, Graphics, MultiShot, Primary>>,
    command_pool: ManuallyDrop<CommandPool<back::Backend, Graphics>>,
    framebuffers: Vec<<back::Backend as Backend>::Framebuffer>,
    image_views: Vec<(<back::Backend as Backend>::ImageView)>,
    render_pass: ManuallyDrop<<back::Backend as Backend>::RenderPass>,
    render_area: Rect,
    queue_group: QueueGroup<back::Backend, Graphics>,
    swapchain: ManuallyDrop<<back::Backend as Backend>::Swapchain>,
    device: ManuallyDrop<back::Device>,
    adapter: Adapter<back::Backend>,
    _surface: <back::Backend as Backend>::Surface,
    _instance: ManuallyDrop<back::Instance>,
    creation_instant: Instant,
    texture: ImageData<back::Backend, back::Device>,
}

impl HalState {
    fn upload_quad(obj: &mut Object, quad: Quad, device: &back::Device) -> Result<(), &'static str> {
        unsafe {
            let mut data_target =
                device
                    .acquire_mapping_writer(&obj.vertices.memory, 0..obj.vertices.requirements.size)
                    .map_err(|_| "Failed to acquire a memory writer!")?;
            let points = quad.vertex_attributes();
            data_target[..points.len()].copy_from_slice(&points);
            device
                .release_mapping_writer(data_target)
                .map_err(|_| "Couldn't release the mapping writer!")?;
        }
        Result::Ok(())
    }

    pub fn add_object(&mut self, position: glm::TVec3<f32>) -> Result<(), &'static str> {
        let (vertices, indices) = {
            const F32_XY_RGB_UV_QUAD: usize = size_of::<f32>() * (2 + 3) * 4;
            let vertices =
                BufferBundle::new(&self.adapter, &*self.device, F32_XY_RGB_UV_QUAD, BufferUsage::VERTEX)?;

            const U16_QUAD_INDICES: usize = size_of::<u16>() * 2 * 3;
            let indexes =
                BufferBundle::new(&self.adapter, &*self.device, U16_QUAD_INDICES, BufferUsage::INDEX)?;
            (vertices, indexes)
        };
        self.objects.push(Object { vertices, indices , mat: glm::translation(&position)});
        Result::Ok(())
    }

    pub fn draw_quad_frame(&mut self, state: &LocalState) -> Result<(), &'static str> {
        // SETUP FOR THIS FRAME
        let image_available = &self.image_available_semaphores[self.current_frame];
        let render_finished = &self.render_finished_semaphores[self.current_frame];
        // Advance the frame _before_ we start using the `?` operator
        self.current_frame = (self.current_frame + 1) % self.frames_in_flight;

        let (i_u32, i_usize) = unsafe {
            let image_index = self
                .swapchain
                .acquire_image(core::u64::MAX, FrameSync::Semaphore(image_available))
                .map_err(|_| "Couldn't acquire an image from the swapchain!")?;
            (image_index, image_index as usize)
        };

        let flight_fence = &self.in_flight_fences[i_usize];
        unsafe {
            self.device
                .wait_for_fence(flight_fence, core::u64::MAX)
                .map_err(|_| "Failed to wait on the fence!")?;
            self.device
                .reset_fence(flight_fence)
                .map_err(|_| "Couldn't reset the fence!")?;
        }

        unsafe {
            let duration = Instant::now().duration_since(self.creation_instant);
            let time_f32 = duration.as_secs() as f32 + duration.subsec_nanos() as f32 * 1e-9;

            let buffer = &mut self.command_buffers[i_usize];
            const TRIANGLE_CLEAR: [ClearValue; 1] =
                [ClearValue::Color(ClearColor::Float([0.1, 0.2, 0.3, 1.0]))];
            buffer.begin(false);
            {
                let mut encoder = buffer.begin_render_pass_inline(
                    &self.render_pass,
                    &self.framebuffers[i_usize],
                    self.render_area,
                    TRIANGLE_CLEAR.iter(),
                );
                encoder.bind_graphics_pipeline(&self.graphics_pipeline);

                //let buffer_ref: &<back::Backend as Backend>::Buffer = &self.postprocessing_quad.vertices.buffer;
                //let buffers: ArrayVec<[_; 1]> = [(buffer_ref, 0)].into();
                encoder.bind_vertex_buffers(0, &[(&self.postprocessing_quad.vertices.buffer as &<back::Backend as Backend>::Buffer, 0u64)].into());
                encoder.bind_index_buffer(IndexBufferView {
                    buffer: &self.postprocessing_quad.indices.buffer,
                    offset: 0,
                    index_type: IndexType::U16,
                });
                encoder.bind_graphics_descriptor_sets(
                    &self.pipeline_layout,
                    0,
                    Some(self.descriptor_set.deref()),
                    &[],
                );
                /*
                encoder.push_graphics_constants(
                    &self.pipeline_layout,
                    ShaderStageFlags::FRAGMENT,
                    0,
                    &[time_f32.to_bits()],
                );
                */
                encoder.push_graphics_constants(
                    &self.pipeline_layout,
                    ShaderStageFlags::VERTEX,
                    0,
                    cast_slice::<f32, u32>(&state.view.data),
                );
                encoder.push_graphics_constants(
                    &self.pipeline_layout,
                    ShaderStageFlags::VERTEX,
                    16,
                    cast_slice::<f32, u32>(&state.projection.data),
                );
                encoder.draw_indexed(0..6, 0, 0..1);
            }
            buffer.finish();
        }
        let command_buffers = &self.command_buffers[i_usize..=i_usize];
        let wait_semaphores: ArrayVec<[_; 1]> =
            [(image_available, PipelineStage::COLOR_ATTACHMENT_OUTPUT)].into();
        let signal_semaphores: ArrayVec<[_; 1]> = [render_finished].into();
        // yes, you have to write it twice like this. yes, it's silly.
        let present_wait_semaphores: ArrayVec<[_; 1]> = [render_finished].into();
        let submission = Submission {
            command_buffers,
            wait_semaphores,
            signal_semaphores,
        };
        let the_command_queue = &mut self.queue_group.queues[0];
        unsafe {
            the_command_queue.submit(submission, Some(flight_fence));
            self.swapchain
                .present(the_command_queue, i_u32, present_wait_semaphores)
                .map_err(|_| "Failed to present into the swapchain!")
        }
    }

    pub fn draw_clear_frame(&mut self, color: [f32; 4]) -> Result<(), &'static str> {
        // setup to clear
        let image_available = &self.image_available_semaphores[self.current_frame];
        let render_finished = &self.render_finished_semaphores[self.current_frame];

        self.current_frame = (self.current_frame + 1) % self.frames_in_flight;

        let (i_u32, i_usize) = unsafe {
            let image_index = self
                .swapchain
                .acquire_image(core::u64::MAX, FrameSync::Semaphore(image_available))
                .map_err(|_| "Couldn't acquire an image from the swapchain!")?;
            (image_index, image_index as usize)
        };

        let flight_fence = &self.in_flight_fences[i_usize];
        unsafe {
            self.device
                .wait_for_fence(flight_fence, core::u64::MAX)
                .map_err(|_| "Failed to wait on the fence!")?;
            self.device
                .reset_fence(flight_fence)
                .map_err(|_| "Couldn't reset the fence1")?;
        }

        //record commands
        // gdzieś tutaj pójdzie generowanie command bufferów na podstawie danych octo

        unsafe {
            let buffer = &mut self.command_buffers[i_usize];
            let clear_values = [ClearValue::Color(ClearColor::Float(color))];
            buffer.begin(false);
            buffer.begin_render_pass_inline(
                &self.render_pass,
                &self.framebuffers[i_usize],
                self.render_area,
                clear_values.iter(),
            );
            buffer.finish();
        }

        // submit and present
        let command_buffers = &self.command_buffers[i_usize..=i_usize];
        let wait_semaphores: ArrayVec<[_; 1]> =
            [(image_available, PipelineStage::COLOR_ATTACHMENT_OUTPUT)].into();
        let signal_semaphores: ArrayVec<[_; 1]> = [render_finished].into();
        let present_wait_semaphores: ArrayVec<[_; 1]> = [render_finished].into();
        let submission = Submission {
            command_buffers,
            wait_semaphores,
            signal_semaphores,
        };
        let command_queue = &mut self.queue_group.queues[0];
        unsafe {
            command_queue.submit(submission, Some(flight_fence));
            self.swapchain
                .present(command_queue, i_u32, present_wait_semaphores)
                .map_err(|_| "Failed to present into the swapchain!")
        }
    }

    fn initialize_hardware(window: &Window) -> Result<(back::Instance, <back::Backend as Backend>::Surface, Adapter<back::Backend>, back::Device, QueueGroup<back::Backend, Graphics>), &'static str> {
        let instance = back::Instance::create(crate::window::WINDOW_NAME, 1);
        let surface = instance.create_surface(window);

        let adapter = instance
            .enumerate_adapters()
            .into_iter()
            .find(|a| {
                a.queue_families
                    .iter()
                    .any(|qf| qf.supports_graphics() && surface.supports_queue_family(qf))
            })
            .ok_or("Couldn't find a graphical Adapter!")?;
        let (mut device, mut queue_group) = {
            let queue_family = adapter
                .queue_families
                .iter()
                .find(|qf| qf.supports_graphics() && surface.supports_queue_family(qf))
                .ok_or("Couldn't find a QueueFamily with graphics!")?;
            let Gpu { device, mut queues } = unsafe {
                adapter
                    .physical_device
                    .open(&[(&queue_family, &[1.0; 1])])
                    .map_err(|_| "Couldn't open the PhysicalDevice!")?
            };
            let queue_group = queues
                .take::<Graphics>(queue_family.id())
                .ok_or("Couldn't take ownership of the QueueGroup!")?;
            let _ = if queue_group.queues.len() > 0 {
                Ok(())
            } else {
                Err("The QueueGroup did not have any CommandQueues available!")
            }?;
            (device, queue_group)
        };

        Result::Ok((instance, surface, adapter, device, queue_group))
    }
    pub fn new(window: &Window) -> Result<Self, &'static str> {
        println!("init start");
        let (instance, mut surface, adapter, mut device, mut queue_group) = Self::initialize_hardware(window)?;

        let (swapchain, extent, backbuffer, format, frames_in_flight) = {
            let (caps, preferred_formats, present_modes, composite_alphas) =
                surface.compatibility(&adapter.physical_device);
            info!("{:?}", caps);
            info!("Preferred Formats: {:?}", preferred_formats);
            info!("Present Modes: {:?}", present_modes);
            info!("Composite Alphas: {:?}", composite_alphas);
            //
            let present_mode = {
                use gfx_hal::window::PresentMode::*;
                [Mailbox, Fifo, Relaxed, Immediate]
                    .iter()
                    .cloned()
                    .find(|pm| present_modes.contains(pm))
                    .ok_or("No PresentMode values specified!")?
            };
            let composite_alpha = {
                use gfx_hal::window::CompositeAlpha::*;
                [Opaque, Inherit, PreMultiplied, PostMultiplied]
                    .iter()
                    .cloned()
                    .find(|ca| composite_alphas.contains(ca))
                    .ok_or("No CompositeAlpha values specified!")?
            };
            let format = match preferred_formats {
                None => Format::Rgba8Srgb,
                Some(formats) => match formats
                    .iter()
                    .find(|format| format.base_format().1 == ChannelType::Srgb)
                    .cloned()
                    {
                        Some(srgb_format) => srgb_format,
                        None => formats
                            .get(0)
                            .cloned()
                            .ok_or("Preferred format list was empty!")?,
                    },
            };
            let extent = {
                let window_client_area = window
                    .get_inner_size()
                    .ok_or("Window doesn't exist")?
                    .to_physical(window.get_hidpi_factor());
                Extent2D {
                    width: caps.extents.end.width.min(window_client_area.width as u32),
                    height: caps
                        .extents
                        .end
                        .height
                        .min(window_client_area.height as u32),
                }
            };

            let image_count = if present_mode == PresentMode::Mailbox {
                (caps.image_count.end - 1).min(3)
            } else {
                (caps.image_count.end - 1).min(2)
            };
            let image_layers = 1;
            let image_usage = if caps.usage.contains(Usage::COLOR_ATTACHMENT) {
                Usage::COLOR_ATTACHMENT
            } else {
                Err("The Surface isn't capable of supporting color!")?
            };
            let swapchain_config = SwapchainConfig {
                present_mode,
                composite_alpha,
                format,
                extent,
                image_count,
                image_layers,
                image_usage,
            };
            info!("{:?}", swapchain_config);
            //
            let (swapchain, backbuffer) = unsafe {
                device
                    .create_swapchain(&mut surface, swapchain_config, None)
                    .map_err(|_| "Failed to create the swapchain!")?
            };
            (swapchain, extent, backbuffer, format, image_count as usize)
        };

        let (image_available_semaphores, render_finished_semaphores, in_flight_fences) = {
            let mut image_available_semaphores: Vec<<back::Backend as Backend>::Semaphore> = vec![];
            let mut render_finished_semaphores: Vec<<back::Backend as Backend>::Semaphore> = vec![];
            let mut in_flight_fences: Vec<<back::Backend as Backend>::Fence> = vec![];
            for _ in 0..frames_in_flight {
                in_flight_fences.push(
                    device
                        .create_fence(true)
                        .map_err(|_| "Could not create a fence!")?,
                );
                image_available_semaphores.push(
                    device
                        .create_semaphore()
                        .map_err(|_| "Could not create a semaphore!")?,
                );
                render_finished_semaphores.push(
                    device
                        .create_semaphore()
                        .map_err(|_| "Could not create a semaphore!")?,
                );
            }
            (
                image_available_semaphores,
                render_finished_semaphores,
                in_flight_fences,
            )
        };

        let render_pass = {
            let color_attachment = Attachment {
                format: Some(format),
                samples: 1,
                ops: AttachmentOps {
                    load: AttachmentLoadOp::Clear,
                    store: AttachmentStoreOp::Store,
                },
                stencil_ops: AttachmentOps::DONT_CARE,
                layouts: Layout::Undefined..Layout::Present,
            };
            let subpass = SubpassDesc {
                colors: &[(0, Layout::ColorAttachmentOptimal)],
                depth_stencil: None,
                inputs: &[],
                resolves: &[],
                preserves: &[],
            };
            unsafe {
                device
                    .create_render_pass(&[color_attachment], &[subpass], &[])
                    .map_err(|_| "Couldn't create a render pass!")?
            }
        };
        let image_views: Vec<_> = match backbuffer {
            Backbuffer::Images(images) => images
                .into_iter()
                .map(|image| unsafe {
                    device
                        .create_image_view(
                            &image,
                            ViewKind::D2,
                            format,
                            Swizzle::NO,
                            SubresourceRange {
                                aspects: Aspects::COLOR,
                                levels: 0..1,
                                layers: 0..1,
                            },
                        )
                        .map_err(|_| "Couldn't create the image_view for the image!")
                })
                .collect::<Result<Vec<_>, &str>>()?,
            Backbuffer::Framebuffer(_) => unimplemented!("Can't handle framebuffer backbuffer!"),
        };

        let framebuffers: Vec<<back::Backend as Backend>::Framebuffer> = {
            image_views
                .iter()
                .map(|image_view| unsafe {
                    device
                        .create_framebuffer(
                            &render_pass,
                            vec![image_view],
                            Extent {
                                width: extent.width as u32,
                                height: extent.height as u32,
                                depth: 1,
                            },
                        )
                        .map_err(|_| "Failed to create a framebuffer!")
                })
                .collect::<Result<Vec<_>, &str>>()?
        };
        let mut command_pool = unsafe {
            device
                .create_command_pool_typed(&queue_group, CommandPoolCreateFlags::RESET_INDIVIDUAL)
                .map_err(|_| "Could not create the raw command pool!")?
        };
        let command_buffers: Vec<_> = framebuffers
            .iter()
            .map(|_| command_pool.acquire_command_buffer())
            .collect();

        let (
            descriptor_set_layouts,
            descriptor_pool,
            descriptor_set,
            pipeline_layout,
            graphics_pipeline,
        ) = Self::create_pipeline(&mut device, extent, &render_pass)?;

        let (vertices, indices) = {
            const F32_XY_RGB_UV_QUAD: usize = size_of::<f32>() * (2 + 3 + 2) * 4;
            let vertices =
                BufferBundle::new(&adapter, &device, F32_XY_RGB_UV_QUAD, BufferUsage::VERTEX)?;

            const U16_QUAD_INDICES: usize = size_of::<u16>() * 2 * 3;
            let indexes =
                BufferBundle::new(&adapter, &device, U16_QUAD_INDICES, BufferUsage::INDEX)?;
            (vertices, indexes)
        };
        let mut postprocessing_quad = Object { vertices, indices };
        let quad = Quad {
            x: -1.0,
            y: -1.0,
            w: 2.0,
            h: 2.0,
        };
        Self::upload_quad(&mut postprocessing_quad, quad, &device)?;

        unsafe {
            let mut data_target = device
                .acquire_mapping_writer(&postprocessing_quad.indices.memory, 0..postprocessing_quad.indices.requirements.size)
                .map_err(|_| "Failed to acquire and index buffer mapping writer!")?;
            const INDEX_DATA: &[u16] = &[0, 1, 2, 2, 3, 0];
            data_target[..INDEX_DATA.len()].copy_from_slice(&INDEX_DATA);
            device
                .release_mapping_writer(data_target)
                .map_err(|_| "Couldn't release the index buffer mapping writer!")?
        }

        let texture = ImageData::new(
            &adapter,
            &device,
            &mut command_pool,
            &mut queue_group.queues[0],
            image::load_from_memory(crate::CREATURE_BYTES)
                .expect("binary corrupted")
                .to_rgba(),
        )?;

        unsafe {
            device.write_descriptor_sets(vec![
                gfx_hal::pso::DescriptorSetWrite {
                    set: &descriptor_set,
                    binding: 0,
                    array_offset: 0,
                    descriptors: Some(gfx_hal::pso::Descriptor::Image(
                        texture.image_view.deref(),
                        Layout::Undefined,
                    )),
                },
                gfx_hal::pso::DescriptorSetWrite {
                    set: &descriptor_set,
                    binding: 1,
                    array_offset: 0,
                    descriptors: Some(gfx_hal::pso::Descriptor::Sampler(
                        texture.sampler.samp.deref(),
                    )),
                },
            ]);
        }

        println!("init end");

        Ok(Self {
            objects: vec![],
            postprocessing_quad,
            _instance: ManuallyDrop::new(instance),
            _surface: surface,
            adapter,
            device: ManuallyDrop::new(device),
            queue_group,
            swapchain: ManuallyDrop::new(swapchain),
            render_area: extent.to_extent().rect(),
            render_pass: ManuallyDrop::new(render_pass),
            image_views,
            framebuffers,
            command_pool: ManuallyDrop::new(command_pool),
            command_buffers,
            image_available_semaphores,
            render_finished_semaphores,
            descriptor_pool: ManuallyDrop::new(descriptor_pool),
            descriptor_set: ManuallyDrop::new(descriptor_set),
            in_flight_fences,
            frames_in_flight,
            current_frame: 0,
            descriptor_set_layouts,
            pipeline_layout: ManuallyDrop::new(pipeline_layout),
            graphics_pipeline: ManuallyDrop::new(graphics_pipeline),
            creation_instant: Instant::now(),
            texture,
        })
    }
}

impl core::ops::Drop for HalState {
    fn drop(&mut self) {
        let _ = self.device.wait_idle();
        unsafe {
            for descriptor_set_layout in self.descriptor_set_layouts.drain(..) {
                self.device
                    .destroy_descriptor_set_layout(descriptor_set_layout)
            }
            for fence in self.in_flight_fences.drain(..) {
                self.device.destroy_fence(fence)
            }
            for semaphore in self.render_finished_semaphores.drain(..) {
                self.device.destroy_semaphore(semaphore)
            }
            for semaphore in self.image_available_semaphores.drain(..) {
                self.device.destroy_semaphore(semaphore)
            }
            for framebuffer in self.framebuffers.drain(..) {
                self.device.destroy_framebuffer(framebuffer);
            }
            for image_view in self.image_views.drain(..) {
                self.device.destroy_image_view(image_view);
            }
            // LAST RESORT STYLE CODE, NOT TO BE IMITATED LIGHTLY
            use core::ptr::read;

            self.postprocessing_quad.vertices.manually_drop(&self.device);
            self.postprocessing_quad.indices.manually_drop(&self.device);
            for obj in &self.objects {
                obj.vertices.manually_drop(&self.device);
                obj.indices.manually_drop(&self.device);
            }
            self.texture.manually_drop(&self.device);
            self.device
                .destroy_descriptor_pool(ManuallyDrop::into_inner(read(&self.descriptor_pool)));
            self.device
                .destroy_pipeline_layout(ManuallyDrop::into_inner(read(&self.pipeline_layout)));
            self.device
                .destroy_graphics_pipeline(ManuallyDrop::into_inner(read(&self.graphics_pipeline)));
            self.device.destroy_command_pool(
                ManuallyDrop::into_inner(read(&self.command_pool)).into_raw(),
            );
            self.device
                .destroy_render_pass(ManuallyDrop::into_inner(read(&self.render_pass)));
            self.device
                .destroy_swapchain(ManuallyDrop::into_inner(read(&self.swapchain)));
            ManuallyDrop::drop(&mut self.device);
            ManuallyDrop::drop(&mut self._instance);
        }
    }
}
