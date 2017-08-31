use gfx::{Normal, Vertex};
use std::sync::Arc;
use vulkano::buffer::CpuAccessibleBuffer;

pub struct VulkanPrimitive {
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    pub normal_buffer: Arc<CpuAccessibleBuffer<[Normal]>>,
    pub index_buffer: Arc<CpuAccessibleBuffer<[u32]>>,
}

impl VulkanPrimitive {
    pub fn new(
        vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
        normal_buffer: Arc<CpuAccessibleBuffer<[Normal]>>,
        index_buffer: Arc<CpuAccessibleBuffer<[u32]>>,
    ) -> VulkanPrimitive {
        VulkanPrimitive {
            vertex_buffer: vertex_buffer,
            normal_buffer: normal_buffer,
            index_buffer: index_buffer,
        }
    }
}
