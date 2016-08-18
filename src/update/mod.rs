use common::print_status;
use std::collections::HashSet;
use constants;

// Update the listed packages, or all packages
pub fn update_packages<'a>(packages_to_update: HashSet<&'a str>) -> Vec<i32> {
    let packages = if let Some(x) = opt_packages {
        x
    } else if let Some(x) = cmd_packages {
        x.values_of(constants::PACKAGES).unwrap_or(vec![])
    } else {
        return vec![0];
    };
    if packages.len() == 0 || packages[0].to_string() == "all" {
        println!("\tUpdating all packages");
    } else {
        print_status("Updating", &packages);
    }
    vec![0]
}
