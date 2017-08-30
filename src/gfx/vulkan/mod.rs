pub mod errors;
mod shaders;
mod vertex;
pub mod vulkano_win_window;
mod vulkan_gfx_instance;
pub mod vulkan_gfx_loop;
pub mod vulkan_gfx_loop_builder;
pub mod vulkan_gfx;
mod geometry_data;
mod geometry_entry;

use self::shaders::basic::vs;
pub use self::vulkan_gfx_instance::VulkanGfxInstance;
use vulkano::framebuffer;
use vulkano::pipeline;

pub type UniformGlobal = vs::ty::Global;
pub type UniformModel = vs::ty::Model;
pub type Rp = framebuffer::RenderPassAbstract + Send + ::std::marker::Sync;
pub type Pl = pipeline::GraphicsPipelineAbstract + Send + ::std::marker::Sync;
pub type Fb = framebuffer::FramebufferAbstract + Send + ::std::marker::Sync;
