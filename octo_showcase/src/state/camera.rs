use nalgebra_glm as glm;
use glm::TMat4;
use glm::TVec3;
use glm::Qua;


#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: TVec3<f32>,
    pitch: f32,
    yaw: f32,

    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub ratio: f32,

}

impl Camera {
    pub fn new(ratio: f32,fov: f32, near: f32, far: f32) -> Camera {
        Camera {
            fov,
            near,
            far,
            ratio,
            position: glm::vec3(0f32, 0f32, 0f32),
            pitch: 0f32,
            yaw: 0f32,
        }
    }
    pub fn update_orientation(&mut self, d_pitch_deg: f32, d_yaw_deg: f32) {
        self.pitch = (self.pitch + d_pitch_deg).max(-89.0).min(89.0);
        self.yaw= (self.yaw + d_yaw_deg) % 360.0;
    }
    pub fn make_front(&self) -> TVec3<f32> {
        let pitch_rad = f32::to_radians(self.pitch);
        let yaw_rad = f32::to_radians(self.yaw);
        glm::make_vec3(&[
            yaw_rad.sin() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.cos() * pitch_rad.cos(),
        ])
    }

    pub fn make_right(&self) -> TVec3<f32> {
        let yaw_rad = f32::to_radians(self.yaw);
        glm::make_vec3(&[
            yaw_rad.cos(),
            0.0f32,
            -yaw_rad.sin(),
        ])
    }
    pub fn make_view_matrix(&self) -> TMat4<f32> {
        glm::look_at_lh(
            &self.position,
            &(self.position + self.make_front()),
            &glm::make_vec3(&[0.0f32, 1.0, 0.0]),
        )
    }

    pub fn make_projection_matrix(&self) -> TMat4<f32> {
        let mut temp = glm::perspective_lh_zo(self.ratio, f32::to_radians(self.fov), self.near, self.far);
        temp[(1, 1)] *= -1.0;
        temp
    }
    pub fn matrix(&self) -> TMat4<f32> {
        self.make_projection_matrix() * self.make_view_matrix()
    }
}
