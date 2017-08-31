use super::boxed_scene::BoxedScene;
use super::camera::CameraScroll;
use super::errors::*;
use super::into_boxed_scene::IntoBoxedScene;
use super::player::PlayerTransform;
use super::scene_object::SceneObject;
use super::scheduler::{Scheduler, SchedulerSetup};
use cgmath::Matrix4;
use gfx::Gfx;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SceneState<C, S> {
    pub core: Rc<RefCell<C>>,
    pub state: Rc<RefCell<S>>,
}

pub struct Scene<C, S> {
    state: Rc<RefCell<S>>,
    objects: Vec<SceneObject>,
    pub scheduler: Scheduler<SceneState<C, S>>,
}

impl<C: 'static + CameraScroll + PlayerTransform, S: 'static> IntoBoxedScene<C> for Scene<C, S> {
    fn into_boxed_scene(mut self, gfx: &Gfx) -> Result<Box<BoxedScene<C>>> {
        self.setup(gfx)?;
        Ok(Box::new(self))
    }
}

impl<C, S> BoxedScene<C> for Scene<C, S> {
    fn tick(&mut self, core: Rc<RefCell<C>>) -> Result<()> {
        let mut scheduler = &mut self.scheduler;

        {
            let mut s = SceneState {
                core: core.clone(),
                state: self.state.clone(),
            };

            scheduler.tick(&mut s)?;
        }

        Ok(())
    }
}

impl<C: 'static + CameraScroll + PlayerTransform, S: 'static> Scene<C, S> {
    /// Create a new, empty scene.
    pub fn new(state: S) -> Scene<C, S> {
        Scene {
            state: Rc::new(RefCell::new(state)),
            objects: Vec::new(),
            scheduler: Scheduler::new(),
        }
    }

    /// Register the given scene object.
    pub fn register<O: Into<SceneObject>>(&mut self, object: O) {
        self.objects.push(object.into());
    }

    pub fn setup(&mut self, gfx: &Gfx) -> Result<()> {
        use self::SceneObject::*;

        for object in &mut self.objects {
            match *object {
                Player(ref mut player) => {
                    gfx.register_geometry(player)?;
                    player.setup_scheduler(&mut self.scheduler);
                }
                Camera(ref mut camera) => {
                    gfx.set_camera(camera)?;
                    camera.setup_scheduler(&mut self.scheduler);
                }
                StaticEntity(ref mut static_entity) => {
                    gfx.register_geometry(static_entity)?;
                }
            }
        }

        Ok(())
    }
}

impl<C, S> PlayerTransform for SceneState<C, S>
where
    C: PlayerTransform,
{
    fn player_transform(&mut self) -> Result<Option<Matrix4<f32>>> {
        self.core.try_borrow_mut()?.player_transform()
    }
}

impl<C, S> CameraScroll for SceneState<C, S>
where
    C: CameraScroll,
{
    fn take_scroll(&mut self) -> Result<i32> {
        self.core.try_borrow_mut()?.take_scroll()
    }
}
