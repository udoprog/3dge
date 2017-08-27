use cgmath::{Matrix4, Point3};
use cgmath::prelude::*;
use gfx::Vertex;
use gfx::errors as gfx;
use gfx::geometry::{Geometry, GeometryObject};
use std::sync::{Arc, RwLock};

pub struct ModelGeometry {
    location: Point3<f32>,
    mesh: Vec<Vertex>,
}

impl ModelGeometry {
    pub fn new(mesh: Vec<Vertex>) -> ModelGeometry {
        ModelGeometry {
            location: Point3::new(0.0, 0.0, 0.0),
            mesh: mesh,
        }
    }
}

pub struct Model {
    geometry: Arc<RwLock<ModelGeometry>>,
}

impl Model {
    pub fn from_gltf() -> Result<Model> {}

    pub fn new() -> Model {
        Model { geometry: Arc::new(RwLock::new(ModelGeometry::new())) }
    }

    pub fn transform(&mut self, transform: &Matrix4<f32>) -> gfx::Result<()> {
        let mut g = self.geometry.write().map_err(|_| gfx::Error::PoisonError)?;
        g.location = transform.transform_point(g.location);
        Ok(())
    }

    /// Get the position of the player.
    pub fn position(&self) -> gfx::Result<Point3<f32>> {
        self.geometry.position()
    }
}

impl GeometryObject for Model {
    fn geometry(&self) -> Box<Geometry> {
        Box::new(self.geometry.clone())
    }
}

impl Geometry for Arc<RwLock<ModelGeometry>> {
    fn transformation(&self) -> gfx::Result<Matrix4<f32>> {
        let g = self.read().map_err(|_| gfx::Error::PoisonError)?;
        Ok(Matrix4::from_translation(g.location.to_vec()))
    }

    fn position(&self) -> gfx::Result<Point3<f32>> {
        Ok(self.read().map_err(|_| gfx::Error::PoisonError)?.location)
    }

    fn vertices(&self) -> gfx::Result<Vec<Vertex>> {
        let g = self.read().map_err(|_| gfx::Error::PoisonError)?;
        Ok(g.vertices.clone())
    }
}
