use super::camera_object::CameraObject;

#[derive(Debug)]
pub enum Command {
    ClearCamera,
    SetCamera(Box<CameraObject>),
}
