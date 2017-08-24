use events::errors as events;
use gfx::errors as gfx;

#[derive(Debug)]
pub enum Error {
    Gfx(gfx::Error),
    Events(events::Error),
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<gfx::Error> for Error {
    fn from(value: gfx::Error) -> Error {
        Error::Gfx(value)
    }
}

impl From<events::Error> for Error {
    fn from(value: events::Error) -> Error {
        Error::Events(value)
    }
}
