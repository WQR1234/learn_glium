use std::sync::LazyLock;
use crate::object_3d::{Vertex};


fn generate_sphere_vertices_indices(radius: f32, latitude_count: usize, longitude_count: usize) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices: Vec<Vertex> = Vec::new();

    for lat in 0..=latitude_count {
        let theta = lat as f32 * std::f32::consts::PI / latitude_count as f32;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let v = lat as f32 / latitude_count as f32;

        for lon in 0..=longitude_count {
            let phi = lon as f32 * 2.0 * std::f32::consts::PI / longitude_count as f32;
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();

            let x = radius * sin_theta * cos_phi;
            let y = radius * cos_theta;
            let z = radius * sin_theta * sin_phi;

            let u = lon as f32 / longitude_count as f32;

            vertices.push(Vertex {position: [x, y, z], tex_coords: [u, v]});

        }
    }

    let mut indices: Vec<u16> = Vec::new();
    for lat in 0..latitude_count {
        for lon in 0..longitude_count {
            let first = (lat as u16) * (longitude_count + 1) as u16 + lon as u16;
            let second = first + (longitude_count + 1) as u16;

            indices.push(first);
            indices.push(second);
            indices.push(first + 1);

            indices.push(second);
            indices.push(second + 1);
            indices.push(first + 1);


        }
    }

    (vertices, indices)
}


pub static SPHERE_SHAPE_INDEX: LazyLock<(Vec<Vertex>, Vec<u16>)> = LazyLock::new(|| generate_sphere_vertices_indices(1.0, 20, 20));
