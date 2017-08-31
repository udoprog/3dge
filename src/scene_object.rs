use super::camera::Camera;
use super::player::Player;
use super::static_entity::StaticEntity;
use std::sync::{Arc, RwLock};

pub enum SceneObject {
    Player(Player),
    Camera(Arc<RwLock<Camera>>),
    StaticEntity(StaticEntity),
}

impl From<Arc<RwLock<Camera>>> for SceneObject {
    fn from(value: Arc<RwLock<Camera>>) -> SceneObject {
        SceneObject::Camera(value)
    }
}

impl From<Player> for SceneObject {
    fn from(value: Player) -> SceneObject {
        SceneObject::Player(value)
    }
}

impl From<StaticEntity> for SceneObject {
    fn from(value: StaticEntity) -> SceneObject {
        SceneObject::StaticEntity(value)
    }
}
