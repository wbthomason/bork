// Primary module; main control flow

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate clap;

extern crate libc;

#[macro_use]
extern crate log;
extern crate log4rs;

extern crate toml;

mod config;
mod constants;
mod installation;
mod removal;
mod search;
mod update;
mod util;

fn main() {
    // Start logging
    log4rs::init_file(constants::NIX_LOG_CONF_PATH, Default::default()).unwrap();
    // Parse arguments
    let args = config::get_args();
    debug!("Got args: {:?}", args);
    // Read configuration and load system info
    let config_opts = config::get_config_opts(args.value_of(constants::CONFIG_FILE));

    // If we only want to search, find the named packages
    if args.is_present(constants::SEARCH) || args.is_present(constants::SEARCH_CMD) {
        let packages_for_search =
            util::merge_package_vecs(&[args.values_of(constants::SEARCH),
                                       util::get_subcommand_packages(&args,
                                                                     constants::SEARCH_CMD)]);
        info!("Searching for: {:?}", packages_for_search);
        let (core_str, aur_str) = search::search_packages(packages_for_search);
        println!("Core Packages:\n{}\nAUR Packages:\n{}", core_str, aur_str);
    } else {
        // Remove packages to be removed
        let packages_for_removal =
            util::merge_package_vecs(&[args.values_of(constants::REMOVE),
                                       util::get_subcommand_packages(&args,
                                                                     constants::REMOVE_CMD)]);
        info!("Removing: {:?}", packages_for_removal);
        removal::remove_packages(packages_for_removal);

        // Install packages to be installed
        let packages_for_installation =
            util::merge_package_vecs(&[args.values_of(constants::INSTALL),
                                       util::get_subcommand_packages(&args,
                                                                     constants::INSTALL_CMD)]);
        info!("Installing: {:?}", packages_for_installation);
        installation::install_packages(packages_for_installation);

        // Update packages to be updated
        let packages_for_updating =
            util::merge_package_vecs(&[args.values_of(constants::UPDATE),
                                       util::get_subcommand_packages(&args,
                                                                     constants::UPDATE_CMD)]);
        info!("Updating: {:?}", packages_for_updating);
        update::update_packages(packages_for_updating);
    }
}
