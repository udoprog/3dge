use super::geometry_accessor::GeometryAccessor;
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
