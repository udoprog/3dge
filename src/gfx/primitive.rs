use super::Vertex;
use super::color::Color;
use std::fmt;
use texture::Texture;

#[derive(Clone)]
pub struct Primitive {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub base_color_factor: Color,
    pub base_color_texture: Option<Texture>,
}

impl Primitive {
    pub fn new(
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        base_color_factor: Color,
        base_color_texture: Option<Texture>,
    ) -> Primitive {
        Primitive {
            vertices: vertices,
            indices: indices,
            base_color_factor: base_color_factor,
            base_color_texture: base_color_texture,
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
