use super::geometry_data::GeometryData;
use super::geometry_entry::GeometryEntry;
use gfx::Window;
use gfx::camera_object::CameraObject;
use gfx::command::Command;
use gfx::errors::*;
use gfx::geometry_object::GeometryObject;
use std::sync::{Arc, RwLock};
use std::sync::mpsc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
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
    geometry: Arc<RwLock<GeometryData>>,
}

impl VulkanGfx {
    pub fn new(
        send: mpsc::Sender<Command>,
        device: Arc<Device>,
        window: Arc<Window>,
        swapchain: Arc<Swapchain>,
        images: Vec<Arc<SwapchainImage>>,
        queue: Arc<Queue>,
        geometry: Arc<RwLock<GeometryData>>,
    ) -> VulkanGfx {
        VulkanGfx {
            send: send,
            device: device,
            window: window,
            swapchain: swapchain,
            images: images,
            queue: queue,
            geometry: geometry,
        }
    }

    pub fn clear(&self) -> Result<()> {
        self.send.send(Command::ClearCamera).map_err(
            |_| ErrorKind::SendError,
        )?;

        self.geometry
            .write()
            .map_err(|_| ErrorKind::PoisonError)?
            .clear();

        Ok(())
    }

    pub fn set_camera(&self, camera_object: &CameraObject) -> Result<()> {
        self.send
            .send(Command::SetCamera(camera_object.clone_camera_object()))
            .map_err(|_| ErrorKind::SendError)?;

        Ok(())
    }

    pub fn register_geometry(&self, geometry_object: &GeometryObject) -> Result<()> {
        let g = geometry_object.geometry();

        let buffer = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage::all(),
            g.read_lock()?.vertices()?.iter().cloned(),
        )?;

        let entry = GeometryEntry::new(buffer, g);

        self.geometry
            .write()
            .map_err(|_| ErrorKind::PoisonError)?
            .push(entry);

        Ok(())
    }
}
