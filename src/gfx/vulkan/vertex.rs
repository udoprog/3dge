#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

impl_vertex!(Vertex, position, color);
