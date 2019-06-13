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