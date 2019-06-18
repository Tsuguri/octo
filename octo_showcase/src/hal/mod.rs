pub mod prelude;

use core::mem::{ManuallyDrop};
use std::ops::Deref;

use arrayvec::{ArrayVec};
use gfx_hal::{
    Backend,
    buffer::{IndexBufferView},
    command::{ClearColor, ClearValue},
    device::Device,
    format::{ChannelType, Format},
    image::{Layout, Usage},
    IndexType,
    memory::cast_slice,
    pass::{Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, SubpassDesc},
    pool::{CommandPoolCreateFlags},
    pso::{
        PipelineStage,
        Rect, ShaderStageFlags,
    },
    queue::{Submission},
    Surface,
    window::{Backbuffer, Extent2D, FrameSync, PresentMode, Swapchain, SwapchainConfig},
};
use log::info;
use winit::Window;


use crate::back;
use crate::LocalState;

use gfx_hal::pass::{SubpassDependency, SubpassRef};
use gfx_hal::image::Access as ImageAccess;

pub mod buffers;
pub mod hardware;
pub mod pipeline;
pub mod framebuffer_stuff;
pub mod renderframe_stuff;
pub mod images;


use images::ImageData;
use framebuffer_stuff::FramebufferStuff;
use renderframe_stuff::RenderFrameStuff;

pub struct HalState {
    pipeline: pipeline::Pipeline,
    render_stuff: Vec<RenderFrameStuff>,
    framebuffer_stuff: Vec<FramebufferStuff>,

    current_frame: usize,
    command_pool: ManuallyDrop<prelude::CommandPool>,
    render_pass: ManuallyDrop<prelude::RenderPass>,
    render_area: Rect,
    swapchain: ManuallyDrop<<back::Backend as Backend>::Swapchain>,
    texture: ImageData<back::Backend, back::Device>,
}

impl HalState {
    fn vertex(obj: &hardware::Object) -> ArrayVec<[(&<back::Backend as Backend>::Buffer, u64); 1]> {
        let buffer_ref: &<back::Backend as Backend>::Buffer = &obj.vertices.buffer;
        let buffers: ArrayVec<[_; 1]> = [(buffer_ref, 0)].into();
        buffers
    }

    pub fn draw_quad_frame(&mut self, state: &LocalState, hardware: &mut hardware::Hardware) -> Result<(), &'static str> {
        // SETUP FOR THIS FRAME
        let render_stuff = &self.render_stuff[self.current_frame];
        // Advance the frame _before_ we start using the `?` operator
        self.current_frame = (self.current_frame + 1) % self.render_stuff.len();

        let (i_u32, i_usize) = unsafe {
            let image_index = self
                .swapchain
                .acquire_image(core::u64::MAX, FrameSync::Semaphore(&render_stuff.image_available))
                .map_err(|_| "Couldn't acquire an image from the swapchain!")?;
            (image_index, image_index as usize)
        };
        let fb_stuff = &mut self.framebuffer_stuff[i_usize];

        let flight_fence = &render_stuff.in_flight;
        unsafe {
            hardware.device
                .wait_for_fence(flight_fence, core::u64::MAX)
                .map_err(|_| "Failed to wait on the fence!")?;
            hardware.device
                .reset_fence(flight_fence)
                .map_err(|_| "Couldn't reset the fence!")?;
        }

