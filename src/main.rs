extern crate winit;
extern crate threedge;
extern crate cgmath;

use cgmath::{Matrix4, Rad, SquareMatrix};
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

use threedge::errors::*;

enum Rotation {
    Left,
    Right,
}

fn entry() -> Result<()> {
    let mut events = threedge::events::winit::WinitEvents::new()?;
    let (window, gfx) = events.setup_gfx()?;

    let mut gfx_loop = gfx.new_loop(&window)?;
    let mut start = SystemTime::now();
    let mut frames = 0u64;
    let mut focused = true;
    let mut rotate_left = false;
    let mut rotate_right = false;
    let mut rotate_up = false;
    let mut rotate_down = false;
    let mut roll_left = false;
    let mut roll_right = false;

    let one_sec = Duration::from_secs(1);
    let ten_ms = Duration::from_millis(10);

    // no rotation
    let rotation_identity = <Matrix4<f32> as SquareMatrix>::identity();

    // TODO: abstract away loop into fully event-based engine.
    'main: loop {
        // only render if focused
        if focused {
            frames += 1;
            let now = SystemTime::now();

            if now.duration_since(start).unwrap() > one_sec {
                println!("fps = {}", frames);
                frames = 0;
                start = now;
            }

            gfx_loop.tick()?;
        } else {
            // avoid freewheeling
            thread::sleep(ten_ms);
        }

        let mut rotation = None;

        if rotate_left {
            rotation = Some(
                rotation.unwrap_or(rotation_identity) * Matrix4::from_angle_y(Rad(-0.1)),
            );
        }

        if rotate_right {
            rotation = Some(
                rotation.unwrap_or(rotation_identity) * Matrix4::from_angle_y(Rad(0.1)),
            );
        }

        if rotate_up {
            rotation = Some(
                rotation.unwrap_or(rotation_identity) * Matrix4::from_angle_x(Rad(-0.1)),
            );
        }

        if rotate_down {
            rotation = Some(
                rotation.unwrap_or(rotation_identity) * Matrix4::from_angle_x(Rad(0.1)),
            );
        }

        if roll_left {
            rotation = Some(
                rotation.unwrap_or(rotation_identity) * Matrix4::from_angle_z(Rad(-0.1)),
            );
        }

        if roll_right {
            rotation = Some(
                rotation.unwrap_or(rotation_identity) * Matrix4::from_angle_z(Rad(0.1)),
            );
        }

        if let Some(rotation) = rotation {
            gfx_loop.rotate_camera(&rotation)?;
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
                    println!("focused: {}", state);
                    focused = state;

                    // reset fps counter when we wake up
                    if focused {
                        frames = 0;
                        start = SystemTime::now();
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
                            rotate_left = state == ElementState::Pressed;
                        }
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::D),
                            state,
                            ..
                        } => {
                            rotate_right = state == ElementState::Pressed;
                        }
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::W),
                            state,
                            ..
                        } => {
                            rotate_up = state == ElementState::Pressed;
                        }
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::S),
                            state,
                            ..
                        } => {
                            rotate_down = state == ElementState::Pressed;
                        }
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Q),
                            state,
                            ..
                        } => {
                            roll_left = state == ElementState::Pressed;
                        }
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::E),
                            state,
                            ..
                        } => {
                            roll_right = state == ElementState::Pressed;
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
