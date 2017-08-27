use super::errors::*;
use std::marker;

pub trait Window: marker::Sync + marker::Send {
    fn dimensions(&self) -> Result<[u32; 2]>;
}
