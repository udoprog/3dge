#![recursion_limit="128"]
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
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

extern crate winit;
extern crate cgmath;
extern crate bit_vec;
extern crate image;
extern crate gltf;
extern crate gltf_utils;
extern crate gltf_importer;
extern crate shuteye;

pub mod gltf_loader;
pub mod events;
pub mod errors;
pub mod gfx;
pub mod sg;
pub mod pressed_keys;
pub mod fps_counter;
pub mod player;
pub mod static_entity;
pub mod camera;
pub mod texture;
pub mod gfx_thread;
pub mod model;
pub mod scheduler;
pub mod scene;
pub mod scene_object;
pub mod boxed_scene;
pub mod into_boxed_scene;
pub mod core_loop;
pub mod core_state;
