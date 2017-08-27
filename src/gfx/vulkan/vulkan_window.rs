use gfx::window::Window;
use std::sync::Arc;
use vulkano::swapchain;

pub trait VulkanWindow: Window {
    fn surface(&self) -> &Arc<swapchain::Surface>;
}
