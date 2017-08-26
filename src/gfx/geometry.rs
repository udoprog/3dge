use super::Vertex;
use super::errors::*;
use cgmath::{Matrix4, Point3};

/// Describes the geomtry of some object on screen.
///
/// Needs to be thread-safe to be read by the render thread.
pub trait Geometry: Send + Sync {
    /// Get the homogenous transformation matrix for this geometry.
    fn transformation(&self) -> Result<Matrix4<f32>>;

    /// Position (to origin) of the geometry object.
    fn position(&self) -> Result<Point3<f32>>;

    /// Get all vertices associated with the geometry.
    fn vertices(&self) -> Result<Vec<Vertex>>;
}

pub trait GeometryObject {
    /// Get geometry associated with the game object.
    /// Geometry determines it's location in the world, rotation, and scale.
    fn geometry(&self) -> Box<Geometry>;
}
