use crate::installation_method::InstallationMethod;
use std::path::PathBuf;
use crate::error::Result;

pub struct NoopInstallationMethod {}

impl InstallationMethod for NoopInstallationMethod {
    fn install_to(&self, _path : PathBuf) -> Result<()> {
        info!("NOOP");
        Ok(())
    }
}
