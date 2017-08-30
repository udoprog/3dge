use super::{Fb, Pl, Rp};
use super::errors::*;
use super::geometry_data::GeometryData;
use super::vulkan_gfx_loop::VulkanGfxLoop;
use super::vulkan_window::VulkanWindow;
use gfx::{GfxLoop, GfxLoopBuilder};
use gfx::camera_geometry::CameraGeometry;
use std::sync::{Arc, RwLock};
use vulkano::device::{Device, Queue};
use vulkano::image::SwapchainImage;
use vulkano::swapchain::Swapchain;

pub struct VulkanGfxLoopBuilder {
    camera: Arc<RwLock<Option<Box<CameraGeometry>>>>,
    device: Arc<Device>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    queue: Arc<Queue>,
    window: Arc<Box<VulkanWindow>>,
    dimensions: [u32; 2],
    framebuffers: Option<Vec<Arc<Fb>>>,
    render_pass: Arc<Rp>,
    pipeline: Arc<Pl>,
    geometry: Arc<RwLock<GeometryData>>,
}

impl VulkanGfxLoopBuilder {
    pub fn new(
        camera: Arc<RwLock<Option<Box<CameraGeometry>>>>,
        device: Arc<Device>,
        swapchain: Arc<Swapchain>,
        images: Vec<Arc<SwapchainImage>>,
        queue: Arc<Queue>,
        window: Arc<Box<VulkanWindow>>,
        dimensions: [u32; 2],
        framebuffers: Option<Vec<Arc<Fb>>>,
        render_pass: Arc<Rp>,
        pipeline: Arc<Pl>,
        geometry: Arc<RwLock<GeometryData>>,
    ) -> VulkanGfxLoopBuilder {
        VulkanGfxLoopBuilder {
            camera: camera,
            device: device,
            swapchain: swapchain,
            images: images,
            queue: queue,
            window: window,
            dimensions: dimensions,
            framebuffers: framebuffers,
            render_pass: render_pass,
            pipeline: pipeline,
            geometry: geometry,
        }
    }
}

impl GfxLoopBuilder for VulkanGfxLoopBuilder {
    fn into_loop(&self) -> Result<Box<GfxLoop>> {
        Ok(Box::new(VulkanGfxLoop::new(
            self.camera.clone(),
            self.device.clone(),
            self.swapchain.clone(),
            self.images.clone(),
            self.queue.clone(),
            self.window.clone(),
            self.dimensions.clone(),
            self.framebuffers.clone(),
            self.render_pass.clone(),
            self.pipeline.clone(),
            self.geometry.clone(),
        )))
    }
}
