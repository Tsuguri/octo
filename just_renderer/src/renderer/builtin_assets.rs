use std::rc::Rc;

use glam::{Vec2, Vec3};

use crate::{
    model_data::{ModelData, ModelVertex},
    texture::TextureData,
};

use super::Renderer;

impl Renderer {
    pub(crate) async fn load_builtin_data(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        #[cfg(target_arch = "wasm32")] url_origin: &str,
    ) -> (Vec<ModelData>, Vec<Rc<TextureData>>) {
        let quad_model = ModelData::initialize(
            device,
            "quad",
            vec![
                ModelVertex {
                    position: Vec3::new(0.0f32, 0.0f32, 0.0f32),
                    normal: Vec3::new(0.0f32, 1.0f32, 0.0f32),
                    uv: Vec2::new(0.0f32, 1.0f32),
                },
                ModelVertex {
                    position: Vec3::new(1.0f32, 0.0f32, 0.0f32),
                    normal: Vec3::new(0.0f32, 1.0f32, 0.0f32),
                    uv: Vec2::new(1.0f32, 1.0f32),
                },
                ModelVertex {
                    position: Vec3::new(0.0f32, 0.0f32, 1.0f32),
                    normal: Vec3::new(0.0f32, 1.0f32, 0.0f32),
                    uv: Vec2::new(0.0f32, 0.0f32),
                },
                ModelVertex {
                    position: Vec3::new(1.0f32, 0.0f32, 1.0f32),
                    normal: Vec3::new(0.0f32, 1.0f32, 0.0f32),
                    uv: Vec2::new(1.0f32, 0.0f32),
                },
            ],
            vec![0, 2, 1, 1, 2, 3],
        );
        let bytes = Self::load_file(
            "res/test_image.png",
            #[cfg(target_arch = "wasm32")]
            url_origin,
        )
        .await;
        let tree_texture =
            Rc::new(TextureData::from_bytes(device, queue, &bytes, "test_tree").unwrap());

        return (vec![quad_model], vec![tree_texture]);
    }
}
