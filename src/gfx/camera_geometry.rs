use super::errors::*;
use cgmath::Matrix4;

/// Provides of camera geometry.
pub trait CameraGeometry {
    /// Get the homogeneous view transformation for the camera.
    fn view_transformation(&self) -> Result<Matrix4<f32>>;
}
