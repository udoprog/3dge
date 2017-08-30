use super::Vertex;
use super::errors::*;
use cgmath::{Matrix4, Point3};
use std::sync::{Arc, RwLock};

/// Describes the geomtry of some object on screen.
///
/// Needs to be thread-safe to be read by the render thread.
pub trait Geometry: Send + Sync {
    fn read_lock<'a>(&'a self) -> Result<Box<'a + GeometryAccessor>>;
}

pub trait GeometryAccessor {
    /// Get the homogenous transformation matrix for this geometry.
    fn transformation(&self) -> Result<Matrix4<f32>>;

    /// Position (to origin) of the geometry object.
    fn position(&self) -> Result<Point3<f32>>;

    /// Get all vertices associated with the geometry.
    fn vertices(&self) -> Result<Vec<Vertex>>;
}
