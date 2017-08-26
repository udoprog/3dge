use super::camera::Camera;
use super::errors::*;
use gfx::GfxLoop;
use gfx::geometry::{Geometry, GeometryObject};
use std::sync::{Arc, RwLock};

pub struct Game {
    camera: Arc<RwLock<Camera>>,
    gfx_loop: Box<GfxLoop>,
}

impl Game {
    pub fn new(camera: Arc<RwLock<Camera>>, gfx_loop: Box<GfxLoop>) -> Game {
        Game {
            camera: camera,
            gfx_loop: gfx_loop,
        }
    }

    pub fn register_geometry(&mut self, geometry_object: &GeometryObject) {
        self.gfx_loop.register_geometry(geometry_object);
    }

    pub fn tick(&mut self) -> Result<()> {
        self.gfx_loop.tick()?;
        Ok(())
    }
}
