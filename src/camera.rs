use cgmath::{Matrix4, Point3, Vector3};
use gfx::camera_geometry::CameraGeometry;
use gfx::camera_object::CameraObject;
use gfx::errors as gfx;
use gfx::geometry::Geometry;
use gfx::geometry_object::GeometryObject;
use std::sync::{Arc, RwLock};

/// A camera that always looks at a piece of geometry.
pub struct Camera {
    player: Box<Geometry>,
    location: Point3<f32>,
    zoom: f32,
}

impl Camera {
    pub fn new(player: &GeometryObject) -> Camera {
        Camera {
            player: player.geometry(),
            location: Point3::new(0.0, 1.0, 1.0),
            zoom: 0.0,
        }
    }

    pub fn modify_zoom(&mut self, zoom: f32) {
        let new_zoom = self.zoom + zoom;
        self.zoom = f32::min(0.9, f32::max(0.0, new_zoom));
    }
}

impl CameraObject for Arc<RwLock<Camera>> {
    fn geometry(&self) -> Box<CameraGeometry> {
        Box::new(self.clone())
    }
}

impl CameraGeometry for Arc<RwLock<Camera>> {
    fn view_transformation(&self) -> gfx::Result<Matrix4<f32>> {
        let mut camera = self.write().map_err(|_| gfx::Error::PoisonError)?;
        let player_pos = camera.player.position()?;

        let mut location = camera.location;

        // Slowly following camera, just to see some horizontal movement.
        location.x = f32::min(player_pos.x + 0.2, location.x);
        location.x = f32::max(player_pos.x - 0.2, location.x);

        let inverse_zoom = 1.0 - camera.zoom;

        location.y = player_pos.y + 5.0 * inverse_zoom;
        location.z = 5.0 * inverse_zoom;

        camera.location = location;

        let look_at = Matrix4::look_at(
            /// Where the camera is.
            location,
            /// Where we are looking at
            player_pos,
            /// What should be considered 'up'.
            Vector3::new(0.0, 0.0, -1.0),
        );

        Ok(look_at)
    }
}
