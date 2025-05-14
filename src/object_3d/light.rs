use super::{Object3d, Object3dKind};

pub struct Light {
    position: [f32; 3],

    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],

    object: Object3d,
}

impl Light {
    pub fn new(position: [f32; 3], 
        ambient: [f32; 3], diffuse: [f32; 3], specular: [f32; 3], 
        kind: Object3dKind) -> Self {
        let mut object = Object3d::new(kind);
        object.translate(cgmath::Vector3::from(position));
        object.scale(cgmath::Vector3::new(0.2, 0.2, 0.2));
        object.shader_name = "light".to_string();
        Self {
            position,
            ambient,
            diffuse,
            specular,
            object,
        }
    }

    pub fn translate(&mut self, vec: [f32; 3]) {
        self.object.translate(vec.into());
        self.position[0] += vec[0];
        self.position[1] += vec[1];
        self.position[2] += vec[2];
    }

    pub fn set_position(&mut self, position: [f32; 3]) {
        let vec = cgmath::Vector3::new(position[0] - self.position[0], position[1] - self.position[1], position[2] - self.position[2]);
        self.object.translate(cgmath::Vector3::from(vec));
        self.position = position;
    }

    pub fn get_position(&self) -> &[f32; 3] {
        &self.position
    }

    // pub fn get_model(&self) -> &cgmath::Matrix4<f32> {
    //     &self.object.model
    // }

    pub fn get_object(&self) -> &Object3d {
        &self.object
    }

    
}

pub struct DirectionalLight {
    pub direction: [f32; 3],

    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
}

impl DirectionalLight {
    pub fn new(direction: [f32; 3], ambient: [f32; 3], diffuse: [f32; 3], specular: [f32; 3]) -> DirectionalLight {
        Self {
            direction,
            ambient,
            diffuse,
            specular
        }
    }
}