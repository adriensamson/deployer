mod installation_method;
mod deployement;

use installation_method::noop::NoopInstallationMethod;
use deployement::Deployement;

fn main() {
    let deploy = Deployement::new(NoopInstallationMethod {});
    deploy.do_install();
    deploy.do_switch();
}
