#![feature(shared)]

#[cfg(feature = "gfx-vulkan")]
#[macro_use]
extern crate vulkano;
#[macro_use]
#[cfg(feature = "gfx-vulkan")]
extern crate vulkano_shader_derive;
#[cfg(feature = "gfx-vulkan")]
extern crate vulkano_win;
#[macro_use]
extern crate error_chain;

extern crate winit;
extern crate cgmath;
extern crate bit_vec;

pub mod events;
pub mod errors;
pub mod gfx;
pub mod sg;
pub mod pressed_keys;
pub mod fps_counter;
pub mod game;
pub mod player;
pub mod camera;
