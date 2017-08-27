#[cfg(feature = "gfx-vulkan")]
pub(crate) mod vulkan;
mod window;
pub mod errors;
pub mod geometry;
pub mod plane;
pub mod camera_geometry;
pub mod rectangle;
pub mod color;

use self::camera_geometry::CameraGeometry;
use self::errors::*;
use self::geometry::GeometryObject;
use self::plane::Plane;
pub use self::window::Window;
use cgmath::{Point3, Vector3};
use std::marker;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

pub trait Gfx: marker::Sync + marker::Send {
    /// Create a new infinite plane, with it's normal defined according to the given `up`.
    fn new_plane(&mut self, origin: Point3<f32>, up: Vector3<f32>) -> Box<Plane>;

    /// Register a new piece of geometry that should be rendered.
    fn register_geometry(&mut self, geometry_object: &GeometryObject) -> Result<()>;

    /// Create a new loop.
    fn new_loop(&self, camera: Box<CameraGeometry>) -> Result<Box<GfxLoop>>;

    /// Clone the current gfx handle.
    fn clone_boxed(&self) -> Box<Gfx>;
}

impl Clone for Box<Gfx> {
    fn clone(&self) -> Self {
        self.clone_boxed()
    }
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
