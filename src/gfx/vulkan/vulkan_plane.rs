use gfx::errors as gfx;
use gfx::plane::Plane;
use texture::Texture;

pub struct VulkanPlane {}

impl VulkanPlane {
    pub fn new() -> VulkanPlane {
        VulkanPlane {}
    }
}

impl Plane for VulkanPlane {
    fn bind_texture(&mut self, _texture: &Texture) -> gfx::Result<()> {
        Ok(())
    }
}
