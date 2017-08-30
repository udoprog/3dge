#[cfg(feature = "gfx-vulkan")]
pub mod vulkan;

mod command;
pub mod camera_accessor;
pub mod camera_object;
pub mod color;
pub mod errors;
pub mod geometry;
pub mod geometry_object;
pub mod rectangle;
mod geometry_id;

pub use self::geometry_id::GeometryId;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

#[cfg(feature = "gfx-vulkan")]
pub use self::vulkan::*;

pub enum GfxBuiltInShader {
    /// The simplest possible shader. Gets red color on screen.
    Basic,
}

pub enum GfxShader {
    Default,
    BuiltIn(GfxBuiltInShader),
}
