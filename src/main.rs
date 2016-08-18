// Primary module; main control flow
#[macro_use]
extern crate clap;
extern crate toml;

mod common;
mod config;
mod constants;
mod installation;
mod removal;
mod search;
mod update;
mod util;

fn main() {
    // Parse arguments
    let args = config::get_args();
    // Read configuration and load system info
    let config_opts = config::get_config_opts(args.value_of(constants::CONFIG_FILE));

    // If we only want to search, find the named packages
    if args.is_present(constants::SEARCH) || args.is_present(constants::SEARCH_CMD) {
        let search_results = search::find_packages(
            args.values_of(constants::SEARCH),
            args.subcommand_matches(constants::SEARCH_CMD));
        println!("Search Results: \n{}", search_results);
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
