use super::geometry::Geometry;

pub trait GeometryObject {
    /// Get geometry associated with the game object.
    /// Geometry determines it's location in the world, rotation, and scale.
    fn geometry(&self) -> Box<Geometry>;
}
