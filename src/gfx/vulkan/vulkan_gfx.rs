use super::errors::*;
use super::geometry_data::GeometryData;
use super::shaders::basic::{fs, vs};
use super::vulkan_gfx_loop::VulkanGfxLoop;
use super::vulkan_plane::VulkanPlane;
use super::vulkan_window::VulkanWindow;
use cgmath::{Point3, Vector3};
use gfx::{Gfx, GfxLoop, Vertex};
use gfx::camera_geometry::CameraGeometry;
use gfx::errors as gfx;
use gfx::geometry::GeometryObject;
use gfx::plane::Plane;
use std::f32;
use std::sync::{Arc, RwLock};
use vulkano::device::{Device, Queue};
use vulkano::framebuffer::Subpass;
use vulkano::image::SwapchainImage;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::swapchain::Swapchain;
use vulkano::sync::now;

#[derive(Clone)]
pub struct VulkanGfx {
    device: Arc<Device>,
    window: Arc<Box<VulkanWindow>>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    queue: Arc<Queue>,
    geometry: Arc<RwLock<GeometryData>>,
}

impl VulkanGfx {
    pub fn new(
        device: Arc<Device>,
        window: Arc<Box<VulkanWindow>>,
        swapchain: Arc<Swapchain>,
        images: Vec<Arc<SwapchainImage>>,
        queue: Arc<Queue>,
    ) -> VulkanGfx {
        VulkanGfx {
            device: device,
            window: window,
            swapchain: swapchain,
            images: images,
            queue: queue,
            geometry: Arc::new(RwLock::new(GeometryData::new())),
        }
    }
}

impl Gfx for VulkanGfx {
    fn register_geometry(&mut self, geometry_object: &GeometryObject) -> gfx::Result<()> {
        self.geometry
            .write()
            .map_err(|_| gfx::Error::PoisonError)?
            .push(geometry_object.geometry());
        Ok(())
    }

    fn new_plane(&mut self, _origin: Point3<f32>, _up: Vector3<f32>) -> Box<Plane> {
        Box::new(VulkanPlane::new())
    }

    fn new_loop(&self, camera: Box<CameraGeometry>) -> Result<Box<GfxLoop>> {
        let vs = vs::Shader::load(self.device.clone())?;
        let fs = fs::Shader::load(self.device.clone())?;

        let render_pass = single_pass_renderpass!(
            self.device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: self.swapchain.format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )?;

        let render_pass = Arc::new(render_pass);

        let sub_pass = Subpass::from(render_pass.clone(), 0).ok_or(
            ErrorKind::NoSubpass,
        )?;

        let pipeline = Arc::new(GraphicsPipeline::start()
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(vs.main_entry_point(), ())
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fs.main_entry_point(), ())
            .render_pass(sub_pass)
            .build(self.device.clone())?);

        let gfx_loop = VulkanGfxLoop::new(
            camera,
            self.device.clone(),
            self.swapchain.clone(),
            self.images.clone(),
            self.queue.clone(),
            self.window.clone(),
            self.window.dimensions()?,
            false,
            Some(Box::new(now(self.device.clone()))),
            None,
            render_pass,
            pipeline,
            self.geometry.clone(),
        );

        Ok(Box::new(gfx_loop))
    }

    fn clone_boxed(&self) -> Box<Gfx> {
        Box::new(Clone::clone(self))
    }
}
