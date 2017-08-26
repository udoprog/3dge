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
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

pub trait Gfx {
    /// Create a new infinite plane, with it's normal defined according to the given `up`.
    fn new_plane(&mut self, origin: Point3<f32>, up: Vector3<f32>) -> Box<Plane>;

    fn new_loop(
        &self,
        camera: Box<CameraGeometry>,
        window: &Arc<Box<Window>>,
    ) -> Result<Box<GfxLoop>>;
}

pub trait GfxLoop {
    fn register_geometry(&mut self, geometry_object: &GeometryObject);

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
