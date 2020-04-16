use crate::error::Result;
use crate::installation_method::git::GitInstallationMethod;
use crate::installation_method::noop::NoopInstallationMethod;
use crate::installation_method::tar::TarInstallationMethod;
use serde::{Deserialize, Serialize};
use std::path::Path;

mod git;
mod noop;
mod tar;

pub trait InstallationMethod {
    fn install_to(&self, base_dir: &Path, path: &Path) -> Result<()>;
}

pub fn installation_method_from_config(
    config: &InstallationMethodConfig,
) -> Box<dyn InstallationMethod> {
    match config {
        InstallationMethodConfig::Noop => Box::new(NoopInstallationMethod {}),
        InstallationMethodConfig::Git { source_dir, branch } => {
            Box::new(GitInstallationMethod::new(source_dir, branch))
        }
        InstallationMethodConfig::Tar { filename } => {
            Box::new(TarInstallationMethod::new(filename))
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum InstallationMethodConfig {
    Noop,
    Git { source_dir: String, branch: String },
    Tar { filename: String },
}
