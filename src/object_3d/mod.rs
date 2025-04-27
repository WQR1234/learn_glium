pub mod cube;
pub mod sphere;

use cgmath::{Matrix4, Rad, SquareMatrix, Vector3};
use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub(crate) position: [f32; 3],
    pub(crate) tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);


pub enum Object3dKind {
    Cube,
    Sphere
}



pub struct Object3d {
    pub model: Matrix4<f32>,

    pub kind: Object3dKind,

    pub shader_name: String,

    pub texture_id: Option<usize>,

    pub color: Option<[f32; 3]>

}

impl Object3d {
    pub fn new(kind: Object3dKind) -> Self {
        Self {
            model: Matrix4::identity(),
            kind,
            shader_name: "".to_string(),
            texture_id: None,
            color: None
        }
    }

    pub fn translate(&mut self, vec: Vector3<f32>) {
        let trans = Matrix4::from_translation(vec);
        self.model = self.model * trans;
    }

    pub fn rotate<T: Into<Rad<f32>>>(&mut self, axis: Vector3<f32>, angle: T) {
        let rot = Matrix4::from_axis_angle(axis, angle);
        self.model = self.model * rot;
    }

    pub fn scale(&mut self, vec: Vector3<f32>) {
        let scale = Matrix4::from_nonuniform_scale(vec.x, vec.y, vec.z);
        self.model = self.model * scale;
    }
}







