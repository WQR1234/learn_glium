use cgmath::{Matrix4, Rad, SquareMatrix, Vector3};
use glium::implement_vertex;

#[derive(Copy, Clone)]
struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

struct VertexIndexArray {
    vertex_array: &'static Vec<Vertex>,
    index_array: &'static Vec<u16>,
}

