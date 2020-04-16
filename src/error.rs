use core::result;
use std::io;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    RuntimeError(String),
    ConfigError(String),
}

pub type Result<T> = result::Result<T, Error>;

impl std::convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl std::convert::From<toml::de::Error> for Error {
    fn from(_err: toml::de::Error) -> Error {
        Error::ConfigError(String::from("Error while parsing config"))
    }
}
