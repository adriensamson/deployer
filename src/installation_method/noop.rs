use crate::installation_method::InstallationMethod;
use std::path::Path;
use crate::error::Result;

pub struct NoopInstallationMethod {}

impl InstallationMethod for NoopInstallationMethod {
    fn install_to(&self, _base_dir : &Path, _path : &Path) -> Result<()> {
        info!("NOOP");
        Ok(())
    }
}
