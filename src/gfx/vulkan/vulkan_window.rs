use std::sync::Arc;
use vulkano::swapchain;

pub trait VulkanWindow {
    fn surface(&self) -> &Arc<swapchain::Surface>;
}
