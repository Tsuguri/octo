use crate::back;

use gfx_hal::{
    Backend,
};

pub type Semaphore = <back::Backend as Backend>::Semaphore;
pub type Fence = <back::Backend as Backend>::Fence;
pub type Device = back::Device;
pub type Instance = back::Instance;
pub type Adapter = gfx_hal::Adapter<back::Backend>;
pub type Surface = <back::Backend as Backend>::Surface;
pub type QueueGroup = gfx_hal::queue::QueueGroup<back::Backend, gfx_hal::Graphics>;
pub type Backbuffer = gfx_hal::Backbuffer<back::Backend>;
pub type Swapchain = <back::Backend as Backend>::Swapchain;
pub type Framebuffer = <back::Backend as Backend>::Framebuffer;
pub type ImageView = <back::Backend as Backend>::ImageView;
pub type CommandBuffer =gfx_hal::command::CommandBuffer<back::Backend, gfx_hal::Graphics, gfx_hal::command::MultiShot, gfx_hal::command::Primary>;

pub type DescriptorSetLayout = <back::Backend as Backend>::DescriptorSetLayout;
pub type DescriptorPool = <back::Backend as Backend>::DescriptorPool;
pub type DescriptorSet = <back::Backend as Backend>::DescriptorSet;
pub type PipelineLayout = <back::Backend as Backend>::PipelineLayout;
pub type GraphicsPipeline = <back::Backend as Backend>::GraphicsPipeline;
pub type CommandPool = gfx_hal::CommandPool<back::Backend, gfx_hal::Graphics>;
pub type RenderPass =<back::Backend as Backend>::RenderPass;
pub type Image = <back::Backend as Backend>::Image;

pub type DepthImage = crate::images::DepthImage<back::Backend, back::Device>;
pub type ImageData = crate::images::ImageData<back::Backend, back::Device>;
