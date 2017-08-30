pub struct VertexDef {
    position: [f32; 3],
    normal: [f32; 3],
    color: [f32; 3],
}

impl_vertex!(VertexDef, position, normal, color);
