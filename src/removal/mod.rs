use common::print_status;
use constants;
use std::collections::HashSet;

// Remove the listed packages and their dependencies
pub fn remove_packages<'a>(packages_to_remove: HashSet<&'a str>) -> Vec<i32> {
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
