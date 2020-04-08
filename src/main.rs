use std::fs::{read_to_string, create_dir_all, remove_file};
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::io;
use std::process::Command;
use std::env::current_dir;

fn main() {
    let deploy = Deployement::new();
    deploy.do_install();
    deploy.do_switch();
}

struct Deployement {
    base_dir : PathBuf,
    release : String,
}

impl Deployement {
    fn new() -> Deployement {
        Deployement {
            base_dir: current_dir().unwrap(),
            release: get_date_str(),
        }
    }

    fn get_release_path(&self) -> PathBuf {
        self.base_dir.join("releases").join(&self.release)
    }

    fn do_install(&self) -> io::Result<()> {
        // TODO put code

        self.do_links()?;
        self.do_hook("install")
    }

    fn do_switch(&self) -> io::Result<()> {
        let current_path = self.base_dir.join("current");
        if current_path.exists() {
            remove_file(&current_path)?;
        }
        symlink(self.get_release_path(), &current_path)?;
        self.do_hook("switch")
    }

    fn do_links(&self) -> io::Result<()> {
        let links = read_to_string(self.base_dir.join("deployer.links")).unwrap_or(String::from(""));
        for line in links.lines() {
            let parts : Vec<&str> = line.split_whitespace().collect();
            match parts.as_slice() {
                [from, to] => {
                    let dest_path = self.get_release_path().join(to);
                    create_dir_all(dest_path.parent().unwrap())?;
                    symlink(self.base_dir.join("shared").join(from), dest_path)?;
                },
                [path] => {
                    let dest_path = self.get_release_path().join(path);
                    create_dir_all(dest_path.parent().unwrap())?;
                    symlink(self.base_dir.join("shared").join(path), dest_path)?;
                }
                [] => {}
                _ => {eprintln!("Bad link line: {}", line)}
            }
        }
        Ok(())
    }

    fn do_hook(&self, hook : &str) -> io::Result<()> {
        let path = self.base_dir.join(format!("deployer.{}", hook));
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

fn get_date_str() -> String {
    let output = Command::new("date")
        .arg("+%Y-%m-%d-%H-%M-%S")
        .output()
        .unwrap();
    String::from(String::from_utf8(output.stdout).unwrap().trim())
}
