use crate::input::UserInput;

use nalgebra_glm as glm;
use glm::TMat4;
use crate::input::keyboard_state::{KeyboardState, KeyCode};

pub mod camera;

#[derive(Debug, Clone, Copy)]
pub struct LocalState {
    pub frame_width: f64,
    pub frame_height: f64,
    pub mouse_x: f64,
    pub mouse_y: f64,

    pub camera: camera::Camera,
}

impl Default for LocalState {
    fn default() -> Self {
        LocalState {
            frame_height: 0.0f64,
            frame_width: 0.0f64,
            mouse_x: 0.0f64,
            mouse_y: 0.0f64,
            camera: camera::Camera::new(50.0, 0.1, 100.0)
        }
    }
}

impl LocalState {
    pub fn update(&mut self, input: UserInput, keyboard: &KeyboardState, dt: f32) {

        if let Some(frame_size) = input.new_frame_size {
            self.frame_width = frame_size.0;
            self.frame_height = frame_size.1;
        }

        if let Some(position) = input.new_mouse_position {
            self.mouse_x = position.0;
            self.mouse_y = position.1;
        }

        if keyboard.button_pressed_in_last_frame(KeyCode::A) {
            println!("Ahaha");
        }

        //self.camera.update_orientation(0.0, 60.0*dt);
    }
}
