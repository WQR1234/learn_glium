use cgmath::{Angle, Matrix4, Point3, Quaternion, Vector3};

pub struct Camera {
    pub pos: Point3<f32>,
    pub front: Vector3<f32>,
    pub up: Vector3<f32>,



}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Point3::new(0.0, 0.0, 3.0),
            front: -Vector3::unit_z(),
            up: Vector3::unit_y(),

        }
    }

    pub fn get_view(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.pos, self.pos + self.front, self.up)
    }

    pub fn rotate<T: Into<Quaternion<f32>>>(&mut self, rot_mat: T) {
        let rot_mat: Quaternion<f32> = rot_mat.into();
        self.front = rot_mat * self.front;
    }
}