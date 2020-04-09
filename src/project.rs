use std::path::{PathBuf, Path};
use std::env::current_dir;
use std::fs::{read_dir, read_link};
use crate::release::Release;
use crate::installation_method::InstallationMethod;
use std::process::Command;
use crate::deployement::Deployement;
use crate::error::{Result, Error};
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;

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
        let release = Release::new(&self, find_rollback(&self.base_dir).unwrap().unwrap());
        release.do_switch()
    }

    pub fn deploy<IM : InstallationMethod>(&self, im : IM) -> Result<()> {
        let release = Release::new(&self, get_date_str()?);
        let deployement = Deployement::new(release, im);
        deployement.run()
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
