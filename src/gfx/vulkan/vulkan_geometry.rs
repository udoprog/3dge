use super::vulkan_primitives::VulkanPrimitives;
use gfx::geometry::Geometry;

pub struct VulkanGeometry {
    pub geometry: Box<Geometry>,
    pub primitives: VulkanPrimitives,
}

impl VulkanGeometry {
    pub fn new(geometry: Box<Geometry>, primitives: VulkanPrimitives) -> VulkanGeometry {
        VulkanGeometry {
            geometry: geometry,
            primitives: primitives,
        }
    }
}
