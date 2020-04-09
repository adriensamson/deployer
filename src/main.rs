mod release;
mod installation_method;
mod project;
mod deployement;

use installation_method::noop::NoopInstallationMethod;
use deployement::Deployement;
use crate::project::Project;

fn main() {
    let project = Project::from_current_dir();
    project.deploy(NoopInstallationMethod {});
    project.rollback();
}
