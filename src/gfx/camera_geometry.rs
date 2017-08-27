use super::errors::*;
use cgmath::Matrix4;
use std::marker;

/// Provides of camera geometry.
pub trait CameraGeometry: marker::Sync + marker::Send {
    /// Get the homogeneous view transformation for the camera.
    fn view_transformation(&self) -> Result<Matrix4<f32>>;
}
