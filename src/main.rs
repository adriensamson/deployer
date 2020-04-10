#[macro_use]
extern crate log;
extern crate stderrlog;
#[macro_use]
extern crate structopt;

use structopt::StructOpt;
mod error;
mod release;
mod installation_method;
mod project;

use installation_method::noop::NoopInstallationMethod;
use crate::project::Project;
use crate::error::Result;

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

    let project = Project::from_current_dir().unwrap();

    match opt.cmd.unwrap_or(Cmd::Deploy) {
        Cmd::Deploy => {
            project.deploy(NoopInstallationMethod {})
        }
        Cmd::Rollback => {
            project.rollback()
        }
    }
}
