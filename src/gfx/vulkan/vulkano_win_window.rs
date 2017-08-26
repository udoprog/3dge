use super::errors::*;
use super::vulkan_window::VulkanWindow;
use gfx::window::Window;
use std::sync::Arc;
use vulkano::swapchain;
use vulkano_win;

pub struct VulkanoWinWindow {
    window: vulkano_win::Window,
}

impl VulkanoWinWindow {
    pub fn new(window: vulkano_win::Window) -> VulkanoWinWindow {
        VulkanoWinWindow { window: window }
    }
}

impl Window for VulkanoWinWindow {
    fn dimensions(&self) -> Result<[u32; 2]> {
        let (width, height) = self.window.window().get_inner_size_pixels().ok_or(
            ErrorKind::NoWindowDimensions,
        )?;

        Ok([width, height])
    }
}

/// Trait for Vulkan-enabled windows.
///
/// In particular, these have surfaces associated with them.
impl VulkanWindow for VulkanoWinWindow {
    fn surface(&self) -> &Arc<swapchain::Surface> {
        self.window.surface()
    }
}
