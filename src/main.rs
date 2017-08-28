#[macro_use]
extern crate log;
extern crate env_logger;
extern crate winit;
extern crate threedge;
extern crate cgmath;
extern crate shuteye;

use cgmath::{Matrix4, Point3, SquareMatrix, Vector3};
use cgmath::prelude::*;
use std::fs::File;

use std::sync::{Arc, RwLock};
use std::time::Duration;
use threedge::camera::Camera;
use threedge::errors::*;
use threedge::gfx::color::Color;
use threedge::gfx::rectangle::Rectangle;
use threedge::gfx_thread::GfxThread;
use threedge::model::Model;
use threedge::player::Player;
use threedge::pressed_keys::{Key, PressedKeys};
use threedge::texture::builtin as builtin_texture;

struct Logic {
    /// Identity matrix. Nothing happens when multipled with it.
    no_transform: Matrix4<f32>,
    no_movement: Vector3<f32>,
}

impl Logic {
    pub fn new() -> Logic {
        Logic {
            no_transform: <Matrix4<f32> as SquareMatrix>::identity(),
            no_movement: Vector3::zero(),
        }
    }

    /// Build player transform for a given frame.
    fn player_transform(&self, keys: &PressedKeys) -> Option<Matrix4<f32>> {
        let mut translation = None;

        if keys.test(Key::MoveLeft) {
            translation = Some(
                translation.unwrap_or(self.no_movement) + Vector3::new(-0.02, 0.0, 0.0),
            );
        }

        if keys.test(Key::MoveRight) {
            translation = Some(
                translation.unwrap_or(self.no_movement) + Vector3::new(0.02, 0.0, 0.0),
            );
        }

        if keys.test(Key::MoveUp) {
            translation = Some(
                translation.unwrap_or(self.no_movement) + Vector3::new(0.0, -0.02, 0.0),
            );
        }

        if keys.test(Key::MoveDown) {
            translation = Some(
                translation.unwrap_or(self.no_movement) + Vector3::new(0.0, 0.02, 0.0),
            );
        }

        let mut transform = None;

        if let Some(translation) = translation {
            transform = Some(
                transform.unwrap_or(self.no_transform) *
                    Matrix4::from_translation(translation),
            )
        }

        transform
    }
}

fn entry() -> Result<()> {
    let test = Model::from_gltf(File::open("models/player.gltf")?);

    let mut player = Player::new();
    let camera = Arc::new(RwLock::new(Camera::new(&player)));

    let mut events = threedge::events::winit::WinitEvents::new()?;
    let mut gfx = events.setup_gfx()?;

    let color1 = Color::from_rgb(0.0, 0.0, 1.0);

    let rectangle1 = Rectangle::new(
        Point3::new(0.0, 0.0, 0.2),
        Vector3::new(0.0, 0.0, -1.0),
        color1,
    );

    gfx.register_geometry(&rectangle1)?;
    gfx.register_geometry(&player)?;

    let mut plane = gfx.new_plane(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
    plane.bind_texture(&builtin_texture::debug()?)?;

    let mut focus_update = None;
    let mut focused = true;
    let mut pressed_keys = PressedKeys::new();
    let mut scroll_amount = 0i32;

    let ten_ms = Duration::from_millis(10);
    let logic = Logic::new();

    let _target_frame_length = Duration::from_millis(1000 / 60);

    let mut gfx_thread = GfxThread::new(gfx.clone());
    gfx_thread.start(Box::new(camera.clone()));

    // TODO: abstract away loop into fully event-based engine.
    'main: loop {
        if gfx_thread.errored() {
            error!("exiting due to error in gfx thread");
            break;
        }

        shuteye::sleep(ten_ms);

        if let Some(state) = focus_update.take() {
            gfx_thread.enabled(state)?;
            focused = state;
        }

        if let Some(transform) = logic.player_transform(&pressed_keys) {
            player.transform(&transform)?;
        }

        if scroll_amount != 0 {
            let mut camera = camera.write().map_err(|_| ErrorKind::PoisonError)?;
            let amount = (-scroll_amount as f32) * 0.005;
            camera.modify_zoom(amount);
            scroll_amount = 0i32;
        }

        let mut exit = false;

        events.poll_events(|ev| {
            use winit::Event;
            use winit::DeviceEvent;
            use winit::WindowEvent;
            use winit::KeyboardInput;
            use winit::VirtualKeyCode;
            use winit::ElementState;

            match ev {
                Event::WindowEvent { event: WindowEvent::Closed, .. } => exit = true,
                Event::WindowEvent { event: WindowEvent::Focused(state), .. } => {
                    focus_update = Some(state);
                }
                Event::WindowEvent { .. } => {
                    // ignore other window events
                }
                Event::DeviceEvent { event: DeviceEvent::Key(input), .. } => {
                    match input {
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            state: ElementState::Released,
                            ..
                        } => {
                            // only exit if focused
                            if !exit {
                                exit = focused;
                            }
                        }
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::A),
                            state,
                            ..
                        } => {
                            pressed_keys.set(Key::MoveLeft, state == ElementState::Pressed);
                        }
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::D),
                            state,
                            ..
                        } => {
                            pressed_keys.set(Key::MoveRight, state == ElementState::Pressed);
                        }
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::W),
                            state,
                            ..
                        } => {
                            pressed_keys.set(Key::MoveUp, state == ElementState::Pressed);
                        }
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::S),
                            state,
                            ..
                        } => {
                            pressed_keys.set(Key::MoveDown, state == ElementState::Pressed);
                        }
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Q),
                            state,
                            ..
                        } => {
                            pressed_keys.set(Key::RollLeft, state == ElementState::Pressed);
                        }
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::E),
                            state,
                            ..
                        } => {
                            pressed_keys.set(Key::RollRight, state == ElementState::Pressed);
                        }
                        _ => {
                            // println!("input = {:?}", input);
                        }
                    }
                }
                Event::DeviceEvent {
                    event: DeviceEvent::Motion { axis: 3, value, .. }, ..
                } => {
                    scroll_amount += value as i32;
                }
                Event::DeviceEvent { .. } => {
                    // ignore other device events
                }
                e => {
                    println!("event = {:?}", e);
                }
            }
        });

        if exit {
            break 'main;
        }
    }

    gfx_thread.stop()?;
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
