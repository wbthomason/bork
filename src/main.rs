// Primary module; main control flow
#[macro_use]
extern crate clap;
extern crate toml;
#[macro_use]
extern crate log;
extern crate env_logger;

mod common;
mod config;
mod constants;
mod installation;
mod removal;
mod search;
mod update;
mod util;

fn main() {
    // Start logging
    env_logger::init().expect("Couldn't initialize logging!");
    // Parse arguments
    let args = config::get_args();
    debug!("Got args: {:?}", args);
    // Read configuration and load system info
    let config_opts = config::get_config_opts(args.value_of(constants::CONFIG_FILE));

    // If we only want to search, find the named packages
    if args.is_present(constants::SEARCH) || args.is_present(constants::SEARCH_CMD) {
        let packages_for_search = util::merge_package_vecs(&[
            args.values_of(constants::SEARCH),
            args.values_of(constants::SEARCH_CMD)]);
        let (core_str, aur_str) = search::search_packages(packages_for_search);
        println!("Core Packages:\n{}\nAUR Packages:\n{}", core_str, aur_str);
    }
    else {
        // Remove packages to be removed
        let packages_for_removal = util::merge_package_vecs(&[
            args.values_of(constants::REMOVE),
            args.values_of(constants::REMOVE_CMD)]);
        removal::remove_packages(packages_for_removal);

        // Install packages to be installed
        let packages_for_installation = util::merge_package_vecs(&[
            args.values_of(constants::INSTALL),
            args.values_of(constants::INSTALL_CMD)]);
        installation::install_packages(packages_for_installation);

        // Update packages to be updated
        let packages_for_updating = util::merge_package_vecs(&[
            args.values_of(constants::UPDATE),
            args.values_of(constants::UPDATE_CMD)]);
        update::update_packages(packages_for_updating);
    }
}
