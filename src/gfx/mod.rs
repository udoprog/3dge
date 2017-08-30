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

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    vertex: (f32, f32, f32),
}

impl From<[f32; 3]> for Vertex {
    fn from(value: [f32; 3]) -> Vertex {
        Vertex { vertex: (value[0], value[1], value[2]) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Normal {
    normal: (f32, f32, f32),
}

impl From<[f32; 3]> for Normal {
    fn from(value: [f32; 3]) -> Normal {
        Normal { normal: (value[0], value[1], value[2]) }
    }
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
