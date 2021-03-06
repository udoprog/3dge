pub mod errors;
mod shaders;
pub mod vulkano_win_window;
mod vulkan_gfx_instance;
pub mod vulkan_gfx_loop;
pub mod vulkan_gfx_loop_builder;
pub mod vulkan_gfx;
mod vulkan_primitive;
mod vulkan_primitives;
mod vulkan_geometry;

use self::shaders::basic::vs;
pub use self::vulkan_gfx_instance::VulkanGfxInstance;
use gfx::Vertex;
use vulkano::framebuffer;
use vulkano::pipeline;

impl_vertex!(Vertex, position, normal, tex_coord);

pub type UniformGlobal = vs::ty::Global;
pub type UniformModel = vs::ty::Model;

pub type Rp = framebuffer::RenderPassAbstract + Send + ::std::marker::Sync;
pub type Pl = pipeline::GraphicsPipelineAbstract + Send + ::std::marker::Sync;
pub type Fb = framebuffer::FramebufferAbstract + Send + ::std::marker::Sync;

pub use self::vulkan_gfx::VulkanGfx as Gfx;
pub use self::vulkan_gfx_loop::VulkanGfxLoop as GfxLoop;
pub use self::vulkan_gfx_loop_builder::VulkanGfxLoopBuilder as GfxLoopBuilder;
pub use self::vulkano_win_window::VulkanoWinWindow as Window;
