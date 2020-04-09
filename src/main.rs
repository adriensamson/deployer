mod error;
mod release;
mod installation_method;
mod project;
mod deployement;

use installation_method::noop::NoopInstallationMethod;
use crate::project::Project;

fn main() {
    let project = Project::from_current_dir().unwrap();
    project.deploy(NoopInstallationMethod {}).unwrap();
    project.rollback().unwrap();
}
