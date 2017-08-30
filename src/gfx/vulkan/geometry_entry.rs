use gfx::{Normal, Vertex};
use gfx::geometry::Geometry;
use std::sync::Arc;
use vulkano::buffer::CpuAccessibleBuffer;

pub struct GeometryEntry {
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    pub normal_buffer: Arc<CpuAccessibleBuffer<[Normal]>>,
    pub index_buffer: Arc<CpuAccessibleBuffer<[u32]>>,
    pub geometry: Box<Geometry>,
}

impl GeometryEntry {
    pub fn new(
        vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
        normal_buffer: Arc<CpuAccessibleBuffer<[Normal]>>,
        index_buffer: Arc<CpuAccessibleBuffer<[u32]>>,
        geometry: Box<Geometry>,
    ) -> GeometryEntry {
        GeometryEntry {
            vertex_buffer: vertex_buffer,
            normal_buffer: normal_buffer,
            index_buffer: index_buffer,
            geometry: geometry,
        }
    }
}
