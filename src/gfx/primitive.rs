use super::Vertex;
use std::fmt;
use texture::Texture;

#[derive(Clone)]
pub struct Primitive {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub color_texture: Option<Texture>,
}

impl Primitive {
    pub fn new(
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        color_texture: Option<Texture>,
    ) -> Primitive {
        Primitive {
            vertices: vertices,
            indices: indices,
            color_texture: color_texture,
        }
    }
}

impl fmt::Debug for Primitive {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "Primitive {{ vertices: <{}>, indices: <{}> }}",
            self.vertices.len(),
            self.indices.len()
        )
    }
}
