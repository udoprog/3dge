use super::geometry_data::GeometryData;
use super::geometry_entry::GeometryEntry;
use gfx::Window;
use gfx::camera_geometry::CameraGeometry;
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
    camera: Arc<RwLock<Option<Box<CameraGeometry>>>>,
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
        camera: Arc<RwLock<Option<Box<CameraGeometry>>>>,
        device: Arc<Device>,
        window: Arc<Window>,
        swapchain: Arc<Swapchain>,
        images: Vec<Arc<SwapchainImage>>,
        queue: Arc<Queue>,
        geometry: Arc<RwLock<GeometryData>>,
    ) -> VulkanGfx {
        VulkanGfx {
            send: send,
            camera: camera,
            device: device,
            window: window,
            swapchain: swapchain,
            images: images,
            queue: queue,
            geometry: geometry,
        }
    }

    pub fn clear(&self) -> Result<()> {
        *self.camera.write().map_err(|_| ErrorKind::PoisonError)? = None;

        self.geometry
            .write()
            .map_err(|_| ErrorKind::PoisonError)?
            .clear();

        Ok(())
    }

    pub fn set_camera(&self, camera_object: &CameraObject) -> Result<()> {
        let mut camera = self.camera.write().map_err(|_| ErrorKind::PoisonError)?;
        *camera = Some(camera_object.geometry());
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
