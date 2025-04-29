use crate::object_3d::{Vertex};

pub const CUBE_SHAPE: &'static [Vertex] = &[
    Vertex { position:[-0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coords:[0.0, 0.0] },
    Vertex { position:[0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coords:[1.0, 0.0] },
    Vertex { position:[0.5, 0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coords:[1.0, 1.0] },
    Vertex { position:[0.5, 0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coords:[1.0, 1.0] },
    Vertex { position:[-0.5, 0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coords:[0.0, 1.0] },
    Vertex { position:[-0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0], tex_coords:[0.0, 0.0] },

    Vertex { position:[-0.5, -0.5, 0.5], normal: [0.0, 0.0, 1.0], tex_coords:[0.0, 0.0] },
    Vertex { position:[0.5, -0.5, 0.5], normal: [0.0, 0.0, 1.0], tex_coords:[1.0, 0.0] },
    Vertex { position:[0.5, 0.5, 0.5], normal: [0.0, 0.0, 1.0], tex_coords:[1.0, 1.0] },
    Vertex { position:[0.5, 0.5, 0.5], normal: [0.0, 0.0, 1.0], tex_coords:[1.0, 1.0] },
    Vertex { position:[-0.5, 0.5, 0.5], normal: [0.0, 0.0, 1.0], tex_coords:[0.0, 1.0] },
    Vertex { position:[-0.5, -0.5, 0.5], normal: [0.0, 0.0, 1.0], tex_coords:[0.0, 0.0] },

    Vertex { position:[-0.5, 0.5, 0.5], normal: [-1.0, 0.0, 0.0], tex_coords:[1.0, 0.0] },
    Vertex { position:[-0.5, 0.5, -0.5], normal: [-1.0, 0.0, 0.0], tex_coords:[1.0, 1.0] },
    Vertex { position:[-0.5, -0.5, -0.5], normal: [-1.0, 0.0, 0.0], tex_coords:[0.0, 1.0] },
    Vertex { position:[-0.5, -0.5, -0.5], normal: [-1.0, 0.0, 0.0], tex_coords:[0.0, 1.0] },
    Vertex { position:[-0.5, -0.5, 0.5], normal: [-1.0, 0.0, 0.0], tex_coords:[0.0, 0.0] },
    Vertex { position:[-0.5, 0.5, 0.5], normal: [-1.0, 0.0, 0.0], tex_coords:[1.0, 0.0] },

    Vertex { position:[0.5, 0.5, 0.5], normal: [1.0, 0.0, 0.0], tex_coords:[1.0, 0.0] },
    Vertex { position:[0.5, 0.5, -0.5], normal: [1.0, 0.0, 0.0], tex_coords:[1.0, 1.0] },
    Vertex { position:[0.5, -0.5, -0.5], normal: [1.0, 0.0, 0.0], tex_coords:[0.0, 1.0] },
    Vertex { position:[0.5, -0.5, -0.5], normal: [1.0, 0.0, 0.0], tex_coords:[0.0, 1.0] },
    Vertex { position:[0.5, -0.5, 0.5], normal: [1.0, 0.0, 0.0], tex_coords:[0.0, 0.0] },
    Vertex { position:[0.5, 0.5, 0.5], normal: [1.0, 0.0, 0.0], tex_coords:[1.0, 0.0] },

    Vertex { position:[-0.5, -0.5, -0.5], normal: [0.0, -1.0, 0.0], tex_coords:[0.0, 1.0] },
    Vertex { position:[0.5, -0.5, -0.5], normal: [0.0, -1.0, 0.0], tex_coords:[1.0, 1.0] },
    Vertex { position:[0.5, -0.5, 0.5], normal: [0.0, -1.0, 0.0], tex_coords:[1.0, 0.0] },
    Vertex { position:[0.5, -0.5, 0.5], normal: [0.0, -1.0, 0.0], tex_coords:[1.0, 0.0] },
    Vertex { position:[-0.5, -0.5, 0.5], normal: [0.0, -1.0, 0.0], tex_coords:[0.0, 0.0] },
    Vertex { position:[-0.5, -0.5, -0.5], normal: [0.0, -1.0, 0.0], tex_coords:[0.0, 1.0] },

    Vertex { position:[-0.5, 0.5, -0.5], normal: [0.0, 1.0, 0.0], tex_coords:[0.0, 1.0] },
    Vertex { position:[0.5, 0.5, -0.5], normal: [0.0, 1.0, 0.0], tex_coords:[1.0, 1.0] },
    Vertex { position:[0.5, 0.5, 0.5], normal: [0.0, 1.0, 0.0], tex_coords:[1.0, 0.0] },
    Vertex { position:[0.5, 0.5, 0.5], normal: [0.0, 1.0, 0.0], tex_coords:[1.0, 0.0] },
    Vertex { position:[-0.5, 0.5, 0.5], normal: [0.0, 1.0, 0.0], tex_coords:[0.0, 0.0] },
    Vertex { position:[-0.5, 0.5, -0.5], normal: [0.0, 1.0, 0.0], tex_coords:[0.0, 1.0] },

];



