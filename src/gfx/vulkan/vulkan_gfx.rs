use super::errors::*;
use super::shaders::basic::{fs, vs};
use super::vulkan_gfx_loop::VulkanGfxLoop;
use super::vulkan_plane::VulkanPlane;
use cgmath::{Point3, Vector3};
use gfx::{Gfx, GfxLoop, Vertex};
use gfx::camera_geometry::CameraGeometry;
use gfx::plane::Plane;
use gfx::window::Window;
use std::f32;
use std::sync::Arc;
use vulkano::device::{Device, Queue};
use vulkano::framebuffer::Subpass;
use vulkano::image::SwapchainImage;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::swapchain::Swapchain;
use vulkano::sync::now;

pub struct VulkanGfx {
    device: Arc<Device>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    queue: Arc<Queue>,
}

impl VulkanGfx {
    pub fn new(
        device: Arc<Device>,
        swapchain: Arc<Swapchain>,
        images: Vec<Arc<SwapchainImage>>,
        queue: Arc<Queue>,
    ) -> VulkanGfx {
        VulkanGfx {
            device: device,
            swapchain: swapchain,
            images: images,
            queue: queue,
        }
    }
}

impl Gfx for VulkanGfx {
    fn new_plane(&mut self, _origin: Point3<f32>, _up: Vector3<f32>) -> Box<Plane> {
        Box::new(VulkanPlane::new())
    }

    fn new_loop(
        &self,
        camera: Box<CameraGeometry>,
        window: &Arc<Box<Window>>,
    ) -> Result<Box<GfxLoop>> {
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
            window.clone(),
            window.dimensions()?,
            false,
            Some(Box::new(now(self.device.clone()))),
            None,
            render_pass,
            pipeline,
        );

        Ok(Box::new(gfx_loop))
    }
}
