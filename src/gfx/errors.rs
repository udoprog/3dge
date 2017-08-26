use std::error;
use std::fmt;

#[cfg(feature = "gfx-vulkan")]
pub mod vulkan {
    pub use super::super::vulkan::errors::*;
}

#[cfg(not(feature = "gfx-vulkan"))]
pub mod vulkan {
    #[derive(Debug)]
    pub enum ErrorKind {
    }
}

#[derive(Debug)]
pub enum Error {
    PoisonError,
    /// Returned if a backend does not support the given shader.
    UnsupportedShader,
    Vulkan(vulkan::ErrorKind),
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<vulkan::ErrorKind> for Error {
    fn from(value: vulkan::ErrorKind) -> Error {
        Error::Vulkan(value)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "graphics backend error"
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "graphics backend error")
    }
}
