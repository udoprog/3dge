use super::errors::*;
use super::geometry_data::GeometryData;
use super::geometry_entry::GeometryEntry;
use super::vulkan_window::VulkanWindow;
use gfx::Gfx;
use gfx::camera_geometry::CameraGeometry;
use gfx::camera_object::CameraObject;
use gfx::errors as gfx;
use gfx::geometry_object::GeometryObject;
use std::sync::{Arc, RwLock};
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::device::{Device, Queue};
use vulkano::image::SwapchainImage;
use vulkano::swapchain::Swapchain;

#[derive(Clone)]
pub struct VulkanGfx {
    camera: Arc<RwLock<Option<Box<CameraGeometry>>>>,
    device: Arc<Device>,
    window: Arc<Box<VulkanWindow>>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    queue: Arc<Queue>,
    geometry: Arc<RwLock<GeometryData>>,
}

impl VulkanGfx {
    pub fn new(
        camera: Arc<RwLock<Option<Box<CameraGeometry>>>>,
        device: Arc<Device>,
        window: Arc<Box<VulkanWindow>>,
        swapchain: Arc<Swapchain>,
        images: Vec<Arc<SwapchainImage>>,
        queue: Arc<Queue>,
        geometry: Arc<RwLock<GeometryData>>,
    ) -> VulkanGfx {
        VulkanGfx {
            camera: camera,
            device: device,
            window: window,
            swapchain: swapchain,
            images: images,
            queue: queue,
            geometry: geometry,
        }
    }
}

impl Gfx for VulkanGfx {
    fn clear(&self) -> Result<()> {
        *self.camera.write().map_err(|_| gfx::Error::PoisonError)? = None;

        self.geometry
            .write()
            .map_err(|_| gfx::Error::PoisonError)?
            .clear();

        Ok(())
    }

    fn set_camera(&self, camera_object: &CameraObject) -> Result<()> {
        let mut camera = self.camera.write().map_err(|_| gfx::Error::PoisonError)?;
        *camera = Some(camera_object.geometry());
        Ok(())
    }

    fn register_geometry(&self, geometry_object: &GeometryObject) -> gfx::Result<()> {
        let g = geometry_object.geometry();

        let buffer = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage::all(),
            g.read_lock()?.vertices()?.iter().cloned(),
        )?;

        let entry = GeometryEntry::new(buffer, g);

        self.geometry
            .write()
            .map_err(|_| gfx::Error::PoisonError)?
            .push(entry);

        Ok(())
    }

    fn clone_boxed(&self) -> Box<Gfx> {
        Box::new(Clone::clone(self))
    }
}
