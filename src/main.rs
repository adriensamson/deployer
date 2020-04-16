#[macro_use]
extern crate log;
extern crate serde;
extern crate stderrlog;
extern crate structopt;
extern crate toml;

use structopt::StructOpt;
mod error;
mod installation_method;
mod project;
mod release;

use crate::error::Result;
use crate::installation_method::InstallationMethodConfig;
use crate::project::{Project, ProjectConfig};
use std::env::current_dir;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
    /// Timestamp (sec, ms, ns, none)
    #[structopt(long = "timestamp")]
    ts: Option<stderrlog::Timestamp>,
    #[structopt(short = "C", long)]
    change_dir: Option<String>,

    #[structopt(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(StructOpt, Debug)]
enum Cmd {
    Deploy,
    Rollback,
    InitNoop,
    InitGit,
    InitTar,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    stderrlog::new()
        .module(module_path!())
        .quiet(opt.quiet)
        .verbosity(opt.verbose)
        .timestamp(opt.ts.unwrap_or(stderrlog::Timestamp::Off))
        .init()
        .unwrap();

    let mut base_dir = current_dir()?;
    if let Some(dir) = opt.change_dir {
        base_dir = base_dir.join(dir);
    }

    match opt.cmd.unwrap_or(Cmd::Deploy) {
        Cmd::InitNoop => {
            let config = ProjectConfig {
                installation_method: InstallationMethodConfig::Noop,
            };
            Project::init(base_dir, &config)?;
            return Ok(());
        }
        Cmd::InitGit => {
            let config = ProjectConfig {
                installation_method: InstallationMethodConfig::Git {
                    source_dir: String::from("sources"),
                    branch: String::from("master"),
                },
            };
            Project::init(base_dir, &config)?;
            return Ok(());
        }
        Cmd::InitTar => {
            let config = ProjectConfig {
                installation_method: InstallationMethodConfig::Tar {
                    filename: String::from("archive.tar.gz"),
                },
            };
            Project::init(base_dir, &config)?;
            return Ok(());
        }
        Cmd::Deploy => {
            let project = Project::from_dir(base_dir)?;
            project.deploy()
        }
        Cmd::Rollback => {
            let project = Project::from_dir(base_dir)?;
            project.rollback()
        }
    }
}
