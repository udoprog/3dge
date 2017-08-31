#[cfg(feature = "gfx-vulkan")]
pub mod vulkan;

pub mod primitive;
pub mod primitives;
mod command;
pub mod camera_accessor;
pub mod camera_object;
pub mod color;
pub mod errors;
pub mod geometry;
pub mod geometry_object;
pub mod geometry_accessor;
mod geometry_id;
pub mod vertices;

pub use self::geometry_id::GeometryId;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coord: [f32; 2],
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
