use crate::error::Error::RuntimeError;
use crate::error::Result;
use crate::project::Project;
use std::ffi::{OsStr, OsString};
use std::fs::{create_dir_all, read_to_string, remove_file, rename};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::ffi::OsStringExt;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use std::process::Command;

#[derive(Eq, PartialEq)]
pub enum ReleaseState {
    Normal,
    Installing,
    Rollbacked,
}

impl ReleaseState {
    pub fn from_path(path: &OsStr) -> ReleaseState {
        match path.as_bytes()[0] as char {
            'i' => ReleaseState::Installing,
            'r' => ReleaseState::Rollbacked,
            _ => ReleaseState::Normal,
        }
    }

    pub fn new_path_for(&self, path: &OsStr) -> OsString {
        let mut vec = Vec::from(path.as_bytes());
        match vec[0] as char {
            'i' | 'r' => {
                vec.remove(0);
            }
            _ => {}
        }
        match self {
            ReleaseState::Normal => {}
            ReleaseState::Rollbacked => vec.insert(0, b'r'),
            ReleaseState::Installing => vec.insert(0, b'i'),
        }
        OsString::from_vec(vec)
    }
}

pub struct Release<'a> {
    project: &'a Project,
    release: OsString,
    state: ReleaseState,
}

impl<'a> Release<'a> {
    pub(crate) fn new(project: &'a Project, release: OsString) -> Release<'a> {
        let r = Release {
            project,
            state: ReleaseState::from_path(&release),
            release,
        };
        create_dir_all(r.get_release_path()).unwrap();
        r
    }
}

impl Release<'_> {
    pub(crate) fn get_release_path(&self) -> PathBuf {
        self.project.base_dir.join("releases").join(&self.release)
    }

    pub fn do_switch(&self) -> Result<()> {
        if self.state != ReleaseState::Normal {
            return Err(RuntimeError(String::from("Cannot switch release")));
        }
        let current_path = self.project.base_dir.join("current");
        if current_path.exists() {
            remove_file(&current_path)?;
        }
        info!("switch current");
        symlink(self.get_release_path(), &current_path)?;
        self.do_hook("switch")
    }

    pub fn do_links(&self) -> Result<()> {
        info!("Creating links");
        let links = read_to_string(self.project.base_dir.join("deployer.links"))
            .unwrap_or_else(|_| String::from(""));
        for line in links.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts.as_slice() {
                [from, to] => {
                    let dest_path = self.get_release_path().join(to);
                    create_dir_all(dest_path.parent().unwrap())?;
                    symlink(self.project.base_dir.join("shared").join(from), dest_path)?;
                }
                [path] => {
                    let dest_path = self.get_release_path().join(path);
                    create_dir_all(dest_path.parent().unwrap())?;
                    symlink(self.project.base_dir.join("shared").join(path), dest_path)?;
                }
                [] => {}
                _ => eprintln!("Bad link line: {}", line),
            }
        }
        Ok(())
    }

    pub fn do_hook(&self, hook: &str) -> Result<()> {
        info!("Running {} hook", hook);
        let path = self.project.base_dir.join(format!("deployer.{}", hook));
        if path.exists() {
            let status = Command::new(path)
                .current_dir(self.get_release_path())
                .status()?;
            if !status.success() {
                error!("{} hook failed", hook);
                return Err(RuntimeError(String::from("Hook failed")));
            }
        } else {
            info!("No hook");
        }
        Ok(())
    }

    pub fn change_state(&mut self, state: ReleaseState) -> Result<()> {
        let current = self.project.read_current();
        if current == Some(self.get_release_path()) {
            return Err(RuntimeError(String::from(
                "Cannot change state of current release",
            )));
        }
        let new_path = state.new_path_for(&self.release);
        info!("Renaming {:?} to {:?}", self.release, &new_path);
        rename(
            self.get_release_path(),
            self.project.base_dir.join("releases").join(&new_path),
        )?;
        self.release = new_path;
        self.state = state;
        Ok(())
    }
}
