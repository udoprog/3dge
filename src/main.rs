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
use std::time::Instant;
use threedge::camera::Camera;
use threedge::errors::*;
use threedge::events::winit::WinitEvents;
use threedge::gfx::color::Color;
use threedge::gfx::rectangle::Rectangle;
use threedge::gfx_thread::GfxThread;
use threedge::model::Model;
use threedge::player::Player;
use threedge::pressed_keys::{Key, PressedKeys};
use threedge::scheduler::{Scheduler, SelfScheduler};
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

struct FrameState {
    scroll: i32,
    camera: Arc<RwLock<Camera>>,
    logic: Logic,
    pressed_keys: PressedKeys,
    player: Player,
    focus_update: Option<bool>,
    focused: bool,
    exit: bool,
    gfx_thread: GfxThread,
}

impl FrameState {
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

fn entry() -> Result<()> {
    let mut scheduler: Scheduler<FrameState> = Scheduler::new();

    // let test = Model::from_gltf(File::open("models/player.gltf")?);

    let player = Player::new();
    let camera = Arc::new(RwLock::new(Camera::new(&player)));

    let mut events = WinitEvents::new()?;
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

    let target_sleep = Duration::from_millis(10);
    let mut sleep = target_sleep;

    let _target_frame_length = Duration::from_millis(1000 / 60);

    let mut gfx_thread = GfxThread::new(gfx.clone());
    gfx_thread.start(Box::new(camera.clone()));

    let mut frame_state = FrameState {
        scroll: 0i32,
        camera: camera.clone(),
        logic: Logic::new(),
        pressed_keys: PressedKeys::new(),
        player: player,
        focus_update: None,
        focused: true,
        exit: false,
        gfx_thread: gfx_thread,
    };

    scheduler.on_every_tick(Box::new(move |_, frame_state| {
        if frame_state.scroll != 0 {
            let mut camera = frame_state.camera.write().map_err(
                |_| ErrorKind::PoisonError,
            )?;

            let amount = (-frame_state.scroll as f32) * 0.005;

            camera.modify_zoom(amount);

            // reset accumulated scroll amount
            frame_state.scroll = 0i32;
        }

        Ok(())
    }));

    scheduler.on_every_tick(Box::new(|_, frame_state| {
        // perform player transform based on pressed keys
        if let Some(transform) = frame_state.logic.player_transform(
            &frame_state.pressed_keys,
        )
        {
            frame_state.player.transform(&transform)?;
        }

        Ok(())
    }));

    scheduler.on_every_tick(Box::new(|_, mut frame_state| {
        if let Some(state) = frame_state.focus_update.take() {
            frame_state.gfx_thread.enabled(state)?;
            frame_state.focused = state;
        }

        Ok(())
    }));

    scheduler.run_at(
        10,
        Box::new(|sched, _| {
            println!("happened in ten ticks...");
            sched.run_self_at(10);
            Ok(())
        }),
    );

    loop {
        if frame_state.gfx_thread.errored() {
            error!("exiting due to error in gfx thread");
            break;
        }

        shuteye::sleep(sleep);

        let before = Instant::now();

        frame_state.update_pressed_keys(&mut events);

        scheduler.tick(&mut frame_state)?;

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

        if frame_state.exit {
            break;
        }
    }

    frame_state.gfx_thread.stop()?;
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
