use super::player::Player;
use cgmath::{Matrix4, Point3};
use cgmath::prelude::*;
use gfx::errors as gfx;

/// A camera that always looks at a piece of geometry.
pub struct Camera<'a> {
    player: &'a Player,
    location: Point3<f32>,
}

impl<'a> Camera<'a> {
    pub fn new(player: &Player) -> Camera {
        Camera {
            player: player,
            location: Point3::new(0.0, 0.0, 0.0),
        }
    }

    fn transformation(&self) -> gfx::Result<Matrix4<f32>> {
        let player_pos = self.player.position()?;

        Ok(Matrix4::look_at(
            Point3::new(0.3, 0.3, 1.0),
            self.location,
            player_pos.to_vec(),
        ))
    }
}
