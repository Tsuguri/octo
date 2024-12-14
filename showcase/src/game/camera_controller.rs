use glam::{Mat4, Quat, Vec3};

pub(crate) struct OrthoCameraController {
    pub pitch: f32,
    pub yaw: f32,
    pub look_at: Vec3,
    pub distance: f32,
}

impl OrthoCameraController {
    pub fn new(pitch: f32, yaw: f32, look_at: Vec3, distance: f32) -> Self {
        Self {
            pitch,
            yaw,
            look_at,
            distance,
        }
    }
    pub fn forward(&self) -> Vec3 {
        let rot = Quat::from_euler(glam::EulerRot::YXZ, self.yaw, self.pitch, 0.0);
        rot.mul_vec3(Vec3::new(0.0, 0.0, 1.0))
    }

    pub fn get_data(&self) -> (Vec3, Quat) {
        let pos = self.look_at - self.forward() * self.distance;
        let camera_rot = Quat::from_mat4(&Mat4::look_at_lh(pos, self.look_at, Vec3::Y));

        (pos, camera_rot)
    }
}
