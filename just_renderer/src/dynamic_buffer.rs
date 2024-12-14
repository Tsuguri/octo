use std::marker::PhantomData;

use encase::{internal::WriteInto, ShaderType};
use wgpu::BufferAddress;

pub struct DynamicBuffer<T>
where
    T: ShaderType + WriteInto,
{
    buffer: wgpu::Buffer,
    size: BufferAddress,
    _phantom: PhantomData<T>,
}

impl<T: ShaderType + WriteInto> DynamicBuffer<T> {
    pub fn new(device: &wgpu::Device, usage: wgpu::BufferUsages) -> Self {
        let size = T::min_size().get().next_power_of_two().max(16);
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: size,
            usage: usage | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            buffer,
            size,
            _phantom: PhantomData,
        }
    }

    pub fn inner(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    // returns true if buffer was reallocated
    #[must_use]
    pub fn ensure_size(&mut self, device: &wgpu::Device, data: &T) -> bool {
        let required = data.size().get();
        if required > self.size {
            let new_size = required.next_power_of_two();
            self.buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: new_size,
                usage: self.buffer.usage(),
                mapped_at_creation: false,
            });
            true
        } else {
            false
        }
    }

    // returns true if buffer was reallocated
    #[must_use]
    pub fn update(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, data: &T) -> bool {
        let changed = self.ensure_size(device, data);
        let mut data_buffer = encase::StorageBuffer::new(Vec::new());
        data_buffer.write(data).unwrap();
        let bytes = data_buffer.into_inner();

        queue.write_buffer(&self.buffer, 0, &bytes);
        changed
    }
}
