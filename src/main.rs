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

fn main() {
    let args = config::args();
    installation::install_packages(args.values_of(constants::INSTALL), args.subcommand_matches(constants::INSTALL_CMD));
    removal::remove_packages(args.values_of(constants::REMOVE), args.subcommand_matches(constants::REMOVE_CMD));
    update::update_packages(args.values_of(constants::UPDATE), args.subcommand_matches(constants::UPDATE_CMD));
    search::find_packages(args.values_of(constants::SEARCH), args.subcommand_matches(constants::SEARCH_CMD));
    config::config_opts(args.value_of(constants::CONFIG_FILE));
}
