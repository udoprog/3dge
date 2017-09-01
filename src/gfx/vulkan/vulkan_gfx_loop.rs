use super::{UniformGlobal, UniformModel};
use super::shaders::basic::{fs, vs};
use super::vulkan_geometry::VulkanGeometry;
use super::vulkan_primitive::VulkanPrimitive;
use super::vulkan_primitives::VulkanPrimitives;
use cgmath::{Matrix4, Rad};
use cgmath::prelude::*;
use gfx::{GeometryId, Window};
use gfx::Vertex;
use gfx::camera_object::CameraObject;
use gfx::command::Command;
use gfx::errors::*;
use gfx::primitive::Primitive;
use image;
use std::collections::HashMap;
use std::f32;
use std::mem;
use std::sync::Arc;
use std::sync::mpsc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::device::{Device, Queue};
use vulkano::format::{self, Format};
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, FramebufferBuilder, Subpass};
use vulkano::framebuffer::RenderPassAbstract;
use vulkano::image::AttachmentImage;
use vulkano::image::Dimensions;
use vulkano::image::SwapchainImage;
use vulkano::image::immutable::ImmutableImage;
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::pipeline::viewport::Viewport;
use vulkano::sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode};
use vulkano::swapchain::{self, AcquireError, Swapchain};
use vulkano::sync::GpuFuture;

pub struct VulkanGfxLoopTicker {
    recv: mpsc::Receiver<Command>,
    window: Arc<Window>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
    /// complicated state
    pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
    render_pass: Arc<RenderPassAbstract + Send + Sync>,
    depth_buffer: Arc<AttachmentImage>,
    texture_sampler: Arc<Sampler>,
    debug_image: Arc<ImmutableImage<format::R8G8B8A8Srgb>>,
    /// Current registered geometry.
    visible: HashMap<GeometryId, VulkanGeometry>,
    /// Current camera.
    camera: Option<Box<CameraObject>>,
    /// previous frame
    previous_frame: Option<Box<GpuFuture>>,
    /// swapchains needs to be re-created (typically during re-size)
    recreate_swapchain: bool,
    /// last known window dimensions
    dimensions: [u32; 2],
    /// loaded framebuffers
    framebuffers: Option<Vec<Arc<FramebufferAbstract + Send + Sync>>>,
}

