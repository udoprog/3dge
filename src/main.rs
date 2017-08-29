#[macro_use]
extern crate log;
extern crate env_logger;
extern crate winit;
extern crate threedge;
extern crate cgmath;
extern crate shuteye;

use cgmath::{Matrix4, Point3, SquareMatrix, Vector3};
use cgmath::prelude::*;
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::time::Instant;
use threedge::camera::{Camera, CameraScroll};
use threedge::errors::*;
use threedge::events::winit::WinitEvents;
use threedge::gfx::Gfx;
use threedge::gfx::color::Color;
use threedge::gfx::rectangle::Rectangle;
use threedge::gfx_thread::GfxThread;
use threedge::player::{Player, PlayerTransform};
use threedge::pressed_keys::{Key, PressedKeys};
use threedge::scene::Scene;

struct CoreState {
    /// Identity matrix. Nothing happens when multipled with it.
    no_transform: Matrix4<f32>,
    /// No movement.
    no_movement: Vector3<f32>,
    /// The state of all pressed keys.
    pressed_keys: PressedKeys,
    /// If focusing should be updated.
    focus_update: Option<bool>,
    /// If the game is focused.
    focused: bool,
    /// If the game should be exited.
    exit: bool,
    /// Amount of recorded scrolling between frames.
    scroll: i32,
    /// Events associated with the core state.
    events: WinitEvents,
    /// Graphics subsystem.
    gfx: Box<Gfx>,
    /// Graphics thread handle.
    gfx_thread: GfxThread,
}

impl CoreState {
    fn update_pressed_keys(&mut self) {
        let mut events = Vec::new();

        self.events.poll_events(|ev| events.push(ev));

        for ev in events {
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
        }
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

impl CameraScroll for CoreState {
    fn take_scroll(&mut self) -> Result<i32> {
        if self.scroll != 0 {
            return Ok(mem::replace(&mut self.scroll, 0));
        }

        Ok(0)
    }
}

impl PlayerTransform for CoreState {
    /// Build player transform for a given frame.
    fn player_transform(&mut self) -> Result<Option<Matrix4<f32>>> {
        let mut translation = None;

        let pressed_keys = &self.pressed_keys;

        if pressed_keys.test(Key::MoveLeft) {
            translation = Some(
                translation.unwrap_or(self.no_movement) + Vector3::new(-0.02, 0.0, 0.0),
            );
        }

        if pressed_keys.test(Key::MoveRight) {
            translation = Some(
                translation.unwrap_or(self.no_movement) + Vector3::new(0.02, 0.0, 0.0),
            );
        }

        if pressed_keys.test(Key::MoveUp) {
            translation = Some(
                translation.unwrap_or(self.no_movement) + Vector3::new(0.0, -0.02, 0.0),
            );
        }

        if pressed_keys.test(Key::MoveDown) {
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

        Ok(transform)
    }
}

struct SceneState {}

fn setup_scene() -> Scene<CoreState, SceneState> {
    let player = Player::new();
    let camera = Arc::new(RwLock::new(Camera::new(&player)));

    let color1 = Color::from_rgb(0.0, 0.0, 1.0);

    let rectangle1 = Rectangle::new(
        Point3::new(0.0, 0.0, 0.2),
        Vector3::new(0.0, 0.0, -1.0),
        color1,
    );

    let mut scene: Scene<CoreState, _> = Scene::new(SceneState {});

    scene.register(rectangle1);
    scene.register(player);
    scene.register(camera);

    scene
}

fn entry() -> Result<()> {
    // let test = Model::from_gltf(File::open("models/player.gltf")?);

    let events = WinitEvents::new()?;
    let mut gfx = events.setup_gfx()?;

    let mut core = CoreState {
        no_transform: <Matrix4<f32> as SquareMatrix>::identity(),
        no_movement: Vector3::zero(),
        pressed_keys: PressedKeys::new(),
        focus_update: None,
        focused: true,
        exit: false,
        scroll: 0i32,
        gfx: gfx.clone(),
        gfx_thread: GfxThread::new(gfx.clone()),
        events: events,
    };

    let target_sleep = Duration::from_millis(10);
    let mut sleep = target_sleep;

    core.gfx_thread.start();

    let core = Rc::new(RefCell::new(core));

    {
        let mut scene = setup_scene();

        scene.scheduler.on_every_tick(Box::new(|_, gs| {
            let mut core = gs.core.try_borrow_mut()?;

            if let Some(state) = core.focus_update.take() {
                core.gfx_thread.enabled(state)?;
                core.focused = state;
            }

            Ok(())
        }));

        scene.scheduler.on_every_tick(Box::new(|_, gs| {
            gs.core.try_borrow_mut()?.update_pressed_keys();
            Ok(())
        }));

        scene.setup(gfx.as_mut())?;

        loop {
            if core.try_borrow()?.gfx_thread.errored() {
                error!("exiting due to error in gfx thread");
                break;
            }

            shuteye::sleep(sleep);

            let before = Instant::now();

            scene.tick(core.clone())?;

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

            if core.try_borrow()?.exit {
                break;
            }
        }
    }

    core.try_borrow_mut()?.gfx_thread.stop()?;
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
