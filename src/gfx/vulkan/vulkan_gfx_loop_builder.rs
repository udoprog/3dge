use super::{Pl, Rp};
use super::vulkan_gfx_loop::VulkanGfxLoop;
use gfx::GfxLoop;
use gfx::Window;
use gfx::command::Command;
use gfx::errors::*;
use std::sync::Arc;
use std::sync::mpsc;
use vulkano::device::{Device, Queue};
use vulkano::image::AttachmentImage;
use vulkano::image::SwapchainImage;
use vulkano::swapchain::Swapchain;

pub struct VulkanGfxLoopBuilder {
    recv: mpsc::Receiver<Command>,
    device: Arc<Device>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    queue: Arc<Queue>,
    window: Arc<Window>,
    dimensions: [u32; 2],
    render_pass: Arc<Rp>,
    pipeline: Arc<Pl>,
    depth_buffer: Arc<AttachmentImage>,
}

impl VulkanGfxLoopBuilder {
    pub fn new(
        recv: mpsc::Receiver<Command>,
        device: Arc<Device>,
        swapchain: Arc<Swapchain>,
        images: Vec<Arc<SwapchainImage>>,
        queue: Arc<Queue>,
        window: Arc<Window>,
        dimensions: [u32; 2],
        render_pass: Arc<Rp>,
        pipeline: Arc<Pl>,
        depth_buffer: Arc<AttachmentImage>,
    ) -> VulkanGfxLoopBuilder {
        VulkanGfxLoopBuilder {
            recv: recv,
            device: device,
            swapchain: swapchain,
            images: images,
            queue: queue,
            window: window,
            dimensions: dimensions,
            render_pass: render_pass,
            pipeline: pipeline,
            depth_buffer: depth_buffer,
        }
    }

    pub fn into_loop(self) -> Result<GfxLoop> {
        Ok(VulkanGfxLoop::new(
            self.recv,
            self.device,
            self.swapchain,
            self.images,
            self.queue,
            self.window,
            self.dimensions,
            self.render_pass,
            self.pipeline,
            self.depth_buffer,
        ))
    }
}
