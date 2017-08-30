#[cfg(feature = "gfx-vulkan")]
pub mod vulkan;
pub mod errors;
pub mod geometry;
pub mod geometry_object;
pub mod camera_geometry;
pub mod rectangle;
pub mod color;
pub mod camera_object;
mod command;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}


#[cfg(feature = "gfx-vulkan")]
pub use self::vulkan::vulkan_gfx::VulkanGfx as Gfx;
#[cfg(feature = "gfx-vulkan")]
pub use self::vulkan::vulkan_gfx_loop::VulkanGfxLoop as GfxLoop;
#[cfg(feature = "gfx-vulkan")]
pub use self::vulkan::vulkan_gfx_loop_builder::VulkanGfxLoopBuilder as GfxLoopBuilder;
#[cfg(feature = "gfx-vulkan")]
pub use self::vulkan::vulkano_win_window::VulkanoWinWindow as Window;

pub enum GfxBuiltInShader {
    /// The simplest possible shader. Gets red color on screen.
    Basic,
}

pub enum GfxShader {
    Default,
    BuiltIn(GfxBuiltInShader),
}
