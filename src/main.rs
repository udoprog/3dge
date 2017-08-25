extern crate winit;
extern crate threedge;
extern crate cgmath;

use cgmath::{Matrix4, Rad, SquareMatrix};
use std::thread;
use std::time::Duration;

use threedge::errors::*;
use threedge::fps_counter::FpsCounter;
use threedge::pressed_keys::{Key, PressedKeys};

struct Logic {
    /// Identity matrix. Nothing happens when multipled with it.
    identity: Matrix4<f32>,
}

impl Logic {
    pub fn new() -> Logic {
        Logic { identity: <Matrix4<f32> as SquareMatrix>::identity() }
    }

    /// Build movement for a given frame.
    fn build_movement(&self, keys: &PressedKeys) -> Option<Matrix4<f32>> {
        let mut movement = None;

        if keys.test(Key::MoveLeft) {
            movement = Some(
                movement.unwrap_or(self.identity) * Matrix4::from_angle_y(Rad(-0.1)),
            );
        }

        if keys.test(Key::MoveRight) {
            movement = Some(
                movement.unwrap_or(self.identity) * Matrix4::from_angle_y(Rad(0.1)),
            );
        }

        if keys.test(Key::MoveUp) {
            movement = Some(
                movement.unwrap_or(self.identity) * Matrix4::from_angle_x(Rad(-0.1)),
            );
        }

        if keys.test(Key::MoveDown) {
            movement = Some(
                movement.unwrap_or(self.identity) * Matrix4::from_angle_x(Rad(0.1)),
            );
        }

        if keys.test(Key::RollLeft) {
            movement = Some(
                movement.unwrap_or(self.identity) * Matrix4::from_angle_z(Rad(-0.1)),
            );
        }

        if keys.test(Key::RollRight) {
            movement = Some(
                movement.unwrap_or(self.identity) * Matrix4::from_angle_z(Rad(0.1)),
            );
        }

        movement
    }
}

fn entry() -> Result<()> {
    let mut events = threedge::events::winit::WinitEvents::new()?;
    let (window, gfx) = events.setup_gfx()?;

    let mut refocus = false;
    let mut gfx_loop = gfx.new_loop(&window)?;
    let mut focused = true;
    let mut pressed_keys = PressedKeys::new();
    let ten_ms = Duration::from_millis(10);
    let logic = Logic::new();

    let mut fps_counter = FpsCounter::new(|fps| {
        println!("fps = {}", fps);
        Ok(())
    });

    let target_frame_length = Duration::from_millis(1000 / 60);

    // TODO: abstract away loop into fully event-based engine.
    'main: loop {
        if refocus {
            fps_counter.reset()?;
            refocus = false;
        }

        // only render if focused
        if focused {
            gfx_loop.tick()?;
            fps_counter.tick()?;
        } else {
            // avoid freewheeling
            thread::sleep(ten_ms);
        }

        if let Some(movement) = logic.build_movement(&pressed_keys) {
            gfx_loop.translate_world(&movement)?;
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
                    focused = state;

                    // reset fps counter when we wake up
                    if focused {
                        refocus = true;
                    }
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

    Ok(())
}

fn main() {
    if let Err(e) = entry() {
        println!("Error: {:?}", e);
        ::std::process::exit(1)
    }
}
