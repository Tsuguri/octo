/*use hal;
use hal::buffer;
use hal::command;
use hal::image as i;
use hal::Device;
use image;
use std::io::Cursor;

pub struct ImageLoadingData<B: hal::Backend> {
    //
    pub memory: B::Memory,
    pub buffer: B::Buffer,
    pub kind: i::Kind,
    pub width: u32,
    pub height: u32,
    pub row_pitch: u32,
    pub stride: usize,
}

impl<B: hal::Backend> ImageLoadingData<B> {
    pub fn destroy(self, device: &B::Device) {
        device.destroy_buffer(self.buffer);
        device.free_memory(self.memory)
    }

    pub fn copy_to_image<C: hal::Supports<hal::Transfer>>(
        &self,
        cmd_buffer: &mut hal::command::CommandBuffer<B, C>,
        image: &B::Image,
    ) {
        //
        cmd_buffer.copy_buffer_to_image(
            &self.buffer,
            image,
            i::Layout::TransferDstOptimal,
            &[command::BufferImageCopy {
                buffer_offset: 0,
                buffer_width: self.row_pitch / (self.stride as u32),
                buffer_height: self.height as u32,
                image_layers: i::SubresourceLayers {
                    aspects: hal::format::Aspects::COLOR,
                    level: 0,
                    layers: 0..1,
                },
                image_offset: i::Offset { x: 0, y: 0, z: 0 },
                image_extent: i::Extent {
                    width: self.width,
                    height: self.height,
                    depth: 1,
                },
            }],
        );
    }
}

pub fn load_image<B: hal::Backend>(
    data: &[u8],
    device: &B::Device,
    limits: &hal::Limits,
    upload_type: hal::MemoryTypeId,
    format: image::ImageFormat,
) -> ImageLoadingData<B> {
    // TODO: Proper error handling of image loading.
    let img = image::load(Cursor::new(data), format).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = i::Kind::D2(width as i::Size, height as i::Size, 1, 1);
    let row_alignment_mask = limits.min_buffer_copy_pitch_alignment as u32 - 1;
    let stride = 4usize;
    let row_pitch = (width * stride as u32 + row_alignment_mask) & !row_alignment_mask;
    let upload_size = (height * row_pitch) as u64;

    let image_buffer_unbound = device
        .create_buffer(upload_size, buffer::Usage::TRANSFER_SRC)
        .unwrap();
    let image_mem_reqs = device.get_buffer_requirements(&image_buffer_unbound);
    let image_upload_memory = device
        .allocate_memory(upload_type, image_mem_reqs.size)
        .unwrap();
    let image_upload_buffer = device
        .bind_buffer_memory(&image_upload_memory, 0, image_buffer_unbound)
        .unwrap();

    // copy image data into staging buffer
    {
        let mut data = device
            .acquire_mapping_writer::<u8>(&image_upload_memory, 0..upload_size)
            .unwrap();
        for y in 0..height as usize {
            let row = &(*img)[y * (width as usize) * stride..(y + 1) * (width as usize) * stride];
            let dest_base = y * row_pitch as usize;
            data[dest_base..dest_base + row.len()].copy_from_slice(row);
        }
        device.release_mapping_writer(data);
    }
    ImageLoadingData {
        memory: image_upload_memory,
        buffer: image_upload_buffer,
        kind,
        width,
        height,
        row_pitch,
        stride,
    }
}*/
