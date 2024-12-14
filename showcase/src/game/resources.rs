use glam::Vec4;
use strum::{EnumCount, EnumIter, IntoEnumIterator};

use just_renderer::{MaterialId, Renderer};

#[derive(Debug, EnumCount, Clone, Copy, EnumIter)]
#[repr(u8)]
pub enum GameModel {
    Cube = 0,
    Cow,
}

impl GameModel {
    pub fn path(&self) -> &'static str {
        match self {
            GameModel::Cow => "res/cow1.obj",
            GameModel::Cube => "res/cube.obj",
        }
    }
}

#[derive(Debug, EnumCount, Clone, Copy, EnumIter)]
#[repr(u8)]
pub enum GameTexture {
    Tree = 0,
}

impl GameTexture {
    pub fn path(&self) -> &'static str {
        match self {
            GameTexture::Tree => "res/test_image.png",
        }
    }
}

pub(crate) struct GameAssets {
    pub tree_material: MaterialId,
}

pub(crate) async fn initialize_game_assets(renderer: &mut Renderer) -> GameAssets {
    for model in GameModel::iter() {
        renderer.load_game_model(model as u32, model.path()).await;
    }

    for tex in GameTexture::iter() {
        renderer.load_game_texture(tex as u32, tex.path()).await;
    }

    GameAssets {
        tree_material: renderer.create_material(GameTexture::Tree as u32, Vec4::ZERO),
    }
}
