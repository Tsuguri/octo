use std::f32::consts::PI;

use glam::{Quat, Vec3};
use just_renderer::winit::{event::ElementState, keyboard::KeyCode};
use just_renderer::{
    BuiltInModel, Configuration, InstanceId, Light, LightId, LightStrength, ModelId,
    PointLightParameters, Renderer,
};

use super::{camera_controller::OrthoCameraController, resources::GameAssets, GameModel};

#[derive(Default)]
struct Actions {
    change_scene_render_enabled: bool,
}

pub struct Scene {
    _assets: GameAssets,
    camera: OrthoCameraController,
    cubes: [InstanceId; 9],
    cow: InstanceId,
    actions: Actions,
    light1: LightId,
    light2: LightId,
}

impl Scene {
    pub(crate) fn initialize_scene(renderer: &mut Renderer, assets: GameAssets) -> Self {
        let camera = OrthoCameraController::new(PI / 3.0, -PI / 4.0, Vec3::ZERO, 6.0);

        let (camera_pos, camera_rot) = camera.get_data();
        renderer.set_camera(camera_pos, camera_rot);
        let cubes: [InstanceId; 9] = core::array::from_fn(|id| {
            renderer.create_object_with_transform(
                ModelId::Game(GameModel::Cube as u32),
                assets.tree_material,
                Vec3::new((id % 3) as f32, 0.0f32, (id / 3) as f32),
                Quat::IDENTITY,
                Vec3::splat(0.3),
            )
        });
        let cow = renderer.create_object_with_transform(
            ModelId::Game(GameModel::Cow as u32),
            assets.tree_material,
            Vec3::ZERO,
            Quat::IDENTITY,
            Vec3::ONE,
        );

        let scale = Vec3::splat(30.0);

        renderer.create_object_with_transform(
            ModelId::BuiltIn(BuiltInModel::Quad),
            assets.tree_material,
            Vec3::new(-15.0, -2.0, -15.0),
            Quat::IDENTITY,
            scale,
        );

        let attenuation = PointLightParameters::attenuation_for_range(20.0);

        let light1 = renderer.create_light(Light::Point(PointLightParameters {
            pos: Vec3::new(2.0, 3.0, 2.0),
            colors: LightStrength::splat_floats(1.0, 1.0, 1.0, 0.5, 0.1),
            attenuation,
        }));
        let light2 = renderer.create_light(Light::Point(PointLightParameters {
            pos: Vec3::new(2.0, 3.0, 2.0),
            colors: LightStrength::splat_floats(1.0, 0.0, 1.0, 0.5, 0.1),
            attenuation,
        }));

        Self {
            camera,
            cubes,
            cow,
            _assets: assets,
            actions: Default::default(),
            light1,
            light2,
        }
    }

    pub fn update(&mut self, renderer: &mut Renderer, total_time: f32) {
        let actions = std::mem::replace(&mut self.actions, Default::default());
        if actions.change_scene_render_enabled {
            let conf = renderer.runtime_configuration_mut();
            conf.render_scene = !conf.render_scene;
        }
        let (camera_pos, camera_rot) = self.camera.get_data();
        renderer.set_camera(camera_pos, camera_rot);
        // for cube in self.cubes.iter().enumerate() {
        //     if let Some(obj) = renderer.object(*cube.1) {
        //         let mut pos = obj.positon;
        //         pos.y = f32::sin(total_time * ((cube.0 / 3) as f32 + 0.1))
        //             * ((cube.0 % 3) as f32 + 1.0);
        //         obj.positon = pos;
        //     }
        // }

        if let Some(obj) = renderer.object(self.cubes[0]) {
            obj.positon = Vec3::new(f32::sin(total_time), 2.0, f32::cos(total_time));
        }
        if let Some(obj) = renderer.object(self.cubes[1]) {
            obj.positon = Vec3::new(f32::sin(total_time + PI), 2.0, f32::cos(total_time + PI));
        }

        if let Some(light) = renderer.light(self.light1) {
            match light {
                Light::Point(data) => {
                    data.pos = Vec3::new(f32::sin(total_time), 2.0, f32::cos(total_time));
                }
                _ => {}
            }
        }
        if let Some(light) = renderer.light(self.light2) {
            match light {
                Light::Point(data) => {
                    data.pos = Vec3::new(f32::sin(total_time + PI), 2.0, f32::cos(total_time + PI));
                }
                _ => {}
            }
        }

        if let Some(obj) = renderer.object(self.cow) {
            obj.rotation = Quat::from_rotation_y(-total_time);
        }
    }

    pub fn handle_keyboard_input(&mut self, key: KeyCode, state: ElementState) {
        // TODO: use better input handling for constant speed movement
        let look_at = self.camera.look_at;
        let forward = {
            let mut tmp = self.camera.forward();
            tmp.y = 0.0;
            tmp.normalize()
        };
        let right = -forward.cross(Vec3::Y);
        let speed = 0.2f32;

        match key {
            KeyCode::KeyA => self.camera.look_at = look_at - right * speed,
            KeyCode::KeyD => self.camera.look_at = look_at + right * speed,
            KeyCode::KeyS => self.camera.look_at = look_at - forward * speed,
            KeyCode::KeyW => self.camera.look_at = look_at + forward * speed,
            KeyCode::Space if state == ElementState::Pressed => {
                //self.actions.change_scene_render_enabled = true;
            }

            // uncomment to change camera rotations with arrows
            // VirtualKeyCode::Left => camera.yaw = camera.yaw + 0.1,
            // VirtualKeyCode::Right => camera.yaw = camera.yaw - 0.1,
            // VirtualKeyCode::Up => camera.pitch = camera.pitch + 0.1,
            // VirtualKeyCode::Down => camera.pitch = camera.pitch - 0.1,
            _ => {}
        }
    }
}
