use crate::installation_method::InstallationMethod;
use crate::release::Release;
use crate::error::Result;

pub struct Deployement<'a, IM : InstallationMethod> {
    release : Release<'a>,
    installation_method: IM,
}

impl<'a, IM : InstallationMethod> Deployement<'a, IM> {
    pub fn new(release : Release<'a>, installation_method: IM) -> Deployement<IM> {
        Deployement {
            release,
            installation_method,
        }
    }
}

impl<IM : InstallationMethod> Deployement<'_, IM> {
    pub fn run(&self) -> Result<()> {
        self.installation_method.install_to(self.release.get_release_path())?;

        self.release.do_links()?;
        self.release.do_hook("install")?;
        self.release.do_switch()
    }
}
