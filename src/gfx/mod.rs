#[cfg(feature = "gfx-vulkan")]
pub(crate) mod vulkan;
mod window;
pub mod errors;
pub mod geometry;
pub mod plane;

use self::errors::*;
use self::plane::Plane;
pub use self::window::Window;
use cgmath::{Matrix4, Point3, Vector3};
use std::sync::Arc;

pub trait Gfx {
    /// Create a new infinite plane, with it's normal defined according to the given `up`.
    fn new_plane(&mut self, origin: Point3<f32>, up: Vector3<f32>) -> Box<Plane>;

    fn new_loop(&self, window: &Arc<Box<Window>>) -> Result<Box<GfxLoop>>;
}

pub trait GfxLoop {
    fn tick(&mut self) -> Result<()>;

    fn translate_world(&mut self, translation: &Matrix4<f32>) -> Result<()>;
}

pub enum GfxBuiltInShader {
    /// The simplest possible shader. Gets red color on screen.
    Basic,
}

pub enum GfxShader {
    Default,
    BuiltIn(GfxBuiltInShader),
}
