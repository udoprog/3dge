#[cfg(feature = "gfx-vulkan")]
pub(crate) mod vulkan;
mod window;
pub mod errors;

use self::errors::*;
pub use self::window::Window;
use cgmath::Matrix4;
use std::sync::Arc;

pub trait Gfx {
    fn new_loop(&self, window: &Arc<Box<Window>>) -> Result<Box<GfxLoop>>;
}

pub trait GfxLoop {
    fn tick(&mut self) -> Result<()>;

    fn rotate_camera(&mut self, rotation: &Matrix4<f32>) -> Result<()>;
}

pub enum GfxBuiltInShader {
    /// The simplest possible shader. Gets red color on screen.
    Basic,
}

pub enum GfxShader {
    Default,
    BuiltIn(GfxBuiltInShader),
}
