use gfx::Vertex;
use std::sync::Arc;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::format;
use vulkano::image::immutable::ImmutableImage;

pub struct VulkanPrimitive {
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    pub index_buffer: Arc<CpuAccessibleBuffer<[u32]>>,
    pub color_texture: Arc<ImmutableImage<format::R8G8B8A8Srgb>>,
}

impl VulkanPrimitive {
    pub fn new(
        vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
        index_buffer: Arc<CpuAccessibleBuffer<[u32]>>,
        color_texture: Arc<ImmutableImage<format::R8G8B8A8Srgb>>,
    ) -> VulkanPrimitive {
        VulkanPrimitive {
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            color_texture: color_texture,
        }
    }
}
