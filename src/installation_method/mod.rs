use std::path::PathBuf;
use std::io;

pub mod noop;

pub trait InstallationMethod {
    fn install_to(&self, path : PathBuf) -> io::Result<()>;
}
