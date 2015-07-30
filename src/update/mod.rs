use clap::ArgMatches;
use super::common::print_status;
use super::constants;

// Update the listed packages, or all packages
pub fn update_packages(opt_packages: Option<Vec<&str>>, cmd_packages: Option<&ArgMatches>) {
    let packages =
        if let Some(x) = opt_packages {x}
        else if let Some(x) = cmd_packages {x.values_of(constants::PACKAGES).unwrap_or(vec![])}
        else {return};
    if packages.len() == 0 || packages[0].to_string() == "all" {
        println!("\tUpdating all packages");
    }
    else {
        print_status("Updating", &packages);
    }
}

