use super::{Normal, Vertex};

pub struct Vertices {
    pub vertices: Vec<Vertex>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u32>,
}

impl Vertices {
    pub fn new(vertices: Vec<Vertex>, normals: Vec<Normal>, indices: Vec<u32>) -> Vertices {
        Vertices {
            vertices: vertices,
            normals: normals,
            indices: indices,
        }
    }
}
