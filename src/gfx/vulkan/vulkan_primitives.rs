use super::vulkan_primitive::VulkanPrimitive;

pub struct VulkanPrimitives {
    pub primitives: Vec<VulkanPrimitive>,
}

impl VulkanPrimitives {
    pub fn new(primitives: Vec<VulkanPrimitive>) -> VulkanPrimitives {
        VulkanPrimitives { primitives: primitives }
    }
}