impl VulkanGfxLoopTicker {
    pub fn tick(&mut self) -> Result<()> {
        if let Some(ref mut previous_frame) = self.previous_frame {
            previous_frame.cleanup_finished();
        }

        if self.recreate_swapchain {
            info!("re-creating swapchain");
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
                let mut out: Vec<Arc<FramebufferAbstract + Sync + Send>> = Vec::new();

                for image in &self.images {
                    let fb = Framebuffer::start(self.render_pass.clone())
                        .add(image.clone())
                        .and_then(|b| b.add(self.depth_buffer.clone()))
                        .and_then(FramebufferBuilder::build)?;

                    out.push(Arc::new(fb));
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
            vec![[0.0, 0.0, 0.0, 1.0].into(), 1f32.into()],
        )?;

        let global_buffer = {
            let projection = ::cgmath::perspective(
                Rad(f32::consts::FRAC_PI_2),
                {
                    let d = self.dimensions;
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

            let global = UniformGlobal {
                camera: <Matrix4<f32> as SquareMatrix>::identity().into(),
                view: (view * scale).into(),
                projection: projection.into(),
            };

            CpuAccessibleBuffer::<UniformGlobal>::from_data(
                self.device.clone(),
                BufferUsage::all(),
                global,
            )?
        };

        let state = DynamicState {
            line_width: None,
            viewports: Some(vec![
                Viewport {
                    origin: [0.0, 0.0],
                    dimensions: [self.dimensions[0] as f32, self.dimensions[1] as f32],
                    depth_range: 0.0..1.0,
                },
            ]),
            scissors: None,
        };

        for entry in self.visible.values() {
            let VulkanGeometry {
                ref primitives,
                ref geometry,
            } = *entry;

            for p in &primitives.primitives {
                let VulkanPrimitive {
                    ref vertex_buffer,
                    ref index_buffer,
                    ref base_color_factor,
                    ref base_color_texture,
                    ref use_base_color_texture,
                    ..
                } = *p;

                let geometry = geometry.read_lock()?;
                let transformation = geometry.transformation()?;

                let model = UniformModel {
                    model: transformation.into(),
                    base_color_factor: *base_color_factor,
                    use_base_color_texture: *use_base_color_texture as u32,
                };

                let model_buffer =
                    CpuAccessibleBuffer::from_data(self.device.clone(), BufferUsage::all(), model)?;

                let set = Arc::new(PersistentDescriptorSet::start(self.pipeline.clone(), 0)
                    .add_buffer(global_buffer.clone())?
                    .add_buffer(model_buffer.clone())?
                    .build()?);

                let texture = Arc::new(PersistentDescriptorSet::start(self.pipeline.clone(), 1)
                    .add_sampled_image(base_color_texture.clone(), self.texture_sampler.clone())?
                    .build()?);

                cb = cb.draw_indexed(
                        self.pipeline.clone(),
                        state.clone(),
                        vec![vertex_buffer.clone()],
                        index_buffer.clone(),
                        (set, texture),
                        ()
                    )?;
            }
        }

        let cb = cb.end_render_pass()?;
        let cb = cb.build()?;

        let previous_frame = self.previous_frame.take().ok_or(
            ErrorKind::MissingPreviousFrame,
        )?;

        let previous_frame: Box<GpuFuture> =
            if let Some(update_future) = self.check_for_updates()? {
                Box::new(previous_frame.join(update_future))
            } else {
                Box::new(previous_frame)
            };

        self.previous_frame = Some(Box::new(previous_frame
            .join(acquire_future)
            .then_execute(self.queue.clone(), cb)?
            .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
            .then_signal_fence_and_flush()?));

        Ok(())
    }

    fn new_or_old_future<F: 'static + GpuFuture>(
        &mut self,
        existing: Option<Box<GpuFuture>>,
        future: F,
    ) -> Option<Box<GpuFuture>> {
        if let Some(existing) = existing {
            Some(Box::new(existing.join(future)) as Box<GpuFuture>)
        } else {
            Some(Box::new(future) as Box<GpuFuture>)
        }
    }

    fn process_command(&mut self, command: Command) -> Result<Option<Box<GpuFuture>>> {
        use self::Command::*;

        debug!("command: {:?}", command);

        let mut future = None;

        match command {
            ClearCamera => {
                self.camera = None;
            }
            SetCamera(camera) => {
                self.camera = Some(camera);
            }
            AddGeometry(geometry) => {
                let g = geometry.read_lock()?;

                let mut primitives = Vec::new();

                for p in g.primitives()?.primitives {
                    let Primitive {
                        vertices,
                        indices,
                        base_color_factor,
                        base_color_texture,
                        ..
                    } = p;

                    let vertex_buffer = CpuAccessibleBuffer::from_iter(
                        self.device.clone(),
                        BufferUsage::all(),
                        vertices.into_iter(),
                    )?;

                    let index_buffer = CpuAccessibleBuffer::from_iter(
                        self.device.clone(),
                        BufferUsage::all(),
                        indices.into_iter(),
                    )?;

                    let (base_color_texture, use_base_color_texture) =
                        if let Some(base_color_texture) = base_color_texture {
                            let (width, height) = base_color_texture.dimensions;

                            info!("{:?}: loaded color texture ({}, {})", g.id(), width, height);

                            let (image, tex_future) = ImmutableImage::from_iter(
                                base_color_texture.image_data.into_iter(),
                                Dimensions::Dim2d {
                                    width: width,
                                    height: height,
                                },
                                format::R8G8B8A8Srgb,
                                self.queue.clone(),
                            )?;

                            future = self.new_or_old_future(future, tex_future);
                            (image, true)
                        } else {
                            (self.debug_image.clone(), false)
                        };

                    primitives.push(VulkanPrimitive::new(
                        vertex_buffer,
                        index_buffer,
                        base_color_factor.into(),
                        base_color_texture,
                        use_base_color_texture,
                    ));
                }

                let primitives = VulkanPrimitives::new(primitives);

                self.visible.insert(
                    g.id(),
                    VulkanGeometry::new(geometry.clone_geometry(), primitives),
                );
            }
        }

        Ok(future)
    }

    /// Check for geometry updates.
    fn check_for_updates(&mut self) -> Result<Option<Box<GpuFuture>>> {
        let mut future = None;

        loop {
            match self.recv.try_recv() {
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => return Err(ErrorKind::Disconnected.into()),
                Ok(command) => {
                    if let Some(added_future) = self.process_command(command)? {
                        future = self.new_or_old_future(future, added_future);
                    }
                }
            }
        }

        Ok(future)
    }
}

pub struct VulkanGfxLoop {
    recv: mpsc::Receiver<Command>,
    window: Arc<Window>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<SwapchainImage>>,
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
        }
    }

    pub fn into_ticker(self) -> Result<VulkanGfxLoopTicker> {
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

        let pipeline: Arc<GraphicsPipelineAbstract + Send + Sync> =
            Arc::new(GraphicsPipeline::start()
                .vertex_input(SingleBufferDefinition::<Vertex>::new())
                .vertex_shader(vs.main_entry_point(), ())
                .triangle_list()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fs.main_entry_point(), ())
                .depth_stencil_simple_depth()
                .render_pass(sub_pass)
                .build(self.device.clone())?);

        let depth_buffer =
            AttachmentImage::transient(self.device.clone(), dimensions, Format::D16Unorm)?;

        let dimensions = self.window.dimensions()?;

        let texture_sampler = Sampler::new(
            self.device.clone(),
            Filter::Linear,
            Filter::Linear,
            MipmapMode::Nearest,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            0.0,
            1.0,
            0.0,
            0.0,
        )?;

        let (debug_image, debug_tex_future) = {
            let image = image::load_from_memory_with_format(
                include_bytes!("debug_512x512.png"),
                image::ImageFormat::PNG,
            )?
                .to_rgba();

            let image_data = image.into_raw();

            ImmutableImage::from_iter(
                image_data.into_iter(),
                Dimensions::Dim2d {
                    width: 512,
                    height: 512,
                },
                format::R8G8B8A8Srgb,
                self.queue.clone(),
            )?
        };

        let previous_frame = Some(Box::new(debug_tex_future) as Box<GpuFuture>);

        return Ok(VulkanGfxLoopTicker {
            recv: self.recv,
            window: self.window,
            device: self.device,
            queue: self.queue,
            swapchain: self.swapchain,
            images: self.images,
            pipeline: pipeline,
            render_pass: render_pass,
            depth_buffer: depth_buffer,
            texture_sampler: texture_sampler,
            debug_image: debug_image,
            visible: HashMap::new(),
            camera: None,
            previous_frame: previous_frame,
            recreate_swapchain: false,
            dimensions: dimensions,
            framebuffers: None,
        });
    }
}
