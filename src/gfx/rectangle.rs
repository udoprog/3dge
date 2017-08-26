use cgmath::{Matrix4, Point3, Rad, Vector3};
use cgmath::prelude::*;
use gfx::Vertex;
use gfx::color::Color;
use gfx::errors as gfx;
use gfx::geometry::{Geometry, GeometryObject};
use std::sync::{Arc, RwLock};

pub struct RectangleGeometry {
    origin: Point3<f32>,
    normal: Vector3<f32>,
    color: Color,
}

impl RectangleGeometry {
    pub fn new(origin: Point3<f32>, normal: Vector3<f32>, color: Color) -> RectangleGeometry {
        RectangleGeometry {
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
    fn transformation(&self) -> gfx::Result<Matrix4<f32>> {
        let g = self.read().map_err(|_| gfx::Error::PoisonError)?;
        let translation = Matrix4::from_translation(g.origin.to_vec());
        let rotation = Matrix4::from_axis_angle(g.normal, Rad(0.0));
        Ok(translation * rotation)
    }

    fn position(&self) -> gfx::Result<Point3<f32>> {
        Ok(self.read().map_err(|_| gfx::Error::PoisonError)?.origin)
    }

    fn vertices(&self) -> gfx::Result<Vec<Vertex>> {
        let g = self.read().map_err(|_| gfx::Error::PoisonError)?;

        let color: [f32; 3] = g.color.into();

        let mut vertices = Vec::new();

        vertices.push(Vertex {
            position: [-0.5, -0.5],
            color: color,
        });

        vertices.push(Vertex {
            position: [0.5, -0.5],
            color: color,
        });

        vertices.push(Vertex {
            position: [0.5, 0.5],
            color: color,
        });

        vertices.push(Vertex {
            position: [-0.5, -0.5],
            color: color,
        });

        vertices.push(Vertex {
            position: [0.5, 0.5],
            color: color,
        });

        vertices.push(Vertex {
            position: [-0.5, 0.5],
            color: color,
        });

        Ok(vertices)
    }
}
