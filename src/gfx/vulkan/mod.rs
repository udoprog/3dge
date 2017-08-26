pub mod errors;
mod shaders;
mod vertex;
mod vulkano_win_window;
mod vulkan_window;
mod vulkan_plane;
mod vulkan_gfx_instance;
mod vulkan_gfx_loop;
mod vulkan_gfx;

use self::shaders::basic::vs;
pub use self::vulkan_gfx_instance::VulkanGfxInstance;
use vulkano::descriptor::descriptor_set::DescriptorSet;
use vulkano::framebuffer;
use vulkano::pipeline;

pub type UniformData = vs::ty::Data;
pub type Ds = DescriptorSet + Send + ::std::marker::Sync;
pub type Rp = framebuffer::RenderPassAbstract + Send + ::std::marker::Sync;
pub type Pl = pipeline::GraphicsPipelineAbstract + Send + ::std::marker::Sync;
pub type Fb = framebuffer::FramebufferAbstract + Send + ::std::marker::Sync;
