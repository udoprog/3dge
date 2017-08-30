use cgmath::{Matrix4, Point3, Rad, Vector3};
use cgmath::prelude::*;
use gfx::{GeometryId, Vertex};
use gfx::color::Color;
use gfx::errors::*;
use gfx::geometry::{Geometry, GeometryAccessor};
use gfx::geometry_object::GeometryObject;
use std::sync::{Arc, RwLock, RwLockReadGuard};

#[derive(Debug)]
pub struct RectangleGeometry {
    id: GeometryId,
    origin: Point3<f32>,
    normal: Vector3<f32>,
    color: Color,
}

impl RectangleGeometry {
    pub fn new(origin: Point3<f32>, normal: Vector3<f32>, color: Color) -> RectangleGeometry {
        RectangleGeometry {
            id: GeometryId::allocate(),
            origin: origin,
            normal: normal,
            color: color,
        }
    }
}

pub struct Rectangle {
    geometry: Arc<RwLock<RectangleGeometry>>,
}

impl Rectangle {
    pub fn new(origin: Point3<f32>, normal: Vector3<f32>, color: Color) -> Rectangle {
        Rectangle { geometry: Arc::new(RwLock::new(RectangleGeometry::new(origin, normal, color))) }
    }
}

impl GeometryObject for Rectangle {
    fn geometry(&self) -> Box<Geometry> {
        Box::new(self.geometry.clone())
    }
}

impl Geometry for Arc<RwLock<RectangleGeometry>> {
    fn read_lock<'a>(&'a self) -> Result<Box<'a + GeometryAccessor>> {
        Ok(Box::new(self.read().map_err(|_| ErrorKind::PoisonError)?))
    }
}

impl<'a> GeometryAccessor for RwLockReadGuard<'a, RectangleGeometry> {
    fn id(&self) -> GeometryId {
        self.id
    }

    fn transformation(&self) -> Result<Matrix4<f32>> {
        let translation = Matrix4::from_translation(self.origin.to_vec());
        let rotation = Matrix4::from_axis_angle(self.normal, Rad(0.0));
        Ok(translation * rotation)
    }

    fn position(&self) -> Result<Point3<f32>> {
        Ok(self.origin)
    }

    fn vertices(&self) -> Result<Vec<Vertex>> {
        let color: [f32; 3] = self.color.into();

        let mut vertices = Vec::new();

        vertices.push(Vertex {
            position: [-0.5, -0.5, 0.0],
            color: color,
        });

        vertices.push(Vertex {
            position: [0.5, -0.5, 0.0],
            color: color,
        });

        vertices.push(Vertex {
            position: [0.5, 0.5, 0.0],
            color: color,
        });

        vertices.push(Vertex {
            position: [-0.5, -0.5, 0.0],
            color: color,
        });

        vertices.push(Vertex {
            position: [0.5, 0.5, 0.0],
            color: color,
        });

        vertices.push(Vertex {
            position: [-0.5, 0.5, 0.0],
            color: color,
        });

        Ok(vertices)
    }
}
