use super::errors::*;
use super::model::ModelGeometry;
use super::scheduler::{Scheduler, SchedulerSetup};
use cgmath::{Matrix4, Point3};
use cgmath::prelude::*;
use gfx::GeometryId;
use gfx::errors as gfx;
use gfx::geometry::{Geometry, GeometryAccessor};
use gfx::geometry_object::GeometryObject;
use gfx::vertices::Vertices;
use std::sync::{Arc, RwLock, RwLockReadGuard};

#[derive(Debug)]
pub struct PlayerGeometry {
    id: GeometryId,
    location: Point3<f32>,
    model: ModelGeometry,
}

impl PlayerGeometry {
    pub fn new(model: ModelGeometry) -> PlayerGeometry {
        PlayerGeometry {
            id: GeometryId::allocate(),
            location: Point3::new(0.0, 0.0, 0.0),
            model: model,
        }
    }
}

pub struct Player {
    geometry: Arc<RwLock<PlayerGeometry>>,
}

impl Player {
    pub fn new(model: ModelGeometry) -> Player {
        Player { geometry: Arc::new(RwLock::new(PlayerGeometry::new(model))) }
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

impl GeometryObject for Player {
    fn geometry(&self) -> Box<Geometry> {
        Box::new(self.geometry.clone())
    }
}

impl Geometry for Arc<RwLock<PlayerGeometry>> {
    fn read_lock<'a>(&'a self) -> gfx::Result<Box<'a + GeometryAccessor>> {
        Ok(Box::new(
            self.read().map_err(|_| gfx::ErrorKind::PoisonError)?,
        ))
    }
}

impl<'a> GeometryAccessor for RwLockReadGuard<'a, PlayerGeometry> {
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
            self.model.mesh.clone(),
            self.model.normals.clone(),
            self.model.indices.clone(),
        ))
    }
}

pub trait PlayerTransform {
    fn player_transform(&mut self) -> Result<Option<Matrix4<f32>>>;
}

impl<S: PlayerTransform> SchedulerSetup<S> for Player {
    fn setup_scheduler(&mut self, scheduler: &mut Scheduler<S>) {
        let geometry = self.geometry.clone();

        scheduler.on_every_tick(Box::new(move |_, gs| {
            // perform player transform based on pressed keys
            if let Some(transform) = gs.player_transform()? {
                let mut g = geometry.write().map_err(|_| ErrorKind::PoisonError)?;
                g.location = transform.transform_point(g.location);
            }

            Ok(())
        }));
    }
}
