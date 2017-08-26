use super::errors::*;
use cgmath::Matrix4;

/// Describes the geomtry of some object on screen.
///
/// Needs to be thread-safe to be read by the render thread.
pub trait Geometry: Send + Sync {
    /// Get the homogenous transformation matrix for this geometry.
    fn transformation(&self) -> Result<Matrix4<f32>>;
}
