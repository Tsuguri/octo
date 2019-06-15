use crate::back;

use gfx_hal::{
    Backend,
};

pub type Semaphore = <back::Backend as Backend>::Semaphore;
pub type Device = back::Device;
pub type Instance = back::Instance;
pub type Adapter = gfx_hal::Adapter<back::Backend>;
pub type Surface = <back::Backend as Backend>::Surface;
pub type QueueGroup = gfx_hal::queue::QueueGroup<back::Backend, gfx_hal::Graphics>;
pub type Backbuffer = gfx_hal::Backbuffer<back::Backend>;
pub type Swapchain = <back::Backend as Backend>::Swapchain;

pub type DescriptorSetLayout = <back::Backend as Backend>::DescriptorSetLayout;
pub type DescriptorPool = <back::Backend as Backend>::DescriptorPool;
pub type DescriptorSet = <back::Backend as Backend>::DescriptorSet;
pub type PipelineLayout = <back::Backend as Backend>::PipelineLayout;
pub type GraphicsPipeline = <back::Backend as Backend>::GraphicsPipeline;
