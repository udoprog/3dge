use self::shaders::basic::fs;
use self::shaders::basic::vs;
use self::vertex::Vertex;
use super::{Gfx, GfxLoop};
use super::errors::vulkan::*;
use super::window::Window;
use cgmath::{Matrix4, Point3, Rad, SquareMatrix, Vector3};
use std::f32;

use std::mem;
use std::sync::Arc;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBuffer, DynamicState};
use vulkano::descriptor::descriptor_set::{DescriptorSet, PersistentDescriptorSet};
use vulkano::device::{self, Device, Queue};
use vulkano::framebuffer::{self, Framebuffer, FramebufferBuilder, Subpass};
use vulkano::image::SwapchainImage;
use vulkano::instance::{self, Instance};
use vulkano::pipeline::{self, GraphicsPipeline};
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain::{self, AcquireError, PresentMode, SurfaceTransform, Swapchain};
use vulkano::sync::GpuFuture;
use vulkano::sync::now;
use vulkano_win::{self, VkSurfaceBuild};

use winit;

pub mod errors;
mod shaders;
mod vertex;

pub type UniformData = vs::ty::Data;
pub type Ds = DescriptorSet + Send + ::std::marker::Sync;
pub type Rp = framebuffer::RenderPassAbstract + Send + ::std::marker::Sync;
pub type Pl = pipeline::GraphicsPipelineAbstract + Send + ::std::marker::Sync;
pub type Fb = framebuffer::FramebufferAbstract + Send + ::std::marker::Sync;

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
    pub(crate) fn build_window(&self, events_loop: &winit::EventsLoop) -> Result<VulkanoWinWindow> {
        let window = winit::WindowBuilder::new()
            .with_title("3dge")
            .build_vk_surface(events_loop, self.instance.clone())?;

        Ok(VulkanoWinWindow { window: window })
    }

    pub(crate) fn build_gfx<W>(&self, window: &W) -> Result<VulkanGfx>
    where
        W: Window + VulkanWindow,
    {
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

            println!("caps = {:?}", caps);

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

        let proj = ::cgmath::perspective(
            Rad(f32::consts::FRAC_PI_2),
            {
                let d = images[0].dimensions();
                d[0] as f32 / d[1] as f32
            },
            0.01,
            100.0,
        );

        let view = Matrix4::look_at(
            Point3::new(0.3, 0.3, 1.0),
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );

        let scale = Matrix4::from_scale(0.5);

        let uniform = UniformData {
            world: <Matrix4<f32> as SquareMatrix>::identity().into(),
            view: (view * scale).into(),
            proj: proj.into(),
        };

        Ok(VulkanGfx {
            device: device,
            swapchain: swapchain,
            images: images,
            queue: queue,
            uniform: uniform,
        })
    }
}

pub struct VulkanGfx {
    device: Arc<Device>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    queue: Arc<Queue>,
    uniform: UniformData,
}

impl VulkanGfx {}

impl Gfx for VulkanGfx {
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

        let gfx_loop = VulkanGfxLoop {
            device: self.device.clone(),
            swapchain: self.swapchain.clone(),
            images: self.images.clone(),
            queue: self.queue.clone(),
            window: window.clone(),
            dimensions: window.dimensions()?,
            recreate_swapchain: false,
            previous_frame_end: Some(Box::new(now(self.device.clone()))),
            uniform: self.uniform.clone(),
            uniform_buffer_set: uniform_buffer_set,
            vertex_buffer: vertex_buffer,
            framebuffers: None,
            render_pass: render_pass,
            pipeline: pipeline,
        };

        Ok(Box::new(gfx_loop))
    }
}

struct VulkanGfxLoop {
    device: Arc<Device>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    queue: Arc<Queue>,
    window: Arc<Box<Window>>,
    dimensions: [u32; 2],
    recreate_swapchain: bool,
    previous_frame_end: Option<Box<GpuFuture>>,
    uniform: UniformData,
    uniform_buffer_set: Arc<Ds>,
    vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    framebuffers: Option<Vec<Arc<Fb>>>,
    render_pass: Arc<Rp>,
    pipeline: Arc<Pl>,
}

