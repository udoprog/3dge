use super::shaders::basic::{fs, vs};
use super::vulkan_gfx::VulkanGfx;
use super::vulkan_gfx_loop_builder::VulkanGfxLoopBuilder;
use super::vulkano_win_window::VulkanoWinWindow;
use gfx::Vertex;
use gfx::Window;
use gfx::errors::*;
use std::sync::Arc;
use std::sync::mpsc;
use vulkano::device::{self, Device};
use vulkano::framebuffer::Subpass;
use vulkano::instance::{self, Instance};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::swapchain::{PresentMode, SurfaceTransform, Swapchain};
use vulkano_win::{self, VkSurfaceBuild};
use winit;

pub struct VulkanGfxInstance {
    instance: Arc<Instance>,
}

impl VulkanGfxInstance {
    pub fn new() -> Result<VulkanGfxInstance> {
        let instance = {
            let extensions = vulkano_win::required_extensions();
            Instance::new(None, &extensions, None)?
        };

        Ok(VulkanGfxInstance { instance: instance })
    }

    /// Backend-specific implementation for building windows.
    pub fn build_window(&self, events_loop: &winit::EventsLoop) -> Result<Window> {
        let window = winit::WindowBuilder::new()
            .with_title("3dge")
            .build_vk_surface(events_loop, self.instance.clone())?;

        Ok(VulkanoWinWindow::new(window))
    }

    pub fn build_gfx(&self, window: Arc<Window>) -> Result<(VulkanGfx, VulkanGfxLoopBuilder)> {
        let (send, recv) = mpsc::channel();

        let physical = instance::PhysicalDevice::enumerate(&self.instance)
            .next()
            .ok_or(ErrorKind::NoSupportedDevice)?;

        let dimensions = window.dimensions()?;

        let queue = physical
            .queue_families()
            .find(|&q| {
                q.supports_graphics() && window.surface().is_supported(q).unwrap_or(false)
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
            let caps = window.surface().capabilities(physical)?;

            let alpha = caps.supported_composite_alpha.iter().next().ok_or(
                ErrorKind::NoCompositeAlphaCapability,
            )?;

            let format = caps.supported_formats[0].0;

            Swapchain::new(
                device.clone(),
                window.surface().clone(),
                caps.min_image_count,
                format,
                dimensions,
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

        let gfx = VulkanGfx::new(
            send,
            device.clone(),
            window.clone(),
            swapchain.clone(),
            images.clone(),
            queue.clone(),
        );

        let vs = vs::Shader::load(device.clone())?;
        let fs = fs::Shader::load(device.clone())?;

        let render_pass = Arc::new(single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )?);

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
            .build(device.clone())?);

        let gfx_loop_builder = VulkanGfxLoopBuilder::new(
            recv,
            device.clone(),
            swapchain.clone(),
            images.clone(),
            queue.clone(),
            window.clone(),
            window.dimensions()?,
            None,
            render_pass,
            pipeline,
        );

        Ok((gfx, gfx_loop_builder))
    }
}
