use super::{UniformGlobal, UniformModel};
use super::geometry_entry::GeometryEntry;
use super::shaders::basic::{fs, vs};
use cgmath::{Matrix4, Rad};
use cgmath::prelude::*;
use gfx::{GeometryId, Window};
use gfx::camera_object::CameraObject;
use gfx::command::Command;
use gfx::errors::*;
use gfx::vertices::Vertices;
use std::collections::HashMap;
use std::f32;
use std::mem;
use std::sync::Arc;
use std::sync::mpsc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBuffer, DynamicState};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::device::{Device, Queue};
use vulkano::format::Format;
use vulkano::framebuffer::{Framebuffer, FramebufferBuilder, Subpass};
use vulkano::image::AttachmentImage;
use vulkano::image::SwapchainImage;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::vertex::TwoBuffersDefinition;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain::{self, AcquireError, Swapchain};
use vulkano::sync::GpuFuture;

pub struct VulkanGfxLoop {
    recv: mpsc::Receiver<Command>,
    window: Arc<Window>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    /// Current registered geometry.
    visible: HashMap<GeometryId, GeometryEntry>,
    /// Current camera.
    camera: Option<Box<CameraObject>>,
}

impl VulkanGfxLoop {
    pub fn new(
        recv: mpsc::Receiver<Command>,
        window: Arc<Window>,
        device: Arc<Device>,
        queue: Arc<Queue>,
        swapchain: Arc<Swapchain>,
        images: Vec<Arc<SwapchainImage>>,
    ) -> VulkanGfxLoop {
        VulkanGfxLoop {
            recv: recv,
            window: window,
            device: device,
            queue: queue,
            swapchain: swapchain,
            images: images,
            visible: HashMap::new(),
            camera: None,
        }
    }

    fn process_command(&mut self, command: Command) -> Result<()> {
        use self::Command::*;

        debug!("command: {:?}", command);

        match command {
            ClearCamera => {
                self.camera = None;
            }
            SetCamera(camera) => {
                self.camera = Some(camera);
            }
            AddGeometry(geometry) => {
                let g = geometry.read_lock()?;

                let Vertices {
                    vertices,
                    normals,
                    indices,
                } = g.vertices()?;

                let vertex_buffer = CpuAccessibleBuffer::from_iter(
                    self.device.clone(),
                    BufferUsage::all(),
                    vertices.into_iter(),
                )?;

                let normal_buffer = CpuAccessibleBuffer::from_iter(
                    self.device.clone(),
                    BufferUsage::all(),
                    normals.into_iter(),
                )?;

                let index_buffer = CpuAccessibleBuffer::from_iter(
                    self.device.clone(),
                    BufferUsage::all(),
                    indices.into_iter(),
                )?;

                self.visible.insert(
                    g.id(),
                    GeometryEntry::new(
                        vertex_buffer,
                        normal_buffer,
                        index_buffer,
                        geometry.clone_geometry(),
                    ),
                );
            }
        }

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

    pub fn run(&mut self) -> Result<()> {
        let dimensions = self.window.dimensions()?;

        let vs = vs::Shader::load(self.device.clone())?;
        let fs = fs::Shader::load(self.device.clone())?;

        let render_pass = Arc::new(single_pass_renderpass!(
            self.device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: self.swapchain.format(),
                    samples: 1,
                },
                depth: {
                    load: Clear,
                    store: DontCare,
                    format: Format::D16Unorm,
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {depth}
            }
        )?);

        let sub_pass = Subpass::from(render_pass.clone(), 0).ok_or(
            ErrorKind::NoSubpass,
        )?;

        let pipeline = Arc::new(GraphicsPipeline::start()
            .vertex_input(TwoBuffersDefinition::new())
            .vertex_shader(vs.main_entry_point(), ())
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fs.main_entry_point(), ())
            .depth_stencil_simple_depth()
            .render_pass(sub_pass)
            .build(self.device.clone())?);

