use std::path::PathBuf;
use std::fs::{remove_file, read_to_string, create_dir_all};
use std::process::Command;
use std::os::unix::fs::symlink;
use crate::project::Project;
use crate::error::Result;
use std::ffi::OsString;

pub struct Release<'a> {
    project : &'a Project,
    release : OsString,
}

impl<'a> Release<'a> {
    pub(crate) fn new(project: &'a Project, release: OsString) -> Release<'a> {
        let r = Release {
            project,
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
        let current_path = self.project.base_dir.join("current");
        if current_path.exists() {
            remove_file(&current_path)?;
        }
        symlink(self.get_release_path(), &current_path)?;
        self.do_hook("switch")
    }

    pub fn do_links(&self) -> Result<()> {
        let links = read_to_string(self.project.base_dir.join("deployer.links")).unwrap_or(String::from(""));
        for line in links.lines() {
            let parts : Vec<&str> = line.split_whitespace().collect();
            match parts.as_slice() {
                [from, to] => {
                    let dest_path = self.get_release_path().join(to);
                    create_dir_all(dest_path.parent().unwrap())?;
                    symlink(self.project.base_dir.join("shared").join(from), dest_path)?;
                },
                [path] => {
                    let dest_path = self.get_release_path().join(path);
                    create_dir_all(dest_path.parent().unwrap())?;
                    symlink(self.project.base_dir.join("shared").join(path), dest_path)?;
                }
                [] => {}
                _ => {eprintln!("Bad link line: {}", line)}
            }
        }
        Ok(())
    }

    pub fn do_hook(&self, hook : &str) -> Result<()> {
        let path = self.project.base_dir.join(format!("deployer.{}", hook));
        if path.exists() {
            Command::new(path)
                .current_dir(self.get_release_path())
                .status()
                .unwrap();
            // FIXME : check status code
        }
        Ok(())
    }
}
