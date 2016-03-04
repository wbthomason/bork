use clap::ArgMatches;
use common::print_status;
use constants;
use providers::Provider;

// Update the listed packages, or all packages
pub fn update_packages(
    valid_providers: Vec<Provider>,
    installed_packages: Vec<i32>,
    opt_packages: Option<Vec<&str>>,
    cmd_packages: Option<&ArgMatches>
    ) -> Vec<i32> {
    let packages =
        if let Some(x) = opt_packages {x}
        else if let Some(x) = cmd_packages {x.values_of(constants::PACKAGES).unwrap_or(vec![])}
        else {return vec![0]};
    if packages.len() == 0 || packages[0].to_string() == "all" {
        println!("\tUpdating all packages");
    }
    else {
        print_status("Updating", &packages);
    }
    vec![0]
}

