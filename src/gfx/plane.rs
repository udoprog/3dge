use super::errors::*;
use texture::Texture;

pub trait Plane {
    fn bind_texture(&mut self, texture: &Texture) -> Result<()>;
}
