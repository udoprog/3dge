use gfx::errors::*;
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

    pub fn dimensions(&self) -> Result<[u32; 2]> {
        let (width, height) = self.window.window().get_inner_size_pixels().ok_or(
            ErrorKind::NoWindowDimensions,
        )?;

        Ok([width, height])
    }

    pub fn surface(&self) -> &Arc<swapchain::Surface> {
        self.window.surface()
    }
}
