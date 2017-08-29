#[macro_use]
extern crate log;
extern crate env_logger;
extern crate winit;
extern crate threedge;
extern crate cgmath;
extern crate shuteye;

use cgmath::{Matrix4, Point3, SquareMatrix, Vector3};
use cgmath::prelude::*;
use std::mem;

use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::time::Instant;
use threedge::camera::{Camera, CameraScroll};
use threedge::errors::*;
use threedge::events::winit::WinitEvents;
use threedge::gfx::color::Color;
use threedge::gfx::rectangle::Rectangle;
use threedge::gfx_thread::GfxThread;
use threedge::player::{Player, PlayerTransform};
use threedge::pressed_keys::{Key, PressedKeys};
use threedge::scene::{Scene, SceneObject};
use threedge::scheduler::{Scheduler, SchedulerSetup, SelfScheduler};
use threedge::texture::builtin as builtin_texture;

struct GameState {
    /// Identity matrix. Nothing happens when multipled with it.
    no_transform: Matrix4<f32>,
    no_movement: Vector3<f32>,
    /// Amount of recorded scrolling between frames.
    scroll: i32,
    /// The state of all pressed keys.
    pressed_keys: PressedKeys,
    /// If focusing should be updated.
    focus_update: Option<bool>,
    /// If the game is focused.
    focused: bool,
    /// If the game should be exited.
    exit: bool,
    /// Graphics thread.
    gfx_thread: GfxThread,
}

impl GameState {
    fn update_pressed_keys(&mut self, events: &mut WinitEvents) {
        events.poll_events(|ev| {
            use winit::Event;
            use winit::DeviceEvent;
            use winit::WindowEvent;

            match ev {
                Event::WindowEvent { event: WindowEvent::Closed, .. } => self.exit = true,
                Event::WindowEvent { event: WindowEvent::Focused(state), .. } => {
                    self.focus_update = Some(state);
                }
                Event::WindowEvent { .. } => {
                    // ignore other window events
                }
                Event::DeviceEvent { event: DeviceEvent::Key(input), .. } => {
                    self.handle_device_event(input);
                }
                Event::DeviceEvent {
                    event: DeviceEvent::Motion { axis: 3, value, .. }, ..
                } => {
                    self.scroll += value as i32;
                }
                Event::DeviceEvent { .. } => {
                    // ignore other device events
                }
                e => {
                    println!("event = {:?}", e);
                }
            }
        });
    }

    fn handle_device_event(&mut self, input: winit::KeyboardInput) {
        use winit::KeyboardInput;
        use winit::VirtualKeyCode;
        use winit::ElementState;

        match input {
            KeyboardInput {
                virtual_keycode: Some(VirtualKeyCode::Escape),
                state: ElementState::Released,
                ..
            } => {
                // only exit if focused
                if !self.exit {
                    self.exit = self.focused;
                }
            }
            KeyboardInput {
                virtual_keycode: Some(VirtualKeyCode::A),
                state,
                ..
            } => {
                self.pressed_keys.set(
                    Key::MoveLeft,
                    state == ElementState::Pressed,
                );
            }
            KeyboardInput {
                virtual_keycode: Some(VirtualKeyCode::D),
                state,
                ..
            } => {
                self.pressed_keys.set(
                    Key::MoveRight,
                    state == ElementState::Pressed,
                );
            }
            KeyboardInput {
                virtual_keycode: Some(VirtualKeyCode::W),
                state,
                ..
            } => {
                self.pressed_keys.set(
                    Key::MoveUp,
                    state == ElementState::Pressed,
                );
            }
            KeyboardInput {
                virtual_keycode: Some(VirtualKeyCode::S),
                state,
                ..
            } => {
                self.pressed_keys.set(
                    Key::MoveDown,
                    state == ElementState::Pressed,
                );
            }
            KeyboardInput {
                virtual_keycode: Some(VirtualKeyCode::Q),
                state,
                ..
            } => {
                self.pressed_keys.set(
                    Key::RollLeft,
                    state == ElementState::Pressed,
                );
            }
            KeyboardInput {
                virtual_keycode: Some(VirtualKeyCode::E),
                state,
                ..
            } => {
                self.pressed_keys.set(
                    Key::RollRight,
                    state == ElementState::Pressed,
                );
            }
            _ => {
                // println!("input = {:?}", input);
            }
        }
    }
}

