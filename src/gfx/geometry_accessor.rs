use super::primitives::Primitives;
use cgmath::{Matrix4, Point3};
use gfx::GeometryId;
use gfx::errors::*;

pub trait GeometryAccessor {
    fn id(&self) -> GeometryId;

    /// Get the homogenous transformation matrix for this geometry.
    fn transformation(&self) -> Result<Matrix4<f32>>;

    /// Position (to origin) of the geometry object.
    fn position(&self) -> Result<Point3<f32>>;

    /// Get all vertices associated with the geometry.
    fn primitives(&self) -> Result<Primitives>;
}
