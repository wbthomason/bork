use clap::ArgMatches;
use super::common::print_status;
use super::constants;

// Remove the listed packages and their dependencies
pub fn remove_packages(opt_packages: Option<Vec<&str>>, cmd_packages: Option<&ArgMatches>) {
    let packages =
        if let Some(x) = opt_packages {x}
        else if let Some(x) = cmd_packages {x.values_of(constants::PACKAGES).unwrap()}
        else {return};
    print_status("Removing", &packages);
}
