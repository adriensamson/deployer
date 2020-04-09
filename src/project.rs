use std::path::{PathBuf, Path};
use std::env::current_dir;
use std::fs::{read_dir, read_link};
use crate::release::Release;
use crate::installation_method::InstallationMethod;
use std::process::Command;
use crate::error::{Result, Error};
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use crate::error::Error::RuntimeError;

pub struct Project {
    pub(crate) base_dir : PathBuf,
}

impl Project {
    pub fn from_current_dir() -> Result<Project> {
        let base_dir = current_dir()?;
        Ok(Project {
            base_dir,
        })
    }

    pub fn rollback(&self) -> Result<()> {
        let rollback_to = find_rollback(&self.base_dir)?;
        match rollback_to {
            None => {
                error!("No release to rollback");
                Err(RuntimeError(String::from("Cannot rollback")))
            },
            Some(path) => {
                info!("Rollbacking to {:?}", path);
                let release = Release::new(&self, path);
                release.do_switch()?;
                info!("Rollback OK");
                Ok(())
            }
        }
    }

    pub fn deploy<IM : InstallationMethod>(&self, im : IM) -> Result<()> {
        let release_path = get_date_str()?;
        info!("Installing to {:?}", release_path);
        let release = Release::new(&self, release_path);

        im.install_to(release.get_release_path())?;

        release.do_links()?;
        release.do_hook("install")?;
        release.do_switch()?;

        info!("Deploy OK");
        Ok(())
    }
}

fn find_rollback(base_dir : &Path) -> Result<Option<OsString>> {
    let current = read_link(base_dir.join("current")).map_or(None, Some);
    let mut entries = Vec::new();
    for res in read_dir(base_dir.join("releases"))? {
        if let Ok(entry) = res {
            if Some(entry.path()) == current {
                continue;
            }
            entries.push(entry.file_name());
        }
    }
    entries.sort_by(|a, b| a.cmp(b).reverse());
    entries.truncate(1);
    Ok(entries.pop())
}

fn get_date_str() -> Result<OsString> {
    let output = Command::new("date")
        .arg("+%Y-%m-%d-%H-%M-%S")
        .output()
        .unwrap();
    if !output.status.success() {
        return Err(Error::RuntimeError(String::from("Error running date")));
    }
    let mut stdout = output.stdout;
    stdout.pop();
    Ok(OsString::from_vec(stdout))
}