impl GfxLoop for VulkanGfxLoop {
    fn tick(&mut self) -> Result<()> {
        if let Some(ref mut previous_frame_end) = self.previous_frame_end {
            previous_frame_end.cleanup_finished();
        }

        if self.recreate_swapchain {
            self.dimensions = self.window.dimensions()?;

            let (new_swapchain, new_images) =
                match self.swapchain.recreate_with_dimension(self.dimensions) {
                    Ok(r) => r,
                    Err(swapchain::SwapchainCreationError::UnsupportedDimensions) => {
                        return Ok(());
                    }
                    Err(err) => panic!("{:?}", err),
                };

            mem::replace(&mut self.swapchain, new_swapchain);
            mem::replace(&mut self.images, new_images);

            self.framebuffers = None;
            self.recreate_swapchain = false;
        }

        if self.framebuffers.is_none() {
            let new_framebuffers = {
                let mut out: Vec<Arc<Fb>> = Vec::new();

                for image in &self.images {
                    let fb = Framebuffer::start(self.render_pass.clone())
                        .add(image.clone())
                        .and_then(FramebufferBuilder::build)
                        .map(Arc::new)?;

                    out.push(fb);
                }

                out
            };

            mem::replace(&mut self.framebuffers, Some(new_framebuffers));
        }

        let (image_num, acquire_future) =
            match swapchain::acquire_next_image(self.swapchain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    self.recreate_swapchain = true;
                    return Ok(());
                }
                Err(err) => panic!("{:?}", err),
            };

        /// Fill up with draw-calls.
        let cb = AutoCommandBufferBuilder::primary_one_time_submit(
            self.device.clone(),
            self.queue.family(),
        )?;

        let cb = cb.begin_render_pass(
            self.framebuffers.as_ref().unwrap()[image_num].clone(),
            false,
            vec![[0.0, 0.0, 0.0, 1.0].into()],
        )?;

        let cb = cb.draw(self.pipeline.clone(),
                  DynamicState {
                      line_width: None,
                      viewports: Some(vec![Viewport {
                          origin: [0.0, 0.0],
                          dimensions: [self.dimensions[0] as f32, self.dimensions[1] as f32],
                          depth_range: 0.0 .. 1.0,
                      }]),
                      scissors: None,
                  },
                  vec![self.vertex_buffer.clone()], (self.uniform_buffer_set.clone()), ())?;

        let cb = cb.end_render_pass()?;
        let cb = cb.build()?;

        let future = if let Some(previous_frame_end) = self.previous_frame_end.take() {
            Box::new(previous_frame_end
                .join(acquire_future)
                .then_execute(self.queue.clone(), cb)?
                .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
                .then_signal_fence_and_flush()?) as Box<GpuFuture>
        } else {
            // _should_ not happen, but if it does, just execute the current command buffer.
            Box::new(cb.execute(self.queue.clone())?
                .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
                .then_signal_fence_and_flush()?) as Box<GpuFuture>
        };

        self.previous_frame_end = Some(future);
        Ok(())
    }

    fn translate_world(&mut self, translation: &Matrix4<f32>) -> Result<()> {
        let world: Matrix4<f32> = self.uniform.world.into();
        self.uniform.world = (translation * world).into();

        let uniform_buffer = CpuAccessibleBuffer::<UniformData>::from_data(
            self.device.clone(),
            BufferUsage::all(),
            self.uniform.clone(),
        )?;

        self.uniform_buffer_set =
            Arc::new(PersistentDescriptorSet::start(self.pipeline.clone(), 0)
                .add_buffer(uniform_buffer.clone())?
                .build()?);

        Ok(())
    }
}

pub(crate) struct VulkanoWinWindow {
    window: vulkano_win::Window,
}

impl Window for VulkanoWinWindow {
    fn dimensions(&self) -> Result<[u32; 2]> {
        let (width, height) = self.window.window().get_inner_size_pixels().ok_or(
            ErrorKind::NoWindowDimensions,
        )?;

        Ok([width, height])
    }
}

/// Trait for Vulkan-enabled windows.
///
/// In particular, these have surfaces associated with them.
impl VulkanWindow for VulkanoWinWindow {
    fn surface(&self) -> &Arc<swapchain::Surface> {
        self.window.surface()
    }
}

pub trait VulkanWindow {
    fn surface(&self) -> &Arc<swapchain::Surface>;
}
