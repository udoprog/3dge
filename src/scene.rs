use super::errors::*;
use super::player::Player;
use super::scheduler::Scheduler;
use gfx::Gfx;
use std::sync::{Arc, RwLock};

pub struct Scene<S> {
    state: S,
    objects: Vec<SceneObject>,
    scheduler: Scheduler<S>,
}

pub enum SceneObject {
    Player(Player),
}

impl<S> Scene<S> {
    /// Create a new, empty scene.
    pub fn new(state: S, gfx: Box<Gfx>) -> Scene<S> {
        Scene {
            state: state,
            objects: Vec::new(),
            scheduler: Scheduler::new(),
        }
    }

    pub fn tick(&mut self) -> Result<()> {
        self.scheduler.tick(&mut self.state)
    }

    /// Register the given scene object.
    pub fn register<O: Into<SceneObject>>(&mut self, object: O) {
        self.objects.push(object.into());
    }
}

impl From<Player> for SceneObject {
    fn from(value: Player) -> SceneObject {
        SceneObject::Player(value)
    }
}
