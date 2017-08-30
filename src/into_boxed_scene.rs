use super::boxed_scene::BoxedScene;
use super::errors::*;
use gfx::Gfx;

pub trait IntoBoxedScene<C> {
    fn into_boxed_scene(self, gfx: Box<Gfx>) -> Result<Box<BoxedScene<C>>>;
}
