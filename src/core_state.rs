use super::camera::CameraScroll;
use super::errors::*;
use super::events::winit::WinitEvents;
use super::gfx_thread::GfxThread;
use super::player::PlayerTransform;
use super::pressed_keys::{Key, PressedKeys};
use cgmath::{Matrix4, Vector3};
use gfx::Gfx;
use std::mem;
use winit;

pub struct CoreState {
    /// Identity matrix. Nothing happens when multipled with it.
    pub no_transform: Matrix4<f32>,
    /// No movement.
    pub no_movement: Vector3<f32>,
    /// The state of all pressed keys.
    pub pressed_keys: PressedKeys,
    /// If focusing should be updated.
    pub focus_update: Option<bool>,
    /// If the game is focused.
    pub focused: bool,
    /// If the game should be exited.
    pub exit: bool,
    /// Amount of recorded scrolling between frames.
    pub scroll: i32,
    /// Events associated with the core state.
    pub events: WinitEvents,
    /// Graphics subsystem.
    pub gfx: Gfx,
    /// Graphics thread handle.
    pub gfx_thread: GfxThread,
}

impl CoreState {
    pub fn update_pressed_keys(&mut self) {
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
