use crate::installation_method::InstallationMethod;
use std::path::PathBuf;
use std::io;

pub struct NoopInstallationMethod {}

impl InstallationMethod for NoopInstallationMethod {
    fn install_to(&self, _path : PathBuf) -> io::Result<()> {
        Ok(())
    }
}
