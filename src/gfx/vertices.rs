use super::Vertex;

pub struct Vertices {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Vertices {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Vertices {
        Vertices {
            vertices: vertices,
            indices: indices,
        }
    }
}
