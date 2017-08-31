//! # Built-in Textures

use super::errors::*;
use super::texture::Texture;
use image;

/// Access the debug texture.
pub fn debug() -> Result<Texture> {
    let image = image::load_from_memory_with_format(
        include_bytes!("debug_512x512.png"),
        image::ImageFormat::PNG,
    )?
        .to_rgba();

    let dimensions = image.dimensions();
    Ok(Texture::from_raw(image.into_raw(), dimensions))
}
