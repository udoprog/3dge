use super::errors::*;
use cgmath::{Matrix4, Point3};
use cgmath::prelude::*;
use gfx::Vertex;
use gfx::errors as gfx;
use gfx::geometry::{Geometry, GeometryObject};
use gltf::Gltf;
use std::io::{BufReader, Read};
use std::sync::{Arc, RwLock};

pub struct ModelGeometry {
    location: Point3<f32>,
    mesh: Vec<Vertex>,
}

pub struct Model {
    geometry: Arc<RwLock<ModelGeometry>>,
}

impl Model {
    pub fn from_gltf<R>(reader: R) -> Result<Model>
    where
        R: Read,
    {
        let gltf = Gltf::from_reader(BufReader::new(reader))?
            .validate_minimally()?;

        let mut mesh = Vec::new();

        println!("model = {:?}", gltf);

        let model = Model {
            geometry: Arc::new(RwLock::new(ModelGeometry {
                location: Point3::new(0.0, 0.0, 0.0),
                mesh: mesh,
            })),
        };

        Ok(model)
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
        Ok(g.mesh.clone())
    }
}
