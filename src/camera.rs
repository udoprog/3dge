use super::errors::*;
use super::scheduler::{Scheduler, SchedulerSetup};
use cgmath::{Matrix4, Point3, Vector3};
use gfx::camera_accessor::CameraAccessor;
use gfx::camera_object::CameraObject;
use gfx::errors as gfx;
use gfx::geometry::Geometry;
use gfx::geometry_object::GeometryObject;
use std::fmt;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

/// Trait for a scroll provider.
pub trait CameraScroll {
    /// Take the current accumulated scroll value.
    fn take_scroll(&mut self) -> Result<i32>;
}

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
            location: Point3::new(0.0, -10.0, -2.0),
            zoom: 0.0,
        }
    }

    pub fn modify_zoom(&mut self, zoom: f32) {
        let new_zoom = self.zoom + zoom;
        self.zoom = f32::min(0.9, f32::max(0.0, new_zoom));
    }
}

impl fmt::Debug for Camera {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Camera {{ }}")
    }
}

impl<S: CameraScroll> SchedulerSetup<S> for Arc<RwLock<Camera>> {
    fn setup_scheduler(&mut self, scheduler: &mut Scheduler<S>) {
        let camera = self.clone();

        scheduler.on_every_tick(Box::new(move |_, s| {
            let scroll = s.take_scroll()?;

            if scroll != 0 {
                let mut camera = camera.write().map_err(|_| ErrorKind::PoisonError)?;
                let amount = (-scroll as f32) * 0.005;
                camera.modify_zoom(amount);
            }

            Ok(())
        }));
    }
}

impl CameraObject for Arc<RwLock<Camera>> {
    fn write_lock<'a>(&'a self) -> gfx::Result<Box<'a + CameraAccessor>> {
        Ok(Box::new(
            self.write().map_err(|_| gfx::ErrorKind::PoisonError)?,
        ))
    }

    fn clone_camera_object(&self) -> Box<CameraObject> {
        Box::new(self.clone())
    }
}

impl<'a> CameraAccessor for RwLockWriteGuard<'a, Camera> {
    fn view_transformation(&mut self) -> gfx::Result<Matrix4<f32>> {
        let player_pos = self.player.read_lock()?.position()?;

        let mut location = self.location;
        location.x = player_pos.x;
        location.y = location.y + 10.0 * self.zoom;
        location.z = player_pos.z - 2.0;

        let look_at = Matrix4::look_at(
            /// Where the camera is.
            location,
            /// Where we are looking at
            player_pos,
            /// What should be considered 'up'.
            Vector3::new(0.0, 1.0, 0.0),
        );

        Ok(look_at)
    }
}
