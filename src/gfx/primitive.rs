use super::{Normal, Vertex};
use std::fmt;

#[derive(Clone)]
pub struct Primitive {
    pub vertices: Vec<Vertex>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u32>,
}

impl Primitive {
    pub fn new(vertices: Vec<Vertex>, normals: Vec<Normal>, indices: Vec<u32>) -> Primitive {
        Primitive {
            vertices: vertices,
            normals: normals,
            indices: indices,
        }
    }
}

impl fmt::Debug for Primitive {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "Primitive {{ vertices: <{}>, normals: <{}>, indices: <{}> }}",
            self.vertices.len(),
            self.normals.len(),
            self.indices.len()
        )
    }
}
