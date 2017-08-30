use super::{Fb, Pl, Rp, UniformGlobal, UniformModel};
use super::geometry_data::GeometryData;
use cgmath::{Matrix4, Rad};
use cgmath::prelude::*;
use gfx::Vertex;
use gfx::Window;
use gfx::camera_geometry::CameraGeometry;
use gfx::command::Command;
use gfx::errors::*;
use std::f32;
use std::mem;
use std::sync::{Arc, RwLock};
use std::sync::mpsc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBuffer, DynamicState};
use vulkano::descriptor::descriptor_set::DescriptorSet;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::device::{Device, Queue};
use vulkano::framebuffer::{Framebuffer, FramebufferBuilder};
use vulkano::image::SwapchainImage;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain::{self, AcquireError, Swapchain};
use vulkano::sync::GpuFuture;
use vulkano::sync::now;

pub type SyncDescriptorSet = DescriptorSet + Send + ::std::marker::Sync;

pub struct VulkanGfxLoop {
    recv: mpsc::Receiver<Command>,
    camera: Arc<RwLock<Option<Box<CameraGeometry>>>>,
    device: Arc<Device>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    queue: Arc<Queue>,
    window: Arc<Window>,
    dimensions: [u32; 2],
    framebuffers: Option<Vec<Arc<Fb>>>,
    render_pass: Arc<Rp>,
    pipeline: Arc<Pl>,
    geometry: Arc<RwLock<GeometryData>>,
    /// if the swapchain needs to be re-created.
    recreate_swapchain: bool,
    /// future associated with the previous frame.
    /// use for synchronizing rendering of subsequent frames.
    previous_frame_end: Option<Box<GpuFuture>>,
}

impl VulkanGfxLoop {
    pub fn new(
        recv: mpsc::Receiver<Command>,
        camera: Arc<RwLock<Option<Box<CameraGeometry>>>>,
        device: Arc<Device>,
        swapchain: Arc<Swapchain>,
        images: Vec<Arc<SwapchainImage>>,
        queue: Arc<Queue>,
        window: Arc<Window>,
        dimensions: [u32; 2],
        framebuffers: Option<Vec<Arc<Fb>>>,
        render_pass: Arc<Rp>,
        pipeline: Arc<Pl>,
        geometry: Arc<RwLock<GeometryData>>,
    ) -> VulkanGfxLoop {
        VulkanGfxLoop {
            recv: recv,
            camera: camera,
            device: device.clone(),
            swapchain: swapchain,
            images: images,
            queue: queue,
            window: window,
            dimensions: dimensions,
            framebuffers: framebuffers,
            render_pass: render_pass,
            pipeline: pipeline,
            geometry: geometry,
            recreate_swapchain: false,
            previous_frame_end: Some(Box::new(now(device.clone()))),
        }
    }

    fn create_global(&self) -> Result<Arc<SyncDescriptorSet>> {
        let projection = ::cgmath::perspective(
            Rad(f32::consts::FRAC_PI_2),
            {
                let d = self.dimensions;
                d[0] as f32 / d[1] as f32
            },
            0.01,
            100.0,
        );

        let view =
            if let Some(ref camera) = *self.camera.read().map_err(|_| ErrorKind::PoisonError)? {
                camera.view_transformation()?
            } else {
                <Matrix4<f32> as SquareMatrix>::identity()
            };

        let scale = Matrix4::from_scale(1.0);

        let uniform = UniformGlobal {
            camera: <Matrix4<f32> as SquareMatrix>::identity().into(),
            view: (view * scale).into(),
            projection: projection.into(),
        };

        let uniform_buffer = CpuAccessibleBuffer::<UniformGlobal>::from_data(
            self.device.clone(),
            BufferUsage::all(),
            uniform,
        )?;

        let uniform_buffer_set = Arc::new(PersistentDescriptorSet::start(self.pipeline.clone(), 0)
            .add_buffer(uniform_buffer.clone())?
            .build()?);

        Ok(Arc::new(uniform_buffer_set))
    }

    fn create_geometry(
        &self,
    ) -> Result<Vec<(Arc<CpuAccessibleBuffer<[Vertex]>>, Arc<SyncDescriptorSet>)>> {
        let mut out: Vec<(Arc<CpuAccessibleBuffer<[Vertex]>>, Arc<SyncDescriptorSet>)> = Vec::new();

        let geometry = self.geometry.read().map_err(|_| ErrorKind::PoisonError)?;

        for entry in &geometry.entries {
            let buffer = entry.buffer.clone();
            let geometry = entry.geometry.read_lock()?;

            let transformation = geometry.transformation()?;

            let model = UniformModel { model: transformation.into() };

            let uniform_buffer =
                CpuAccessibleBuffer::from_data(self.device.clone(), BufferUsage::all(), model)?;

            let ds = Arc::new(PersistentDescriptorSet::start(self.pipeline.clone(), 0)
                .add_buffer(uniform_buffer.clone())?
                .build()?);

            out.push((buffer, ds));
        }

        Ok(out)
    }

    fn process_command(&mut self, _command: Command) -> Result<()> {
        // do nothing right now
        Ok(())
    }

    /// Check for geometry updates.
    fn check_for_updates(&mut self) -> Result<()> {
        loop {
            match self.recv.try_recv() {
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => return Err(ErrorKind::Disconnected.into()),
                Ok(command) => self.process_command(command)?,
            }
        }

        Ok(())
    }

    pub fn tick(&mut self) -> Result<()> {
        self.check_for_updates()?;

        if let Some(ref mut previous_frame_end) = self.previous_frame_end {
            previous_frame_end.cleanup_finished();
        }

        if self.recreate_swapchain {
            println!("re-creating swapchain");
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
                Err(err) => {
                    return Err(err.into());
                }
            };

        /// Fill up with draw-calls.
        let mut cb = AutoCommandBufferBuilder::primary_one_time_submit(
            self.device.clone(),
            self.queue.family(),
        )?;

        cb = cb.begin_render_pass(
            self.framebuffers.as_ref().unwrap()[image_num].clone(),
            false,
            vec![[0.0, 0.0, 0.0, 1.0].into()],
        )?;

        let uniform_buffer_set = self.create_global()?;

        let geometry = self.create_geometry()?;

        for (buffer, position) in geometry {
            cb = cb.draw(self.pipeline.clone(),
            DynamicState {
                line_width: None,
                viewports: Some(vec![Viewport {
                    origin: [0.0, 0.0],
                    dimensions: [self.dimensions[0] as f32, self.dimensions[1] as f32],
                    depth_range: 0.0 .. 1.0,
                }]),
                scissors: None,
            },
            vec![buffer], (uniform_buffer_set.clone(), position), ())?;
        }

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
}
