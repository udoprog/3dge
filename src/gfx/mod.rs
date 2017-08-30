#[cfg(feature = "gfx-vulkan")]
pub(crate) mod vulkan;
mod window;
pub mod errors;
pub mod geometry;
pub mod geometry_object;
pub mod camera_geometry;
pub mod rectangle;
pub mod color;
pub mod camera_object;

use self::camera_object::CameraObject;
use self::errors::*;
use self::geometry_object::GeometryObject;
pub use self::window::Window;
use std::marker;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

pub trait Gfx: marker::Sync + marker::Send {
    /// Clear graphics.
    fn clear(&self) -> Result<()>;

    /// Set the camera geometry.
    fn set_camera(&self, camera_geometry: &CameraObject) -> Result<()>;

    /// Register a new piece of geometry that should be rendered.
    fn register_geometry(&self, geometry_object: &GeometryObject) -> Result<()>;

    /// Clone the current gfx handle.
    fn clone_boxed(&self) -> Box<Gfx>;
}

impl Clone for Box<Gfx> {
    fn clone(&self) -> Self {
        self.clone_boxed()
    }
}

pub trait GfxLoopBuilder: marker::Sync + marker::Send {
    /// Build loop.
    fn into_loop(&self) -> Result<Box<GfxLoop>>;
}

pub trait GfxLoop {
    fn tick(&mut self) -> Result<()>;
}

pub enum GfxBuiltInShader {
    /// The simplest possible shader. Gets red color on screen.
    Basic,
}

pub enum GfxShader {
    Default,
    BuiltIn(GfxBuiltInShader),
}
