use super::{Ds, Fb, Pl, Rp, UniformData};
use super::errors::*;
use super::vertex::Vertex;
use cgmath::Matrix4;
use gfx::GfxLoop;
use gfx::window::Window;
use std::f32;
use std::mem;
use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBuffer, DynamicState};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::device::{Device, Queue};
use vulkano::framebuffer::{Framebuffer, FramebufferBuilder};
use vulkano::image::SwapchainImage;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain::{self, AcquireError, Swapchain};
use vulkano::sync::GpuFuture;

pub struct VulkanGfxLoop {
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

impl VulkanGfxLoop {
    pub fn new(
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
    ) -> VulkanGfxLoop {
        VulkanGfxLoop {
            device: device,
            swapchain: swapchain,
            images: images,
            queue: queue,
            window: window,
            dimensions: dimensions,
            recreate_swapchain: recreate_swapchain,
            previous_frame_end: previous_frame_end,
            uniform: uniform,
            uniform_buffer_set: uniform_buffer_set,
            vertex_buffer: vertex_buffer,
            framebuffers: framebuffers,
            render_pass: render_pass,
            pipeline: pipeline,
        }
    }
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
