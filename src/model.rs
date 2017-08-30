use super::errors::*;
use cgmath::{Matrix4, Point3};
use cgmath::prelude::*;
use gfx::{GeometryId, Normal, Vertex};
use gfx::errors as gfx;
use gfx::geometry::{Geometry, GeometryAccessor};
use gfx::geometry_object::GeometryObject;
use gfx::vertices::Vertices;
use gltf_importer::{self, Config};
use gltf_importer::config::ValidationStrategy;
use gltf_utils::PrimitiveIterators;
use std::path::Path;
use std::sync::{Arc, RwLock, RwLockReadGuard};

#[derive(Debug)]
pub struct ModelGeometry {
    id: GeometryId,
    location: Point3<f32>,
    pub mesh: Vec<Vertex>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u32>,
}

impl ModelGeometry {
    pub fn from_gltf<P: AsRef<Path>>(path: P) -> Result<ModelGeometry> {
        let config = Config { validation_strategy: ValidationStrategy::Complete };

        let (gltf, buffers) = gltf_importer::import_with_config(path, config)?;

        let m = gltf.meshes().nth(0).ok_or(ErrorKind::NoMesh)?;
        let p = m.primitives().nth(0).ok_or(ErrorKind::NoPrimitive)?;

        let mesh: Vec<Vertex> = {
            if let Some(positions) = p.positions(&buffers) {
                positions.map(Into::into).collect()
            } else {
                Vec::new()
            }
        };

        let normals = Vec::new();

        let indices = p.indices_u32(&buffers)
            .ok_or(ErrorKind::NoIndices)?
            .collect();

        Ok(ModelGeometry {
            id: GeometryId::allocate(),
            location: Point3::new(0.0, 0.0, 0.0),
            mesh: mesh,
            normals: normals,
            indices: indices,
        })
    }
}

pub struct Model {
    geometry: Arc<RwLock<ModelGeometry>>,
}

impl Model {
    pub fn new(geometry: ModelGeometry) -> Model {
        Model { geometry: Arc::new(RwLock::new(geometry)) }
    }

    pub fn transform(&mut self, transform: &Matrix4<f32>) -> gfx::Result<()> {
        let mut g = self.geometry.write().map_err(
            |_| gfx::ErrorKind::PoisonError,
        )?;
        g.location = transform.transform_point(g.location);
        Ok(())
    }

    /// Get the position of the player.
    pub fn position(&self) -> gfx::Result<Point3<f32>> {
        self.geometry.read_lock()?.position()
    }
}

impl GeometryObject for Model {
    fn geometry(&self) -> Box<Geometry> {
        Box::new(self.geometry.clone())
    }
}

impl Geometry for Arc<RwLock<ModelGeometry>> {
    fn read_lock<'a>(&'a self) -> gfx::Result<Box<'a + GeometryAccessor>> {
        Ok(Box::new(
            self.read().map_err(|_| gfx::ErrorKind::PoisonError)?,
        ))
    }
}

impl<'a> GeometryAccessor for RwLockReadGuard<'a, ModelGeometry> {
    fn id(&self) -> GeometryId {
        self.id
    }

    fn transformation(&self) -> gfx::Result<Matrix4<f32>> {
        Ok(Matrix4::from_translation(self.location.to_vec()))
    }

    fn position(&self) -> gfx::Result<Point3<f32>> {
        Ok(self.location)
    }

    fn vertices(&self) -> gfx::Result<Vertices> {
        Ok(Vertices::new(
            self.mesh.clone(),
            self.normals.clone(),
            self.indices.clone(),
        ))
    }
}
