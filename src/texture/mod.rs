//! # Texture-handling utilities

pub mod builtin;
pub mod texture;
pub mod errors;

use self::errors::*;

pub use self::texture::Texture;
use image;
use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

const IMAGE_PNG: &'static str = "image/png";

pub fn mime_type_from_file_ext(path: &Path) -> Result<&str> {
    match path.extension().and_then(OsStr::to_str) {
        Some("png") => Ok(IMAGE_PNG),
        ext => Err(
            ErrorKind::UnsupportedExtension(ext.map(String::from)).into(),
        ),
    }
}

pub fn load_from_path<P: AsRef<Path>>(mime_type: Option<&str>, path: P) -> Result<Texture> {
    let path = path.as_ref();

    let mime_type = if let Some(mime_type) = mime_type {
        mime_type
    } else {
        mime_type_from_file_ext(path)?
    };

    match mime_type.as_ref() {
        IMAGE_PNG => {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let image = image::load(reader, image::ImageFormat::PNG)?.to_rgba();
            let dimensions = image.dimensions();
            Ok(Texture::from_raw(image.into_raw(), dimensions))
        }
        _ => Err(ErrorKind::UnsupportedMimeType(mime_type.to_string()).into()),
    }
}

pub fn load_from_memory(mime_type: &str, bytes: &[u8]) -> Result<Texture> {
    match mime_type {
        IMAGE_PNG => {
            let image = image::load_from_memory_with_format(bytes, image::ImageFormat::PNG)?
                .to_rgba();
            let dimensions = image.dimensions();
            Ok(Texture::from_raw(image.into_raw(), dimensions))
        }
        _ => Err(ErrorKind::UnsupportedMimeType(mime_type.to_string()).into()),
    }
}
