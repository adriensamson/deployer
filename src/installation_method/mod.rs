use std::path::PathBuf;
use crate::error::Result;
use crate::installation_method::noop::NoopInstallationMethod;
use serde::{Deserialize, Serialize};
use crate::installation_method::git::GitInstallationMethod;

mod noop;
mod git;

pub trait InstallationMethod {
    fn install_to(&self, path : PathBuf) -> Result<()>;
}

pub fn installation_method_from_config(config : &InstallationMethodConfig) -> Box<dyn InstallationMethod> {
    match config {
        InstallationMethodConfig::Noop => Box::new(NoopInstallationMethod {}),
        InstallationMethodConfig::Git {source_dir, branch} => Box::new(GitInstallationMethod::new(source_dir, branch)),
    }
}

#[derive(Serialize, Deserialize)]
pub enum InstallationMethodConfig {
    Noop,
    Git { source_dir : String, branch : String},
}
