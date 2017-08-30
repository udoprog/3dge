use super::camera_object::CameraObject;
use super::geometry::Geometry;

#[derive(Debug)]
pub enum Command {
    ClearCamera,
    SetCamera(Box<CameraObject>),
    AddGeometry(Box<Geometry>),
}
