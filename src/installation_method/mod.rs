use std::path::PathBuf;
use crate::error::Result;

pub mod noop;

pub trait InstallationMethod {
    fn install_to(&self, path : PathBuf) -> Result<()>;
}
