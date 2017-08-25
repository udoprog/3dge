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
