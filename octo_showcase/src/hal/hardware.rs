use core::mem::{ManuallyDrop, size_of};
use std::ops::Deref;

use crate::back;
use super::buffers::{BufferBundleS, BufferUsage};
use super::prelude;
use super::prelude::*;
use gfx_hal::Instance;
use gfx_hal::queue::family::QueueFamily;
use gfx_hal::window::{Surface, Extent2D};
use gfx_hal::adapter::PhysicalDevice;
use gfx_hal::device::Device;

use nalgebra_glm as glm;

use crate::Quad;

pub struct Object {
    pub vertices: BufferBundleS,
    pub indices: BufferBundleS,
    pub mat: glm::TMat4<f32>,
}

pub struct Hardware {
    pub objects: Vec<Object>,
    instance: ManuallyDrop<prelude::Instance>,
    pub surface: prelude::Surface,
    pub adapter: Adapter,
    pub device: ManuallyDrop<prelude::Device>,
    pub queue_group: ManuallyDrop<QueueGroup>,
}

impl Hardware {
    fn upload_quad(&self, obj: &mut Object, quad: Quad) -> Result<(), &'static str> {
        let points = quad.vertex_attributes();
        self.write_data(&obj.vertices, &points)?;

        const INDEX_DATA: &[u16] = &[0, 1, 2, 2, 3, 0];
        self.write_data(&obj.indices, INDEX_DATA)?;


        Result::Ok(())
    }

    pub fn add_object(&mut self, position: glm::TVec3<f32>) -> Result<(), &'static str> {
        let (vertices, indices) = {
            const F32_XY_RGB_UV_QUAD: usize = size_of::<f32>() * (2 + 3) * 4;
            let vertices = self.create_buffer_bundle(F32_XY_RGB_UV_QUAD, BufferUsage::VERTEX)?;

            const U16_QUAD_INDICES: usize = size_of::<u16>() * 2 * 3;
            let indexes = self.create_buffer_bundle(U16_QUAD_INDICES, BufferUsage::INDEX)?;
            (vertices, indexes)
        };
        let quad = Quad {
            x: -1.0,
            y: -1.0,
            w: 2.0,
            h: 2.0,
        };
        let mut obj = Object { vertices, indices, mat: glm::translation(&position) };
        self.upload_quad(&mut obj, quad)?;
        self.objects.push(obj);

        Result::Ok(())
    }

    pub fn write_data<T: Copy>(&self, buffer: &BufferBundleS, data: &[T]) -> Result<(), &'static str> {
        unsafe {
            let mut data_target = self.device.acquire_mapping_writer(&buffer.memory, 0..buffer.requirements.size).map_err(|_| "Failed to acquire buffer mapping writer!")?;
            data_target[..data.len()].copy_from_slice(data);
            self.device.release_mapping_writer(data_target).map_err(|_| "Couldn't release buffer mapping writer")?;
        }
        Result::Ok(())
    }
    fn initialize_hardware(window: &winit::Window) -> Result<(prelude::Instance, prelude::Surface, Adapter, prelude::Device, QueueGroup), &'static str> {
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
            let gfx_hal::Gpu { device, mut queues } = unsafe {
                adapter
                    .physical_device
                    .open(&[(&queue_family, &[1.0; 1])])
                    .map_err(|_| "Couldn't open the PhysicalDevice!")?
            };
            let queue_group = queues
                .take::<gfx_hal::Graphics>(queue_family.id())
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
    pub fn new(window: &winit::Window) -> Result<Hardware, &'static str> {
        let (instance, mut surface, adapter, device, queue_group) = Self::initialize_hardware(window)?;

        Result::Ok(Hardware {
            objects: vec![],
            device: ManuallyDrop::new(device),
            instance: ManuallyDrop::new(instance),
            adapter,
            surface,
            queue_group: ManuallyDrop::new(queue_group),

        })
    }

    pub fn create_buffer_bundle(&self, size: usize, usage: BufferUsage) -> Result<BufferBundleS, &'static str> {
        BufferBundleS::new(&self.adapter, &*self.device, size, usage)
    }
}


impl core::ops::Drop for Hardware {
    fn drop(&mut self) {
        unsafe {
            for obj in &self.objects {
                obj.vertices.manually_drop(&self.device);
                obj.indices.manually_drop(&self.device);
            }
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
