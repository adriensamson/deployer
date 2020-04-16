use crate::error::Error::{IoError, RuntimeError};
use crate::error::Result;
use crate::installation_method::InstallationMethod;
use std::path::Path;
use std::process::Command;

pub struct TarInstallationMethod {
    filename: String,
}

impl TarInstallationMethod {
    pub fn new(filename: &str) -> TarInstallationMethod {
        TarInstallationMethod {
            filename: filename.to_string(),
        }
    }
}

impl InstallationMethod for TarInstallationMethod {
    fn install_to(&self, base_dir: &Path, path: &Path) -> Result<()> {
        let path_as_str = path.as_os_str().to_str().unwrap();
        info!("TAR extract");
        Command::new("tar")
            .args(&["xf", &self.filename, "-C", path_as_str])
            .current_dir(base_dir)
            .status()
            .map_err(IoError)
            .and_then(|s| {
                if s.success() {
                    Ok(())
                } else {
                    Err(RuntimeError(String::from("Error running tar extract")))
                }
            })
    }
}
