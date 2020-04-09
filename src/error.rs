use std::io;
use core::result;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    RuntimeError(String),
    ConfigError(String),
}

pub type Result<T> = result::Result<T, Error>;

impl std::convert::From<io::Error> for Error {
    fn from(err : io::Error) -> Error {
        Error::IoError(err)
    }
}