impl CameraScroll for GameState {
    fn take_scroll(&mut self) -> i32 {
        if self.scroll != 0 {
            return mem::replace(&mut self.scroll, 0);
        }

        0
    }
}

impl PlayerTransform for GameState {
    /// Build player transform for a given frame.
    fn player_transform(&mut self) -> Option<Matrix4<f32>> {
        let mut translation = None;

        if self.pressed_keys.test(Key::MoveLeft) {
            translation = Some(
                translation.unwrap_or(self.no_movement) + Vector3::new(-0.02, 0.0, 0.0),
            );
        }

        if self.pressed_keys.test(Key::MoveRight) {
            translation = Some(
                translation.unwrap_or(self.no_movement) + Vector3::new(0.02, 0.0, 0.0),
            );
        }

        if self.pressed_keys.test(Key::MoveUp) {
            translation = Some(
                translation.unwrap_or(self.no_movement) + Vector3::new(0.0, -0.02, 0.0),
            );
        }

        if self.pressed_keys.test(Key::MoveDown) {
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
    let mut scheduler: Scheduler<GameState> = Scheduler::new();

    // let test = Model::from_gltf(File::open("models/player.gltf")?);

    let mut events = WinitEvents::new()?;
    let mut gfx = events.setup_gfx()?;

    let mut player = Player::new();
    let mut camera = Arc::new(RwLock::new(Camera::new(&player)));

    let color1 = Color::from_rgb(0.0, 0.0, 1.0);

    let rectangle1 = Rectangle::new(
        Point3::new(0.0, 0.0, 0.2),
        Vector3::new(0.0, 0.0, -1.0),
        color1,
    );

    gfx.set_camera(&camera)?;
    gfx.register_geometry(&rectangle1)?;
    gfx.register_geometry(&player)?;

    let mut plane = gfx.new_plane(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
    plane.bind_texture(&builtin_texture::debug()?)?;

    let target_sleep = Duration::from_millis(10);
    let mut sleep = target_sleep;

    let mut gfx_thread = GfxThread::new(gfx.clone());
    gfx_thread.start();

    let mut gs = GameState {
        no_transform: <Matrix4<f32> as SquareMatrix>::identity(),
        no_movement: Vector3::zero(),
        scroll: 0i32,
        pressed_keys: PressedKeys::new(),
        focus_update: None,
        focused: true,
        exit: false,
        gfx_thread: gfx_thread,
    };

    // let mut scene: Scene<GameState> = Scene::new(gs);

    camera.setup_scheduler(&mut scheduler);
    player.setup_scheduler(&mut scheduler);

    scheduler.on_every_tick(Box::new(|_, mut gs| {
        if let Some(state) = gs.focus_update.take() {
            gs.gfx_thread.enabled(state)?;
            gs.focused = state;
        }

        Ok(())
    }));

    loop {
        if gs.gfx_thread.errored() {
            error!("exiting due to error in gfx thread");
            break;
        }

        shuteye::sleep(sleep);

        let before = Instant::now();

        gs.update_pressed_keys(&mut events);

        scheduler.tick(&mut gs)?;

        let elapsed = before.elapsed();

        if let Some(s) = target_sleep.checked_sub(elapsed) {
            sleep = s;
        } else {
            warn!(
                "frame took longer ({:?}) than the desired frame length ({:?}) to execute",
                elapsed,
                target_sleep
            );
        }

        if gs.exit {
            break;
        }
    }

    gs.gfx_thread.stop()?;
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
