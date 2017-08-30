use gfx::camera_object::CameraObject;
use gfx::command::Command;
use gfx::errors::*;
use gfx::geometry_object::GeometryObject;
use std::sync::mpsc;

#[derive(Clone)]
pub struct VulkanGfx {
    send: mpsc::Sender<Command>,
}

impl VulkanGfx {
    pub fn new(send: mpsc::Sender<Command>) -> VulkanGfx {
        VulkanGfx { send: send }
    }

    pub fn clear(&self) -> Result<()> {
        self.send.send(Command::ClearCamera).map_err(
            |_| ErrorKind::SendError,
        )?;

        Ok(())
    }

    pub fn set_camera(&self, camera_object: &CameraObject) -> Result<()> {
        self.send
            .send(Command::SetCamera(camera_object.clone_camera_object()))
            .map_err(|_| ErrorKind::SendError)?;

        Ok(())
    }

    pub fn register_geometry(&self, geometry_object: &GeometryObject) -> Result<()> {
        self.send
            .send(Command::AddGeometry(geometry_object.geometry()))
            .map_err(|_| ErrorKind::SendError)?;
        Ok(())
    }
}
