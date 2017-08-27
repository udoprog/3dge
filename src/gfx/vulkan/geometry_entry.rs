use gfx::Vertex;
use gfx::geometry::Geometry;
use std::sync::Arc;
use vulkano::buffer::CpuAccessibleBuffer;

pub struct GeometryEntry {
    pub buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    pub geometry: Box<Geometry>,
}

impl GeometryEntry {
    pub fn new(
        buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
        geometry: Box<Geometry>,
    ) -> GeometryEntry {
        GeometryEntry {
            buffer: buffer,
            geometry: geometry,
        }
    }
}
