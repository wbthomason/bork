// Primary module; main control flow

#[macro_use]
extern crate clap;
extern crate hyper;
extern crate term;
extern crate yaml_rust;

mod installation;
mod common;
mod config;
mod constants;
mod removal;
mod search;
mod update;
mod providers;
mod packages;

fn main() {
    let args = config::get_args();
    // Not doing anything with the config file just yet; we need functionality for that
    let config_opts = config::get_config_opts( args.value_of(constants::CONFIG_FILE));
    let valid_providers = providers::get_valid_providers(config_opts[constants::PROVIDERS_KEY]);
    let installed_packages = packages::load_packages();
    let installed_packages = installation::install_packages(
        valid_providers,
        installed_packages,
        args.values_of(constants::INSTALL),
        args.subcommand_matches(constants::INSTALL_CMD)
        );
    let installed_packages = removal::remove_packages(
        installed_packages,
        args.values_of(constants::REMOVE),
        args.subcommand_matches(constants::REMOVE_CMD)
        );
    let installed_packages = update::update_packages(
        valid_providers,
        installed_packages,
        args.values_of(constants::UPDATE),
        args.subcommand_matches(constants::UPDATE_CMD)
        );
    packages::save_packages(installed_packages);
    search::find_packages(
        valid_providers,
        args.values_of(constants::SEARCH),
        args.subcommand_matches(constants::SEARCH_CMD)
        );
}
