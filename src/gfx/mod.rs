#[cfg(feature = "gfx-vulkan")]
pub mod vulkan;

mod command;
pub mod camera_accessor;
pub mod camera_object;
pub mod color;
pub mod errors;
pub mod geometry;
pub mod geometry_object;
mod geometry_id;
pub mod vertices;

pub use self::geometry_id::GeometryId;

pub type Vertex = [f32; 3];
pub type Normal = [f32; 3];
pub type Color = [f32; 4];

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
