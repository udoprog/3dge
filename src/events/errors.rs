use gfx::errors as gfx;

#[derive(Debug)]
pub enum Error {
    Gfx(gfx::Error),
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<gfx::Error> for Error {
    fn from(value: gfx::Error) -> Error {
        Error::Gfx(value)
    }
}
