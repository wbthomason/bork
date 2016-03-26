use clap::ArgMatches;
use common::print_status;
use constants;
use providers::Provider;

// Retrieve details on the listed packages
pub fn find_packages(valid_providers: Vec<Provider>,
                     opt_packages: Option<Vec<&str>>,
                     cmd_packages: Option<&ArgMatches>) {
    let packages = if let Some(x) = opt_packages {
        x
    } else if let Some(x) = cmd_packages {
        x.values_of(constants::PACKAGES).unwrap()
    } else {
        return;
    };
    print_status("Searching for", &packages);
}
