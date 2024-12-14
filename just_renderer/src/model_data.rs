use std::io::{BufReader, Cursor};

use glam::{Vec2, Vec3};
use wgpu::util::DeviceExt as _;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

impl ModelVertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // normal
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<Vec3>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<Vec3>() + std::mem::size_of::<Vec3>())
                        as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub struct ModelData {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
}

impl ModelData {
    pub fn initialize_from_obj(device: &wgpu::Device, data: &[u8], name: &str) -> Self {
        let obj_cursor = Cursor::new(data);
        let mut reader = BufReader::new(obj_cursor);
        let mut out = tobj::load_obj_buf(
            &mut reader,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
            |p| {
                println!("path: {:?}", p.to_str());
                let mat_tex = std::fs::read_to_string(p).unwrap();
                tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_tex)))
            },
        )
        .unwrap();

        assert_eq!(out.0.len(), 1);

        let model = out.0.pop().unwrap();

        let vertices = (0..model.mesh.positions.len() / 3)
            .map(|id| ModelVertex {
                position: Vec3::new(
                    model.mesh.positions[id * 3],
                    model.mesh.positions[id * 3 + 1],
                    model.mesh.positions[id * 3 + 2],
                ),
                normal: Vec3::new(
                    model.mesh.normals[id * 3],
                    model.mesh.normals[id * 3 + 1],
                    model.mesh.normals[id * 3 + 2],
                ),
                uv: Vec2::new(
                    model.mesh.texcoords[id * 2],
                    model.mesh.texcoords[id * 2 + 1],
                ),
            })
            .collect::<Vec<_>>();

        Self::initialize(device, name, vertices, model.mesh.indices)
    }

    pub fn initialize(
        device: &wgpu::Device,
        name: &str,
        vertices: Vec<ModelVertex>,
        indices: Vec<u32>,
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Vertex Buffer", name)),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Index Buffer", name)),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            num_elements: indices.len() as u32,
        }
    }
}
