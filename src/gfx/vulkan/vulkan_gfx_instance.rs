use super::vulkan_gfx::VulkanGfx;
use super::vulkan_gfx_loop_builder::VulkanGfxLoopBuilder;
use super::vulkano_win_window::VulkanoWinWindow;
use gfx::Window;
use gfx::errors::*;
use std::sync::Arc;
use std::sync::mpsc;
use vulkano::instance::Instance;
use vulkano_win::{self, VkSurfaceBuild};
use winit;

pub struct VulkanGfxInstance {
    instance: Arc<Instance>,
}

impl VulkanGfxInstance {
    pub fn new() -> Result<VulkanGfxInstance> {
        let instance = {
            let extensions = vulkano_win::required_extensions();
            Instance::new(None, &extensions, None)?
        };

        Ok(VulkanGfxInstance { instance: instance })
    }

    /// Backend-specific implementation for building windows.
    pub fn build_window(&self, events_loop: &winit::EventsLoop) -> Result<Window> {
        let window = winit::WindowBuilder::new()
            .with_title("3dge")
            .build_vk_surface(events_loop, self.instance.clone())?;

        Ok(VulkanoWinWindow::new(window))
    }

    pub fn build_gfx(&self, window: Arc<Window>) -> Result<(VulkanGfx, VulkanGfxLoopBuilder)> {
        let (send, recv) = mpsc::channel();
        let gfx_loop_builder =
            VulkanGfxLoopBuilder::new(recv, self.instance.clone(), window.clone());
        let gfx = VulkanGfx::new(send);
        Ok((gfx, gfx_loop_builder))
    }
}
