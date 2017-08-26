use super::errors::*;
use super::vulkan_gfx::VulkanGfx;
use super::vulkan_window::VulkanWindow;
use super::vulkano_win_window::VulkanoWinWindow;
use gfx::window::Window;
use std::sync::Arc;
use vulkano::device::{self, Device};
use vulkano::instance::{self, Instance};
use vulkano::swapchain::{PresentMode, SurfaceTransform, Swapchain};
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
    pub(crate) fn build_window(&self, events_loop: &winit::EventsLoop) -> Result<VulkanoWinWindow> {
        let window = winit::WindowBuilder::new()
            .with_title("3dge")
            .build_vk_surface(events_loop, self.instance.clone())?;

        Ok(VulkanoWinWindow::new(window))
    }

    pub(crate) fn build_gfx<W>(&self, window: &W) -> Result<VulkanGfx>
    where
        W: Window + VulkanWindow,
    {
        let physical = instance::PhysicalDevice::enumerate(&self.instance)
            .next()
            .ok_or(ErrorKind::NoSupportedDevice)?;

        let dimensions = window.dimensions()?;

        let queue = physical
            .queue_families()
            .find(|&q| {
                q.supports_graphics() && window.surface().is_supported(q).unwrap_or(false)
            })
            .ok_or(ErrorKind::NoQueueFamily)?;

        let (device, mut queues) = {
            let device_ext = device::DeviceExtensions {
                khr_swapchain: true,
                ..device::DeviceExtensions::none()
            };

            Device::new(
                physical,
                physical.supported_features(),
                &device_ext,
                [(queue, 0.5)].iter().cloned(),
            )?
        };

        let queue = queues.next().ok_or(ErrorKind::NoQueueAvailable)?;

        let (swapchain, images) = {
            let caps = window.surface().capabilities(physical)?;

            let alpha = caps.supported_composite_alpha.iter().next().ok_or(
                ErrorKind::NoCompositeAlphaCapability,
            )?;

            let format = caps.supported_formats[0].0;

            Swapchain::new(
                device.clone(),
                window.surface().clone(),
                caps.min_image_count,
                format,
                dimensions,
                1,
                caps.supported_usage_flags,
                &queue,
                SurfaceTransform::Identity,
                alpha,
                PresentMode::Fifo,
                true,
                None,
            )?
        };

        Ok(VulkanGfx::new(device, swapchain, images, queue))
    }
}
