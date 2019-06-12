use crate::input::UserInput;

use nalgebra_glm as glm;
use glm::TMat4;

#[derive(Debug, Clone, Copy)]
pub struct LocalState {
    pub frame_width: f64,
    pub frame_height: f64,
    pub mouse_x: f64,
    pub mouse_y: f64,

    pub view: TMat4<f32>,
    pub projection: TMat4<f32>,
}

impl Default for LocalState {
    fn default() -> Self {
        let projection = {
            let mut temp = glm::perspective_lh_zo(800.0 / 600.0, f32::to_radians(50.0), 0.1, 100.0);
            temp[(1, 1)] *= -1.0;
            temp
        };
        LocalState {
            view: glm::look_at_lh(
                &glm::make_vec3(&[2.0, 2.0, -5.0]),
                &glm::make_vec3(&[0.0, 0.0, 0.0]),
                &glm::make_vec3(&[0.0, 1.0, 0.0]).normalize(),
            ),
            projection,
            frame_height: 0.0f64,
            frame_width: 0.0f64,
            mouse_x: 0.0f64,
            mouse_y: 0.0f64,
        }
    }
}

impl LocalState {
    pub fn update_from_input(&mut self, input: UserInput) {
        if let Some(frame_size) = input.new_frame_size {
            self.frame_width = frame_size.0;
            self.frame_height = frame_size.1;
        }
        if let Some(position) = input.new_mouse_position {
            self.mouse_x = position.0;
            self.mouse_y = position.1;
        }
    }
}
