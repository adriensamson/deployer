use std::path::PathBuf;
use crate::error::Result;
use crate::installation_method::noop::NoopInstallationMethod;
use serde::{Deserialize, Serialize};

pub mod noop;

pub trait InstallationMethod {
    fn install_to(&self, path : PathBuf) -> Result<()>;
}

pub fn installation_method_from_config(config : &InstallationMethodConfig) -> Box<dyn InstallationMethod> {
    match config {
        InstallationMethodConfig::Noop => Box::new(NoopInstallationMethod {}),
    }
}

#[derive(Serialize, Deserialize)]
pub enum InstallationMethodConfig {
    Noop,
}
