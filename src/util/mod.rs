// A collection of useful utilities for bork

pub fn merge_package_vecs(opt_packages: Option<Vec<&str>>,
                          cmd_packages: Option<&ArgMatches>,
                          key: &str)
                          -> Option<Vec<&str>> {
    let cmd_package_names = cmd_packages.map(|arg_packages| arg_packages.values_of(key).iter());
    cmd_package_names.map_or(
        opt_packages, |package_names| package_names.extend(opt_packages.unwrap_or([])))

}
