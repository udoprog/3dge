use cgmath::{Matrix4, Point3, Vector3};
use gfx::camera_geometry::CameraGeometry;
use gfx::errors as gfx;
use gfx::geometry::{Geometry, GeometryObject};
use std::sync::{Arc, RwLock};

/// A camera that always looks at a piece of geometry.
pub struct Camera {
    player: Box<Geometry>,
    location: Point3<f32>,
}

impl Camera {
    pub fn new(player: &GeometryObject) -> Camera {
        Camera {
            player: player.geometry(),
            location: Point3::new(0.0, 1.0, 1.0),
        }
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

        location.y = player_pos.y + 1.0;

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
