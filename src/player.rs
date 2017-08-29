use super::errors::*;
use super::scheduler::{Scheduler, SchedulerSetup};
use cgmath::{Matrix4, Point3};
use cgmath::prelude::*;
use gfx::Vertex;
use gfx::errors as gfx;
use gfx::geometry::Geometry;
use gfx::geometry_object::GeometryObject;
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
        let mut g = self.geometry.write().map_err(|_| gfx::Error::PoisonError)?;
        g.location = transform.transform_point(g.location);
        Ok(())
    }

    /// Get the position of the player.
    pub fn position(&self) -> gfx::Result<Point3<f32>> {
        self.geometry.position()
    }
}

impl GeometryObject for Player {
    fn geometry(&self) -> Box<Geometry> {
        Box::new(self.geometry.clone())
    }
}

impl Geometry for Arc<RwLock<PlayerGeometry>> {
    fn transformation(&self) -> gfx::Result<Matrix4<f32>> {
        let g = self.read().map_err(|_| gfx::Error::PoisonError)?;
        Ok(Matrix4::from_translation(g.location.to_vec()))
    }

    fn position(&self) -> gfx::Result<Point3<f32>> {
        Ok(self.read().map_err(|_| gfx::Error::PoisonError)?.location)
    }

    fn vertices(&self) -> gfx::Result<Vec<Vertex>> {
        let mut vertices = Vec::new();

        let red = [1.0, 0.0, 0.0];
        let green = [0.0, 1.0, 0.0];

        vertices.push(Vertex {
            position: [-0.1, -0.1, 0.0],
            color: red,
        });

        vertices.push(Vertex {
            position: [0.1, -0.1, 0.0],
            color: red,
        });

        vertices.push(Vertex {
            position: [0.1, 0.1, 0.0],
            color: red,
        });

        vertices.push(Vertex {
            position: [-0.1, -0.1, 0.0],
            color: green,
        });

        vertices.push(Vertex {
            position: [0.1, 0.1, 0.0],
            color: green,
        });

        vertices.push(Vertex {
            position: [-0.1, 0.1, 0.0],
            color: green,
        });

        Ok(vertices)
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
                let mut g = geometry.write().map_err(|_| gfx::Error::PoisonError)?;
                g.location = transform.transform_point(g.location);
            }

            Ok(())
        }));
    }
}
