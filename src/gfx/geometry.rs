use cgmath::{Matrix4, Point3};
use gfx::GeometryId;
use gfx::Vertex;
use gfx::errors::*;
use std::fmt;

pub trait CloneGeometry {
    fn clone_geometry(&self) -> Box<Geometry>;
}

/// Describes the geomtry of some object on screen.
///
/// Needs to be thread-safe to be read by the render thread.
pub trait Geometry: fmt::Debug + Send + Sync + CloneGeometry {
    fn read_lock<'a>(&'a self) -> Result<Box<'a + GeometryAccessor>>;
}

impl<T> CloneGeometry for T
where
    T: 'static + Clone + Geometry,
{
    fn clone_geometry(&self) -> Box<Geometry> {
        Box::new(self.clone())
    }
}

pub trait GeometryAccessor {
    fn id(&self) -> GeometryId;

    /// Get the homogenous transformation matrix for this geometry.
    fn transformation(&self) -> Result<Matrix4<f32>>;

    /// Position (to origin) of the geometry object.
    fn position(&self) -> Result<Point3<f32>>;

    /// Get all vertices associated with the geometry.
    fn vertices(&self) -> Result<Vec<Vertex>>;
}
