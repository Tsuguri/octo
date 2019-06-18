use super::prelude;
use super::prelude::*;
use std::mem::ManuallyDrop;
use std::ops::Deref;

use gfx_hal::Device;
use crate::hal::hardware::Hardware;

pub struct Pipeline {
    pub descriptor_set_layouts: Vec<DescriptorSetLayout>,
    pub descriptor_pool: ManuallyDrop<DescriptorPool>,
    pub descriptor_set: ManuallyDrop<DescriptorSet>,
    pub pipeline_layout: ManuallyDrop<PipelineLayout>,
    pub graphics_pipeline: ManuallyDrop<GraphicsPipeline>,
}

impl Pipeline {
    pub unsafe fn write_descriptor_sets(&self, hardware: &Hardware, texture: &ImageData){
        hardware.device.write_descriptor_sets(vec![
            gfx_hal::pso::DescriptorSetWrite {
                set: &*self.descriptor_set,
                binding: 0,
                array_offset: 0,
                descriptors: Some(gfx_hal::pso::Descriptor::Image(
                    texture.image_view.deref(),
                    gfx_hal::image::Layout::Undefined,
                )),
            },
            gfx_hal::pso::DescriptorSetWrite {
                set: &*self.descriptor_set,
                binding: 1,
                array_offset: 0,
                descriptors: Some(gfx_hal::pso::Descriptor::Sampler(
                    texture.sampler.samp.deref(),
                )),
            },
        ]);
    }
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