use super::errors::*;

pub trait Window {
    fn dimensions(&self) -> Result<[u32; 2]>;
}
