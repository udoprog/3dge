use super::camera_accessor::CameraAccessor;
use super::errors::*;
use std::fmt;

pub trait CameraObject: fmt::Debug + Send + Sync {
    fn write_lock<'a>(&'a self) -> Result<Box<'a + CameraAccessor>>;

    /// Clone the camera object.
    fn clone_camera_object(&self) -> Box<CameraObject>;
}
