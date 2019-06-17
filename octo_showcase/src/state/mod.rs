use crate::input::UserInput;

use nalgebra_glm as glm;
use glm::TMat4;
use crate::input::keyboard_state::{KeyboardState, KeyCode};
use crate::input::mouse_state::MouseState;

pub mod camera;

#[derive(Debug, Clone, Copy)]
pub struct LocalState {
    pub frame_width: f64,
    pub frame_height: f64,

    pub camera: camera::Camera,
}

impl Default for LocalState {
    fn default() -> Self {
        LocalState {
            frame_height: 0.0f64,
            frame_width: 0.0f64,
            camera: camera::Camera::new(800.0/600.0,50.0, 0.1, 100.0),
        }
    }
}

impl LocalState {
    pub fn update(&mut self, input: UserInput, keyboard: &KeyboardState, mouse: &MouseState, dt: f32) {
        if let Some(frame_size) = input.new_frame_size {
            self.frame_width = frame_size.0;
            self.frame_height = frame_size.1;
            self.camera.ratio = (frame_size.0/frame_size.1) as f32;
            println!("new ratio: {}", self.camera.ratio);
        }

        let dm = mouse.get_mouse_move();

        let dmove = 0.7f32;
        let kmove = 3.0f32;
        if mouse.left_button_down() {
            self.camera.update_orientation(-dm[1] as f32 * dmove, dm[0] as f32 * dmove);
        }

        if keyboard.is_button_down(KeyCode::A) {
            self.camera.position -= self.camera.make_right() * dt * kmove;
        }
        if keyboard.is_button_down(KeyCode::D) {
            self.camera.position += self.camera.make_right() * dt * kmove;
        }
        if keyboard.is_button_down(KeyCode::W) {
            self.camera.position += self.camera.make_front() * dt * kmove;
        }
        if keyboard.is_button_down(KeyCode::S) {
            self.camera.position -= self.camera.make_front() * dt * kmove;
        }
        if keyboard.is_button_down(KeyCode::Space) {
            self.camera.position += (&self.camera.make_front().cross(&self.camera.make_right())) * dt * kmove;
        }
        if keyboard.is_button_down(KeyCode::LShift) {
            self.camera.position -= (&self.camera.make_front().cross(&self.camera.make_right())) * dt * kmove;
        }


        //self.camera.update_orientation(0.0, 60.0*dt);
    }
}
