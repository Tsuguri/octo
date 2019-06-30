use core::mem::{ManuallyDrop, size_of};
use std::path::Path;

use tobj;
use crate::back;
use crate::Vertex;
use super::buffers::{BufferBundleS, BufferUsage};
use super::prelude;
use super::prelude::*;
use gfx_hal::Instance;
use gfx_hal::queue::family::QueueFamily;
use gfx_hal::window::Surface;
use gfx_hal::adapter::PhysicalDevice;
use gfx_hal::device::Device;
use gfx_hal::pool::CommandPoolCreateFlags;
use arrayvec::ArrayVec;

use nalgebra_glm as glm;

use crate::Quad;

#[derive(Debug, Copy, Clone)]
pub struct ModelId(usize);

impl TextureId {
    pub fn id(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TextureId(usize);

pub struct Model {
    vertices: BufferBundleS,
    pub indices: BufferBundleS,
    pub indices_len: u32,

}

impl ModelId {
    pub fn id(&self) -> usize {
        self.0
    }
}

impl Model {
    pub fn vertex(&self) -> ArrayVec<[(&<back::Backend as gfx_hal::Backend>::Buffer, u64); 1]> {
        let buffer_ref: &<back::Backend as gfx_hal::Backend>::Buffer = &self.vertices.buffer;
        let buffers: ArrayVec<[_; 1]> = [(buffer_ref, 0)].into();
        buffers
    }
}

pub struct Hardware {
    pub models: Vec<Model>,
    pub textures: Vec<ImageData>,
    instance: ManuallyDrop<prelude::Instance>,
    pub surface: prelude::Surface,
    pub adapter: Adapter,
    pub device: ManuallyDrop<prelude::Device>,
    pub queue_group: ManuallyDrop<QueueGroup>,
    pub command_pool: ManuallyDrop<CommandPool>,
}

impl Hardware {
    pub fn add_texture_file(&mut self, filename: &str) -> Result<TextureId, &'static str> {
        let f = std::fs::File::open(filename).unwrap();
        let f = std::io::BufReader::new(f);
        let img = image::load(f, image::ImageFormat::PNG).unwrap().to_rgba();
        let tex = ImageData::new(
            &self.adapter,
            &*self.device,
            &mut self.command_pool,
            &mut self.queue_group.queues[0],
            img
        )?;
        self.textures.push(tex);
        Result::Ok(TextureId(self.textures.len()))
    }
    pub fn add_texture_bytes(&mut self, buffer: &[u8]) -> Result<TextureId, &'static str> {
        let tex = ImageData::new(
            &self.adapter,
            &*self.device,
            &mut self.command_pool,
            &mut self.queue_group.queues[0],
            image::load_from_memory(buffer).expect("binary not ok").to_rgba(),
        )?;
        self.textures.push(tex);

        Result::Ok(TextureId(self.textures.len() - 1))
    }
    pub fn add_object(&mut self, filename: &str) -> Result<ModelId, &'static str> {
        let (models, _materials) = tobj::load_obj(&Path::new(filename)).unwrap();

        if models.len() != 1 {
            panic!("there should be exactly one mode");
        }
        let model = &models[0];
        let mesh = &model.mesh;

        let indices: Vec<_> = mesh.indices.iter().map(|x| *x as u16).collect();
        let num = mesh.positions.len() / 3;

        let vertices: Vec<_> = (0..num).map(|x| {
            let xyz = [mesh.positions[x * 3], mesh.positions[x * 3 + 1], mesh.positions[x * 3 + 2]];

            let normal = [mesh.normals[x * 3], mesh.normals[x * 3 + 1], mesh.normals[x * 3 + 2]];
            let uv = [0.0f32, 0.0f32];
            //let uv = [mesh.texcoords[x * 2], mesh.texcoords[x * 2 + 1]];
            Vertex {
                xyz,
                normal,
                uv,
            }
        }).collect();
        let (vertex_buffer, index_buffer) = {
            let vertex_buffer = self.create_buffer_bundle(vertices.len() * size_of::<f32>() * (2 + 3 + 3), BufferUsage::VERTEX)?;

            let index_buffer = self.create_buffer_bundle(indices.len() * size_of::<u16>(), BufferUsage::INDEX)?;
            (vertex_buffer, index_buffer)
        };
        self.write_data(&vertex_buffer, &vertices)?;
        self.write_data(&index_buffer, &indices)?;

        let obj = Model { indices_len: indices.len() as u32, vertices: vertex_buffer, indices: index_buffer };
        self.models.push(obj);

        Result::Ok(ModelId(self.models.len() - 1))
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
        let (device, queue_group) = {
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
        let (instance, surface, adapter, device, queue_group) = Self::initialize_hardware(window)?;


        let mut command_pool = unsafe {
            device
                .create_command_pool_typed(&queue_group, CommandPoolCreateFlags::RESET_INDIVIDUAL)
                .map_err(|_| "Could not create the raw command pool!")?
        };

        Result::Ok(Hardware {
            models: vec![],
            textures: vec![],
            device: ManuallyDrop::new(device),
            instance: ManuallyDrop::new(instance),
            adapter,
            surface,
            queue_group: ManuallyDrop::new(queue_group),
            command_pool: ManuallyDrop::new(command_pool),

        })
    }

    pub fn create_buffer_bundle(&self, size: usize, usage: BufferUsage) -> Result<BufferBundleS, &'static str> {
        BufferBundleS::new(&self.adapter, &*self.device, size, usage)
    }
}


impl core::ops::Drop for Hardware {
    fn drop(&mut self) {
        unsafe {
            for obj in self.models.drain(..) {
                obj.vertices.manually_drop(&self.device);
                obj.indices.manually_drop(&self.device);
            }
            for texture in self.textures.drain(..) {
                texture.manually_drop(&self.device);
            }
            use core::ptr::read;
            self.device.destroy_command_pool(
                ManuallyDrop::into_inner(read(&self.command_pool)).into_raw(),
            );
            println!("dropping hardware");
            ManuallyDrop::drop(&mut self.queue_group);
            ManuallyDrop::drop(&mut self.device);
            ManuallyDrop::drop(&mut self.instance);
        }
    }
}
