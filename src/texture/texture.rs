#![allow(dead_code)]

#[derive(Clone)]
pub struct Texture {
    pub image_data: Vec<u8>,
    pub dimensions: (u32, u32),
}

impl Texture {
    pub fn from_raw(image_data: Vec<u8>, dimensions: (u32, u32)) -> Texture {
        Texture {
            image_data: image_data,
            dimensions: dimensions,
        }
    }
}
