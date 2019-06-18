use crate::hal::hardware::Hardware;

use gfx_hal::Device;
use super::prelude::*;

pub struct RenderFrameStuff {
    pub image_available: Semaphore,
    pub render_finished: Semaphore,
    pub in_flight: Fence,

}


impl RenderFrameStuff {
    pub fn new(hardware: &Hardware) -> Result<RenderFrameStuff, &'static str>{
        let image_available =
            hardware.device
                .create_semaphore()
                .map_err(|_| "Could not create a semaphore!")?;
        let render_finished =
            hardware.device
                .create_semaphore()
                .map_err(|_| "Could not create a semaphore!")?;
        let in_flight =
            hardware.device
                .create_fence(true)
                .map_err(|_| "Could not create a fence!")?;

        Result::Ok(RenderFrameStuff{
            image_available,
            render_finished,
            in_flight
        })
    }
    pub fn drop_manually(self, hardware: &Hardware) {
        unsafe {
            hardware.device.destroy_semaphore(self.image_available);
            hardware.device.destroy_semaphore(self.render_finished);
            hardware.device.destroy_fence(self.in_flight);
        }
    }
}

