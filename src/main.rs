extern crate env_logger;
extern crate threedge;
extern crate cgmath;

use std::sync::{Arc, RwLock};
use threedge::camera::Camera;
use threedge::core_loop::CoreLoop;
use threedge::core_state::CoreState;
use threedge::errors::*;
use threedge::gltf_loader::GltfLoader;
use threedge::player::Player;
use threedge::scene::Scene;
use threedge::static_entity::StaticEntity;

struct SceneState {}

fn setup_scene() -> Result<Scene<CoreState, SceneState>> {
    let player = GltfLoader::from_file("assets/player.gltf")?;

    let player = Player::new(player.model_from_node("Player")?.ok_or(
        ErrorKind::NoNode("Player"),
    )?);

    let assets = GltfLoader::from_file("assets/assets.gltf")?;

    let mut scene = Scene::new(SceneState {});

    scene.register(Arc::new(RwLock::new(Camera::new(&player))));
    scene.register(player);

    let floor = StaticEntity::new(assets.model_from_node("Floor")?.ok_or(
        ErrorKind::NoNode("Floor"),
    )?);
    scene.register(floor);

    Ok(scene)
}

fn entry() -> Result<()> {
    let mut core_loop = CoreLoop::new()?;

    core_loop.set_scene(setup_scene()?)?;
    core_loop.run()?;

    Ok(())
}

fn main() {
    if let Err(e) = env_logger::init() {
        println!("failed to initialize logging: {:?}", e);
        return;
    }

    if let Err(e) = entry() {
        println!("Error: {:?}", e);
        ::std::process::exit(1)
    }
}
