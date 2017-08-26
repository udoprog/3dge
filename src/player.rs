use super::game::GeometryObject;
use super::gfx::errors as gfx;
use super::gfx::geometry::Geometry;
use cgmath::{Matrix4, Point3};
use cgmath::prelude::*;
use std::sync::{Arc, RwLock};

pub struct PlayerGeometry {
    location: Point3<f32>,
}

impl PlayerGeometry {
    pub fn new() -> PlayerGeometry {
        PlayerGeometry { location: Point3::new(0.0, 0.0, 0.0) }
    }
}

pub struct Player {
    geometry: Arc<RwLock<PlayerGeometry>>,
}

impl Player {
    pub fn new() -> Player {
        Player { geometry: Arc::new(RwLock::new(PlayerGeometry::new())) }
    }

    pub fn transform(&mut self, transform: &Matrix4<f32>) -> gfx::Result<()> {
        let mut g = self.geometry.write().map_err(|e| gfx::Error::PoisonError)?;
        g.location = transform.transform_point(g.location);
        Ok(())
    }

    /// Get the position of the player.
    pub fn position(&self) -> gfx::Result<Point3<f32>> {
        let g = self.geometry.read().map_err(|e| gfx::Error::PoisonError)?;
        Ok(g.location)
    }
}

impl GeometryObject for Player {
    fn geometry(&self) -> Box<Geometry> {
        Box::new(self.geometry.clone())
    }
}

impl Geometry for Arc<RwLock<PlayerGeometry>> {
    fn transformation(&self) -> gfx::Result<Matrix4<f32>> {
        let g = self.read().map_err(|e| gfx::Error::PoisonError)?;
        Ok(Matrix4::from_translation(g.location.to_vec()))
    }
}
