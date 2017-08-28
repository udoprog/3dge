use super::camera_geometry::CameraGeometry;

pub trait CameraObject {
    fn geometry(&self) -> Box<CameraGeometry>;
}
