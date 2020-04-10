use crate::installation_method::InstallationMethod;
use std::path::PathBuf;
use crate::error::Result;
use std::process::Command;
use crate::error::Error::{RuntimeError, IoError};

pub struct GitInstallationMethod {
    source_dir : String,
    branch : String,
}

impl GitInstallationMethod {
    pub fn new(source_dir : &str, branch : &str) -> GitInstallationMethod {
        GitInstallationMethod {
            source_dir: source_dir.to_string(),
            branch: branch.to_string(),
        }
    }
}

impl InstallationMethod for GitInstallationMethod {
    fn install_to(&self, path : PathBuf) -> Result<()> {
        let path_as_str = path.as_os_str().to_str().unwrap();
        info!("GIT fetch origin");
        Command::new("git")
            .args(&["fetch", "origin"])
            .current_dir(&self.source_dir)
            .status()
            .map_err(IoError)
            .and_then(|s| if s.success() { Ok(())} else { Err(RuntimeError(String::from("Error running git fetch")))})?;
        info!("GIT clone");
        Command::new("git")
            .args(&["clone", "--no-local", "--depth", "1", "--recurse-submodules", "--branch", &self.branch, &self.source_dir, path_as_str])
            .status()
            .map_err(IoError)
            .and_then(|s| if s.success() { Ok(())} else { Err(RuntimeError(String::from("Error running git fetch")))})?;
        info!("GIT rm .git");
        Command::new("find")
            .args(&[path_as_str, "-name", ".git", "-exec", "rm", "-rf", "{}", "+"])
            .status()
            .map_err(IoError)
            .and_then(|s| if s.success() { Ok(())} else { Err(RuntimeError(String::from("Error running git fetch")))})?;
        Ok(())
    }
}
