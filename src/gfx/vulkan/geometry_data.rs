use gfx::geometry::Geometry;

pub struct GeometryData {
    pub geometry: Vec<Box<Geometry>>,
}

impl GeometryData {
    pub fn new() -> GeometryData {
        GeometryData { geometry: Vec::new() }
    }

    pub fn push(&mut self, geometry: Box<Geometry>) {
        self.geometry.push(geometry);
    }
}
