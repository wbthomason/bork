extern crate clap;

use clap::ArgMatches;
use common::print_status;
use constants;

// Remove the listed packages and their dependencies
pub fn remove_packages(installed_packages: Vec<i32>,
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
    print_status("Removing", &packages);
    vec![0]
}
