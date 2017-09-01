use gfx::Vertex;
use std::sync::Arc;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::format;
use vulkano::image::immutable::ImmutableImage;

pub struct VulkanPrimitive {
    pub vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    pub index_buffer: Arc<CpuAccessibleBuffer<[u32]>>,
    pub base_color_factor: [f32; 4],
    pub base_color_texture: Arc<ImmutableImage<format::R8G8B8A8Srgb>>,
    pub use_base_color_texture: bool,
}

impl VulkanPrimitive {
    pub fn new(
        vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
        index_buffer: Arc<CpuAccessibleBuffer<[u32]>>,
        base_color_factor: [f32; 4],
        base_color_texture: Arc<ImmutableImage<format::R8G8B8A8Srgb>>,
        use_base_color_texture: bool,
    ) -> VulkanPrimitive {
        VulkanPrimitive {
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            base_color_factor: base_color_factor,
            base_color_texture: base_color_texture,
            use_base_color_texture: use_base_color_texture,
        }
    }
}
