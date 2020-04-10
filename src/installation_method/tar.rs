use crate::installation_method::InstallationMethod;
use std::path::PathBuf;
use crate::error::Result;
use std::process::Command;
use crate::error::Error::{RuntimeError, IoError};

pub struct TarInstallationMethod {
    filename : String,
}

impl TarInstallationMethod {
    pub fn new(filename : &str) -> TarInstallationMethod {
        TarInstallationMethod {
            filename: filename.to_string(),
        }
    }
}

impl InstallationMethod for TarInstallationMethod {
    fn install_to(&self, path : PathBuf) -> Result<()> {
        let path_as_str = path.as_os_str().to_str().unwrap();
        info!("TAR extract");
        Command::new("tar")
            .args(&["xf", &self.filename, "-C", path_as_str])
            .status()
            .map_err(IoError)
            .and_then(|s| if s.success() { Ok(())} else { Err(RuntimeError(String::from("Error running tar extract")))})
    }
}
