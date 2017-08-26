use super::camera::Camera;
use gfx::geometry::Geometry;

pub struct Game<'a> {
    geometry: Vec<Box<Geometry>>,
    camera: &'a Camera<'a>,
}

impl<'a> Game<'a> {
    pub fn new(camera: &'a Camera<'a>) -> Game<'a> {
        Game {
            geometry: Vec::new(),
            camera: camera,
        }
    }

    pub fn register_geometry(&mut self, geometry_object: &GeometryObject) {
        self.geometry.push(geometry_object.geometry());
    }
}

pub trait GeometryObject {
    /// Get geometry associated with the game object.
    /// Geometry determines it's location in the world, rotation, and scale.
    fn geometry(&self) -> Box<Geometry>;
}
