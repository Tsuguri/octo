use super::prelude;
use super::prelude::*;
use std::mem::ManuallyDrop;

use gfx_hal::Device;

pub struct Pipeline {
    pub descriptor_set_layouts: Vec<DescriptorSetLayout>,
    pub descriptor_pool: ManuallyDrop<DescriptorPool>,
    pub descriptor_set: ManuallyDrop<DescriptorSet>,
    pub pipeline_layout: ManuallyDrop<PipelineLayout>,
    pub graphics_pipeline: ManuallyDrop<GraphicsPipeline>,
}

impl Pipeline {
    pub unsafe fn manually_drop(&mut self, device: &prelude::Device) {
        use core::ptr::read;
        for descriptor_set_layout in self.descriptor_set_layouts.drain(..) {
            device.destroy_descriptor_set_layout(descriptor_set_layout)
        }
        device
            .destroy_descriptor_pool(ManuallyDrop::into_inner(read(&self.descriptor_pool)));
        device
            .destroy_pipeline_layout(ManuallyDrop::into_inner(read(&self.pipeline_layout)));
        device
            .destroy_graphics_pipeline(ManuallyDrop::into_inner(read(&self.graphics_pipeline)));
    }
}