        unsafe {
            let projection = state.camera.make_projection_matrix();
            let view = state.camera.make_view_matrix();

            let buffer = &mut fb_stuff.command_buffer;
            const TRIANGLE_CLEAR: [ClearValue; 2] =
                [
                    ClearValue::Color(ClearColor::Float([0.1, 0.2, 0.3, 1.0])),
                    ClearValue::DepthStencil(gfx_hal::command::ClearDepthStencil(1.0, 0))];
            buffer.begin(false);
            {
                let mut encoder = buffer.begin_render_pass_inline(
                    &self.render_pass,
                    &fb_stuff.framebuffer,
                    self.render_area,
                    TRIANGLE_CLEAR.iter(),
                );
                encoder.bind_graphics_pipeline(&self.pipeline.graphics_pipeline);

                self.pipeline.write_descriptor_sets(&hardware, &self.texture);
                encoder.bind_graphics_descriptor_sets(
                    &self.pipeline.pipeline_layout,
                    0,
                    Some(self.pipeline.descriptor_set.deref()),
                    &[],
                );
                encoder.push_graphics_constants(
                    &self.pipeline.pipeline_layout,
                    ShaderStageFlags::VERTEX,
                    0,
                    cast_slice::<f32, u32>(&view.data),
                );
                encoder.push_graphics_constants(
                    &self.pipeline.pipeline_layout,
                    ShaderStageFlags::VERTEX,
                    16,
                    cast_slice::<f32, u32>(&projection.data),
                );

                for obj in &hardware.objects {
                    encoder.push_graphics_constants(
                        &self.pipeline.pipeline_layout,
                        ShaderStageFlags::VERTEX,
                        32,
                        cast_slice::<f32, u32>(&obj.mat.data),
                    );

                    let buffers = Self::vertex(obj);
                    encoder.bind_vertex_buffers(0, buffers);
                    encoder.bind_index_buffer(IndexBufferView {
                        buffer: &obj.indices.buffer,
                        offset: 0,
                        index_type: IndexType::U16,
                    });

                    encoder.draw_indexed(0..obj.indices_len, 0, 0..1);
                }
            }
            buffer.finish();
        }
        let command_buffers: ArrayVec<_> = [&fb_stuff.command_buffer].into();
        let wait_semaphores: ArrayVec<[_; 1]> =
            [(&render_stuff.image_available, PipelineStage::COLOR_ATTACHMENT_OUTPUT)].into();
        let signal_semaphores: ArrayVec<[_; 1]> = [&render_stuff.render_finished].into();
        // yes, you have to write it twice like this. yes, it's silly.
        let present_wait_semaphores: ArrayVec<[_; 1]> = [&render_stuff.render_finished].into();
        let submission = Submission {
            command_buffers,
            wait_semaphores,
            signal_semaphores,
        };
        let the_command_queue = &mut hardware.queue_group.queues[0];
        unsafe {
            the_command_queue.submit(submission, Some(flight_fence));
            self.swapchain
                .present(the_command_queue, i_u32, present_wait_semaphores)
                .map_err(|_| "Failed to present into the swapchain!")
        }
    }

    pub fn new(window: &Window, hardware: &mut hardware::Hardware) -> Result<Self, &'static str> {
        println!("init start");

        let (swapchain, extent, backbuffer, format, frames_in_flight) = {
            let (caps, preferred_formats, present_modes, composite_alphas) =
                hardware.surface.compatibility(&hardware.adapter.physical_device);
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
                hardware.device
                    .create_swapchain(&mut hardware.surface, swapchain_config, None)
                    .map_err(|_| "Failed to create the swapchain!")?
            };
            (swapchain, extent, backbuffer, format, image_count as usize)
        };

        let render_stuff = {
            let mut render_stuff: Vec<RenderFrameStuff> = vec![];
            for _ in 0..frames_in_flight {
                render_stuff.push(RenderFrameStuff::new(&hardware)?);
            }
            render_stuff
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
            let depth_attachment = Attachment {
                format: Some(Format::D32Float),
                samples: 1,
                ops: AttachmentOps {
                    load: AttachmentLoadOp::Clear,
                    store: AttachmentStoreOp::DontCare,
                },
                stencil_ops: AttachmentOps::DONT_CARE,
                layouts: Layout::Undefined..Layout::DepthStencilAttachmentOptimal,
            };
            let subpass = SubpassDesc {
                colors: &[(0, Layout::ColorAttachmentOptimal)],
                depth_stencil: Some(&(1, Layout::DepthStencilAttachmentOptimal)),
                inputs: &[],
                resolves: &[],
                preserves: &[],
            };


            let in_dependency = SubpassDependency {
                passes: SubpassRef::External..SubpassRef::Pass(0),
                stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT
                    ..PipelineStage::COLOR_ATTACHMENT_OUTPUT | PipelineStage::EARLY_FRAGMENT_TESTS,
                accesses: ImageAccess::empty()
                    ..(ImageAccess::COLOR_ATTACHMENT_READ
                    | ImageAccess::COLOR_ATTACHMENT_WRITE
                    | ImageAccess::DEPTH_STENCIL_ATTACHMENT_READ
                    | ImageAccess::DEPTH_STENCIL_ATTACHMENT_WRITE),
            };
            let out_dependency = SubpassDependency {
                passes: SubpassRef::Pass(0)..SubpassRef::External,
                stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT | PipelineStage::EARLY_FRAGMENT_TESTS
                    ..PipelineStage::COLOR_ATTACHMENT_OUTPUT,
                accesses: (ImageAccess::COLOR_ATTACHMENT_READ
                    | ImageAccess::COLOR_ATTACHMENT_WRITE
                    | ImageAccess::DEPTH_STENCIL_ATTACHMENT_READ
                    | ImageAccess::DEPTH_STENCIL_ATTACHMENT_WRITE)..ImageAccess::empty(),
            };


            unsafe {
                hardware.device
                    .create_render_pass(&[color_attachment, depth_attachment], &[subpass], &[in_dependency, out_dependency])
                    .map_err(|_| "Couldn't create a render pass!")?
            }
        };

        let mut command_pool = unsafe {
            hardware.device
                .create_command_pool_typed(&hardware.queue_group, CommandPoolCreateFlags::RESET_INDIVIDUAL)
                .map_err(|_| "Could not create the raw command pool!")?
        };
        let framebuffer_stuff = match backbuffer {
            Backbuffer::Images(images) => {
                let framebuffer_stuff = images.into_iter().map(|image| -> Result<FramebufferStuff, &'static str> {
                    FramebufferStuff::new(extent, hardware, format, &image, &mut command_pool, &render_pass)
                }).collect::<Result<Vec<_>, &str>>()?;
                framebuffer_stuff
            }
            Backbuffer::Framebuffer(_) => unimplemented!("Can't handle framebuffer backbuffer!"),
        };

        let pipeline = Self::create_pipeline(&mut hardware.device, extent, &render_pass)?;

        let texture = ImageData::new(
            &hardware.adapter,
            &*hardware.device,
            &mut command_pool,
            &mut hardware.queue_group.queues[0],
            image::load_from_memory(crate::CREATURE_BYTES)
                .expect("binary corrupted")
                .to_rgba(),
        )?;


        Ok(Self {
            render_stuff,
            framebuffer_stuff,
            render_pass: ManuallyDrop::new(render_pass),
            command_pool: ManuallyDrop::new(command_pool),
            pipeline,
            current_frame: 0,
            texture,
            render_area: extent.to_extent().rect(),
            swapchain: ManuallyDrop::new(swapchain),
        })
    }

    pub fn drop_stuff(&mut self, hardware: &mut hardware::Hardware) {
        let _ = hardware.device.wait_idle();
        unsafe {
            self.pipeline.manually_drop(&*hardware.device);

            for render_stuff in self.render_stuff.drain(..) {
                render_stuff.drop_manually(hardware);
            }
            for framebuffer_stuff in self.framebuffer_stuff.drain(..) {
                framebuffer_stuff.drop_manually(hardware);
            }
            // LAST RESORT STYLE CODE, NOT TO BE IMITATED LIGHTLY
            use core::ptr::read;

            self.texture.manually_drop(&hardware.device);
            hardware.device.destroy_command_pool(
                ManuallyDrop::into_inner(read(&self.command_pool)).into_raw(),
            );
            hardware.device
                .destroy_render_pass(ManuallyDrop::into_inner(read(&self.render_pass)));
            hardware.device
                .destroy_swapchain(ManuallyDrop::into_inner(read(&self.swapchain)));
            println!("dropped hal");

        }
    }
}

