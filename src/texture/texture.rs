pub struct Texture {
    image_data: Vec<u8>,
}

impl Texture {
    pub fn from_raw(image_data: Vec<u8>) -> Texture {
        Texture { image_data: image_data }
    }
}
