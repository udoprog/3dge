extern crate env_logger;
extern crate threedge;
extern crate cgmath;

use std::sync::{Arc, RwLock};
use threedge::camera::Camera;
use threedge::core_loop::CoreLoop;
use threedge::core_state::CoreState;
use threedge::errors::*;
use threedge::model::ModelGeometry;
use threedge::player::Player;
use threedge::scene::Scene;

struct SceneState {}

fn setup_scene() -> Result<Scene<CoreState, SceneState>> {
    let player_model = ModelGeometry::from_gltf("models/player.gltf")?;

    let player = Player::new(player_model);
    let camera = Arc::new(RwLock::new(Camera::new(&player)));

    let mut scene = Scene::new(SceneState {});

    scene.register(player);
    scene.register(camera);

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
