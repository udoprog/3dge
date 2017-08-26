use super::UniformData;
use super::errors::*;
use super::shaders::basic::{fs, vs};
use super::vertex::Vertex;
use super::vulkan_gfx_loop::VulkanGfxLoop;
use super::vulkan_plane::VulkanPlane;
use cgmath::{Point3, Vector3};
use gfx::{Gfx, GfxLoop};
use gfx::plane::Plane;
use gfx::window::Window;
use std::f32;
use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
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
    uniform: UniformData,
}

impl VulkanGfx {
    pub fn new(
        device: Arc<Device>,
        swapchain: Arc<Swapchain>,
        images: Vec<Arc<SwapchainImage>>,
        queue: Arc<Queue>,
        uniform: UniformData,
    ) -> VulkanGfx {
        VulkanGfx {
            device: device,
            swapchain: swapchain,
            images: images,
            queue: queue,
            uniform: uniform,
        }
    }
}

impl Gfx for VulkanGfx {
    fn new_plane(&mut self, _origin: Point3<f32>, _up: Vector3<f32>) -> Box<Plane> {
        Box::new(VulkanPlane::new())
    }

    fn new_loop(&self, window: &Arc<Box<Window>>) -> Result<Box<GfxLoop>> {
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

        let uniform_buffer = CpuAccessibleBuffer::<UniformData>::from_data(
            self.device.clone(),
            BufferUsage::all(),
            self.uniform.clone(),
        )?;

        let uniform_buffer_set = Arc::new(PersistentDescriptorSet::start(pipeline.clone(), 0)
            .add_buffer(uniform_buffer.clone())?
            .build()?);

        /// FIXME: hardcoded triangle, vertex buffers and textures through API.
        let v1 = Vertex {
            position: [-0.5, -0.5],
            color: [1.0, 1.0, 1.0],
        };
        let v2 = Vertex {
            position: [0.0, 0.5],
            color: [0.5, 1.0, 0.0],
        };
        let v3 = Vertex {
            position: [0.5, -0.25],
            color: [0.0, 0.0, 1.0],
        };

        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage::all(),
            vec![v1, v2, v3].iter().cloned(),
        )?;

        let gfx_loop = VulkanGfxLoop::new(
            self.device.clone(),
            self.swapchain.clone(),
            self.images.clone(),
            self.queue.clone(),
            window.clone(),
            window.dimensions()?,
            false,
            Some(Box::new(now(self.device.clone()))),
            self.uniform.clone(),
            uniform_buffer_set,
            vertex_buffer,
            None,
            render_pass,
            pipeline,
        );

        Ok(Box::new(gfx_loop))
    }
}
