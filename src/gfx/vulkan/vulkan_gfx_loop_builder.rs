use super::vulkan_gfx_loop::VulkanGfxLoop;
use gfx::Window;
use gfx::command::Command;
use gfx::errors::*;
use std::sync::Arc;
use std::sync::mpsc;
use vulkano::device::{self, Device};
use vulkano::instance::{self, Instance};
use vulkano::swapchain::{PresentMode, SurfaceTransform, Swapchain};

pub struct VulkanGfxLoopBuilder {
    recv: mpsc::Receiver<Command>,
    instance: Arc<Instance>,
    window: Arc<Window>,
}

impl VulkanGfxLoopBuilder {
    pub fn new(
        recv: mpsc::Receiver<Command>,
        instance: Arc<Instance>,
        window: Arc<Window>,
    ) -> VulkanGfxLoopBuilder {
        VulkanGfxLoopBuilder {
            recv: recv,
            instance: instance,
            window: window,
        }
    }

    pub fn into_loop(self) -> Result<VulkanGfxLoop> {
        let physical = instance::PhysicalDevice::enumerate(&self.instance)
            .next()
            .ok_or(ErrorKind::NoSupportedDevice)?;

        let queue = physical
            .queue_families()
            .find(|&q| {
                q.supports_graphics() && self.window.surface().is_supported(q).unwrap_or(false)
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
            let caps = self.window.surface().capabilities(physical)?;

            let alpha = caps.supported_composite_alpha.iter().next().ok_or(
                ErrorKind::NoCompositeAlphaCapability,
            )?;

            let format = caps.supported_formats[0].0;

            Swapchain::new(
                device.clone(),
                self.window.surface().clone(),
                caps.min_image_count,
                format,
                self.window.dimensions()?,
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

        Ok(VulkanGfxLoop::new(
            self.recv,
            self.window,
            device,
            queue,
            swapchain,
            images,
        ))
    }
}
