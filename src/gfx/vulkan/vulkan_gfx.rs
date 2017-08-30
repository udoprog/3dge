use gfx::Window;
use gfx::camera_object::CameraObject;
use gfx::command::Command;
use gfx::errors::*;
use gfx::geometry_object::GeometryObject;
use std::sync::Arc;
use std::sync::mpsc;
use vulkano::device::{Device, Queue};
use vulkano::image::SwapchainImage;
use vulkano::swapchain::Swapchain;

#[derive(Clone)]
pub struct VulkanGfx {
    send: mpsc::Sender<Command>,
    device: Arc<Device>,
    window: Arc<Window>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    queue: Arc<Queue>,
}

impl VulkanGfx {
    pub fn new(
        send: mpsc::Sender<Command>,
        device: Arc<Device>,
        window: Arc<Window>,
        swapchain: Arc<Swapchain>,
        images: Vec<Arc<SwapchainImage>>,
        queue: Arc<Queue>,
    ) -> VulkanGfx {
        VulkanGfx {
            send: send,
            device: device,
            window: window,
            swapchain: swapchain,
            images: images,
            queue: queue,
        }
    }

    pub fn clear(&self) -> Result<()> {
        self.send.send(Command::ClearCamera).map_err(
            |_| ErrorKind::SendError,
        )?;

        Ok(())
    }

    pub fn set_camera(&self, camera_object: &CameraObject) -> Result<()> {
        self.send
            .send(Command::SetCamera(camera_object.clone_camera_object()))
            .map_err(|_| ErrorKind::SendError)?;

        Ok(())
    }

    pub fn register_geometry(&self, geometry_object: &GeometryObject) -> Result<()> {
        self.send
            .send(Command::AddGeometry(geometry_object.geometry()))
            .map_err(|_| ErrorKind::SendError)?;
        Ok(())
    }
}
