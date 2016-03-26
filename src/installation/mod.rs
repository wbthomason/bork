use clap::ArgMatches;
use common::print_status;
use constants;
use providers::Provider;

// Install the listed packages and their dependencies
pub fn install_packages(valid_providers: Vec<Provider>,
                        installed_packages: Vec<i32>,
                        opt_packages: Option<Vec<&str>>,
                        cmd_packages: Option<&ArgMatches>)
                        -> Vec<i32> {
    let packages = if let Some(x) = opt_packages {
        x
    } else if let Some(x) = cmd_packages {
        x.values_of(constants::PACKAGES).unwrap()
    } else {
        return vec![0];
    };
    print_status("Installing", &packages);
    vec![0]
}
