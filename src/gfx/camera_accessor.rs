use super::errors::*;
use cgmath::Matrix4;

/// Provides of camera geometry.
pub trait CameraAccessor {
    /// Get the homogeneous view transformation for the camera.
    fn view_transformation(&mut self) -> Result<Matrix4<f32>>;
}