        let depth_buffer =
            AttachmentImage::transient(self.device.clone(), dimensions, Format::D16Unorm)?;

        let mut framebuffers = None;
        let mut dimensions = self.window.dimensions()?;
        let mut recreate_swapchain = false;
        let mut previous_frame_end: Option<Box<GpuFuture>> = None;

        loop {
            self.check_for_updates()?;

            if let Some(ref mut previous_frame_end) = previous_frame_end.as_mut() {
                previous_frame_end.cleanup_finished();
            }

            if recreate_swapchain {
                println!("re-creating swapchain");
                dimensions = self.window.dimensions()?;

                let (new_swapchain, new_images) =
                    match self.swapchain.recreate_with_dimension(dimensions) {
                        Ok(r) => r,
                        Err(swapchain::SwapchainCreationError::UnsupportedDimensions) => {
                            return Ok(());
                        }
                        Err(err) => panic!("{:?}", err),
                    };

                mem::replace(&mut self.swapchain, new_swapchain);
                mem::replace(&mut self.images, new_images);

                framebuffers = None;
                recreate_swapchain = false;
            }

            if framebuffers.is_none() {
                let new_framebuffers = {
                    let mut out = Vec::new();

                    for image in &self.images {
                        let fb = Framebuffer::start(render_pass.clone())
                            .add(image.clone())
                            .and_then(|b| b.add(depth_buffer.clone()))
                            .and_then(FramebufferBuilder::build)
                            .map(Arc::new)?;

                        out.push(fb);
                    }

                    out
                };

                mem::replace(&mut framebuffers, Some(new_framebuffers));
            }

            let (image_num, acquire_future) =
                match swapchain::acquire_next_image(self.swapchain.clone(), None) {
                    Ok(r) => r,
                    Err(AcquireError::OutOfDate) => {
                        recreate_swapchain = true;
                        continue;
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
                framebuffers.as_ref().unwrap()[image_num].clone(),
                false,
                vec![[0.0, 0.0, 0.0, 1.0].into(), 1f32.into()],
            )?;

            let uniform_buffer_set = {
                let projection = ::cgmath::perspective(
                    Rad(f32::consts::FRAC_PI_2),
                    {
                        let d = dimensions;
                        d[0] as f32 / d[1] as f32
                    },
                    0.01,
                    100.0,
                );

                let view = if let Some(ref mut camera) = self.camera {
                    camera.write_lock()?.view_transformation()?
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

                Arc::new(PersistentDescriptorSet::start(pipeline.clone(), 0)
                    .add_buffer(uniform_buffer.clone())?
                    .build()?)
            };

            let state = DynamicState {
                line_width: None,
                viewports: Some(vec![
                    Viewport {
                        origin: [0.0, 0.0],
                        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
                        depth_range: 0.0..1.0,
                    },
                ]),
                scissors: None,
            };

            for entry in self.visible.values() {
                let GeometryEntry {
                    ref vertex_buffer,
                    ref normal_buffer,
                    ref index_buffer,
                    ref geometry,
                } = *entry;

                let geometry = geometry.read_lock()?;
                let transformation = geometry.transformation()?;

                let model = UniformModel { model: transformation.into() };

                let uniform_buffer =
                    CpuAccessibleBuffer::from_data(self.device.clone(), BufferUsage::all(), model)?;

                let position = Arc::new(PersistentDescriptorSet::start(pipeline.clone(), 0)
                    .add_buffer(uniform_buffer.clone())?
                    .build()?);

                cb = cb.draw_indexed(
                        pipeline.clone(),
                        state.clone(),
                        (vertex_buffer.clone(), normal_buffer.clone()),
                        index_buffer.clone(),
                        (uniform_buffer_set.clone(), position),
                        ()
                    )?;
            }

            let cb = cb.end_render_pass()?;
            let cb = cb.build()?;

            let future = if let Some(previous_frame_end) = previous_frame_end.take() {
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

            previous_frame_end = Some(future);
        }

        Ok(())
    }
}
