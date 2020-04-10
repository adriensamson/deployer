use std::path::{PathBuf, Path};
use std::env::current_dir;
use std::fs::{read_dir, read_link, write, read_to_string};
use crate::release::{Release, ReleaseState};
use crate::installation_method::{InstallationMethod, installation_method_from_config, InstallationMethodConfig};
use std::process::Command;
use crate::error::{Result, Error};
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use crate::error::Error::{RuntimeError, ConfigError, IoError};
use serde::{Deserialize, Serialize};
use std::io::ErrorKind;

pub struct Project {
    pub(crate) base_dir : PathBuf,
    installation_method : Box<dyn InstallationMethod>,
}

impl Project {
    pub fn from_current_dir() -> Result<Project> {
        let base_dir = current_dir()?;
        let file_content = read_to_string(base_dir.join("deployer.toml"))
            .map_err(| err| if err.kind() == ErrorKind::NotFound { ConfigError(String::from("deployer.toml not found"))} else { IoError(err) })?;
        let config : ProjectConfig = toml::from_str(&file_content)?;
        Ok(Project {
            base_dir,
            installation_method: installation_method_from_config(&config.installation_method),
        })
    }

    pub fn init(config : &ProjectConfig) -> Result<Project> {
        let base_dir = current_dir()?;
        write(base_dir.join("deployer.toml"), toml::to_string(config).unwrap())?;
        Ok(Project {
            base_dir,
            installation_method: installation_method_from_config(&config.installation_method),
        })
    }

    pub fn rollback(&self) -> Result<()> {
        let current = self.read_current().map(|p| OsString::from(p.file_name().unwrap()));
        let rollback_to = self.find_rollback()?;
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
                if let Some(old_release) = current {
                    let mut old = Release::new(&self, old_release);
                    old.change_state(ReleaseState::Rollbacked)?;
                }
                Ok(())
            }
        }
    }

    pub fn deploy(&self) -> Result<()> {
        let release_path = ReleaseState::Installing.new_path_for(&get_date_str()?);
        info!("Installing to {:?}", release_path);
        let mut release = Release::new(&self, release_path);

        self.installation_method.install_to(release.get_release_path())?;

        release.do_links()?;
        release.do_hook("install")?;
        release.change_state(ReleaseState::Normal)?;
        release.do_switch()?;

        info!("Deploy OK");
        Ok(())
    }

    pub(crate) fn read_current(&self) -> Option<PathBuf> {
        read_link(self.base_dir.join("current")).map_or(None, Some)
    }

    fn find_rollback(&self) -> Result<Option<OsString>> {
        let current = self.read_current();
        let mut entries = Vec::new();
        for res in read_dir(self.base_dir.join("releases"))? {
            if let Ok(entry) = res {
                if Some(entry.path()) == current {
                    continue;
                }
                let filename = entry.file_name();
                let state = ReleaseState::from_path(&filename);
                if state == ReleaseState::Normal {
                    entries.push(entry.file_name());
                }
            }
        }
        entries.sort_by(|a, b| a.cmp(b).reverse());
        entries.truncate(1);
        Ok(entries.pop())
    }
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

#[derive(Serialize, Deserialize)]
pub struct ProjectConfig {
    pub installation_method: InstallationMethodConfig,
}
