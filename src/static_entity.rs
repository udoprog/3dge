use super::model::Model;
use cgmath::{Matrix4, Point3};
use cgmath::prelude::*;
use gfx::GeometryId;
use gfx::errors as gfx;
use gfx::geometry::Geometry;
use gfx::geometry_accessor::GeometryAccessor;
use gfx::geometry_object::GeometryObject;
use gfx::primitives::Primitives;
use std::sync::{Arc, RwLock, RwLockReadGuard};

#[derive(Debug)]
pub struct StaticEntityGeometry {
    id: GeometryId,
    location: Point3<f32>,
    model: Model,
}

impl StaticEntityGeometry {
    pub fn new(model: Model) -> StaticEntityGeometry {
        StaticEntityGeometry {
            id: GeometryId::allocate(),
            location: Point3::new(0.0, 0.0, 0.0),
            model: model,
        }
    }
}

pub struct StaticEntity {
    geometry: Arc<RwLock<StaticEntityGeometry>>,
}

impl StaticEntity {
    pub fn new(model: Model) -> StaticEntity {
        StaticEntity { geometry: Arc::new(RwLock::new(StaticEntityGeometry::new(model))) }
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
        self.geometry
            .read()
            .map_err(|_| gfx::ErrorKind::PoisonError)?
            .position()
    }
}

impl GeometryObject for StaticEntity {
    fn geometry(&self) -> Box<Geometry> {
        Box::new(self.geometry.clone())
    }
}

impl Geometry for Arc<RwLock<StaticEntityGeometry>> {
    fn read_lock<'a>(&'a self) -> gfx::Result<Box<'a + GeometryAccessor>> {
        Ok(Box::new(
            self.read().map_err(|_| gfx::ErrorKind::PoisonError)?,
        ))
    }
}

impl<'a> GeometryAccessor for RwLockReadGuard<'a, StaticEntityGeometry> {
    fn id(&self) -> GeometryId {
        self.id
    }

    fn transformation(&self) -> gfx::Result<Matrix4<f32>> {
        Ok(Matrix4::from_translation(self.location.to_vec()))
    }

    fn position(&self) -> gfx::Result<Point3<f32>> {
        Ok(self.location)
    }

    fn primitives(&self) -> gfx::Result<Primitives> {
        Ok(self.model.primitives())
    }
}
