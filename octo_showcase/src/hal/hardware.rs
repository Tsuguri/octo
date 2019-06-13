use core::mem::ManuallyDrop;
use std::ops::Deref;

use crate::back;
use super::buffers::{BufferBundleS, BufferUsage};
use super::prelude::*;


pub struct Hardware {
    instance: ManuallyDrop<Instance>,
    surface: Surface,
    adapter: Adapter,
    pub device: ManuallyDrop<Device>,
    pub queue_group: ManuallyDrop<QueueGroup>,
}

impl Hardware {
    pub fn new(device: Device, instance: Instance, adapter: Adapter, surface: Surface, queue_group: QueueGroup) -> Hardware {
        Hardware {
            device: ManuallyDrop::new(device),
            instance: ManuallyDrop::new(instance),
            adapter: adapter,
            surface: surface,
            queue_group: ManuallyDrop::new(queue_group),

        }
    }

    pub fn create_buffer_bundle(&self, size: usize, usage: BufferUsage) -> Result<BufferBundleS, &'static str> {
        BufferBundleS::new(&self.adapter, &*self.device, size, usage)
    }
}


impl core::ops::Drop for Hardware {
    fn drop(&mut self) {
        unsafe {
            println!("dropping hardware");
            ManuallyDrop::drop(&mut self.queue_group);
            ManuallyDrop::drop(&mut self.device);
            println!("heh");
            //ManuallyDrop::drop(&mut self.surface);
            println!("heh2");
            ManuallyDrop::drop(&mut self.instance);
            println!("heh3");
        }
    }
}
