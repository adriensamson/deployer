#[macro_use]
extern crate log;
extern crate stderrlog;
#[macro_use]
extern crate structopt;
extern crate toml;
extern crate serde;

use structopt::StructOpt;
mod error;
mod release;
mod installation_method;
mod project;

use crate::project::{Project, ProjectConfig};
use crate::error::Result;
use crate::installation_method::InstallationMethodConfig;

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

    #[structopt(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(StructOpt, Debug)]
enum Cmd {
    Deploy,
    Rollback,
    InitNoop,
    InitGit,
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

    match opt.cmd.unwrap_or(Cmd::Deploy) {
        Cmd::InitNoop => {
            let config = ProjectConfig {
                installation_method: InstallationMethodConfig::Noop,
            };
            Project::init(&config)?;
            return Ok(());
        },
        Cmd::InitGit => {
            let config = ProjectConfig {
                installation_method: InstallationMethodConfig::Git {
                    source_dir: String::from("sources"),
                    branch: String::from("master")
                },
            };
            Project::init(&config)?;
            return Ok(());
        },
        Cmd::Deploy => {
            let project = Project::from_current_dir()?;
            project.deploy()
        }
        Cmd::Rollback => {
            let project = Project::from_current_dir()?;
            project.rollback()
        }
    }
}
