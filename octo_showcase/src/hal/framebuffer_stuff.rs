use super::prelude;
use super::prelude::*;
use super::hardware::Hardware;
use arrayvec::ArrayVec;

use gfx_hal::Device;

pub struct FramebufferStuff {
    pub framebuffer: Framebuffer,
    pub image_view: ImageView,
    pub depth: DepthImage,
    pub command_buffer: CommandBuffer,
}

impl FramebufferStuff {
    pub fn drop_manually(self, hardware: &Hardware) {
        unsafe {
            hardware.device.destroy_framebuffer(self.framebuffer);
            self.depth.manually_drop(&hardware.device);
            hardware.device.destroy_image_view(self.image_view);


        }
    }

    pub fn new(
        extent_2d: gfx_hal::window::Extent2D,
        hardware: &mut super::hardware::Hardware,
        format: gfx_hal::format::Format,
        image: &prelude::Image,
        render_pass: &RenderPass,
    ) -> Result<FramebufferStuff, &'static str>
    {
        unsafe {
            let extent = gfx_hal::image::Extent {
                width: extent_2d.width as _,
                height: extent_2d.height as _,
                depth: 1,
            };
            let image_view =
                hardware.device
                    .create_image_view(
                        image,
                        gfx_hal::image::ViewKind::D2,
                        format,
                        gfx_hal::format::Swizzle::NO,
                        gfx_hal::image::SubresourceRange {
                            aspects: gfx_hal::format::Aspects::COLOR,
                            levels: 0..1,
                            layers: 0..1,
                        },
                    )
                    .map_err(|_| "Couldn't create the image_view for the image!")?;
            let depth = DepthImage::new(&hardware.adapter, &*hardware.device, extent_2d)?;
            let attachments: ArrayVec<[_; 2]> = [&image_view, &depth.image_view].into();
            let framebuffer = hardware.device
                .create_framebuffer(&render_pass, attachments, extent)
                .map_err(|_| "Couldn't crate the framebuffer!")?;
            let command_buffer = hardware.command_pool.acquire_command_buffer();

            Result::Ok(FramebufferStuff { image_view, depth, framebuffer, command_buffer })
        }
    }
}
