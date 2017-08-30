extern crate env_logger;
extern crate threedge;
extern crate cgmath;

use cgmath::{Point3, Vector3};
use std::sync::{Arc, RwLock};
use threedge::camera::Camera;
use threedge::core_loop::CoreLoop;
use threedge::core_state::CoreState;
use threedge::errors::*;
use threedge::gfx::color::Color;
use threedge::gfx::rectangle::Rectangle;
use threedge::player::Player;
use threedge::scene::Scene;

struct SceneState {}

fn setup_scene() -> Scene<CoreState, SceneState> {
    // let player_model = Model::from_gltf(File::open("models/player.gltf")?);

    let player = Player::new();
    let camera = Arc::new(RwLock::new(Camera::new(&player)));

    let rectangle1 = Rectangle::new(
        Point3::new(0.0, 0.0, 0.2),
        Vector3::new(0.0, 1.0, 0.0),
        Color::from_rgb(0.0, 0.0, 1.0),
    );

    let mut scene = Scene::new(SceneState {});

    scene.register(rectangle1);
    scene.register(player);
    scene.register(camera);

    scene
}

fn entry() -> Result<()> {
    let mut core_loop = CoreLoop::new()?;

    core_loop.set_scene(setup_scene())?;
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
