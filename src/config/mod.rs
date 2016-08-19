// Config: Manage parsing of command line options and the config file

use clap::{App, Arg, SubCommand, ArgMatches};
use toml::{Table, Parser};
use std::fs::File;
use std::io::prelude::*;

use constants;

pub fn get_args<'_>() -> ArgMatches<'_> {
    App::new("bork")
        .version(crate_version!())
        .about("A hopefully helpful AUR helper.
You can use the subcommands for operations of only one type,
or compose the options and flags to trigger operations of
multiple types. Operations are run in the order: [Search] xor [Remove, Install, Update]")
        .author("Wil Thomason <wbthomason@cs.cornell.edu>")
        .arg(Arg::with_name(constants::INSTALL)
            .short(constants::SHORT_INSTALL)
            .long(constants::INSTALL)
            .takes_value(true)
            .multiple(true)
            .help("Install packages"))
        .arg(Arg::with_name(constants::REMOVE)
            .short(constants::SHORT_REMOVE)
            .long(constants::REMOVE)
            .takes_value(true)
            .multiple(true)
            .help("Remove packages"))
        .arg(Arg::with_name(constants::UPDATE)
            .short(constants::SHORT_UPDATE)
            .long(constants::UPDATE)
            .takes_value(true)
            .multiple(true)
            .help("Update specific packages."))
        .arg(Arg::with_name(constants::UPDATE_ALL)
            .short(constants::SHORT_UPDATE_ALL)
            .long(constants::UPDATE_ALL)
            .help("Update all packages."))
        .arg(Arg::with_name(constants::SEARCH)
            .short(constants::SHORT_SEARCH)
            .long(constants::SEARCH)
            .takes_value(true)
            .multiple(true)
            .help("Search for packages"))
        .arg(Arg::with_name(constants::CONFIG_FILE)
            .short(constants::SHORT_CONFIG_FILE)
            .long(constants::CONFIG_FILE)
            .takes_value(true)
            .help("Specify the path to an alternate configuration file.
                                  \
                   Defaults to /etc/bork/config.toml."))
        .subcommand(SubCommand::with_name(constants::INSTALL_CMD)
            .about("Install packages")
            .arg(Arg::with_name(constants::PACKAGES)
                .help("The list of packages to install")
                .multiple(true)
                .index(1)
                .required(true)))
        .subcommand(SubCommand::with_name(constants::REMOVE_CMD)
            .about("Remove packages")
            .arg(Arg::with_name(constants::PACKAGES)
                .help("The list of packages to remove")
                .multiple(true)
                .index(1)
                .required(true)))
        .subcommand(SubCommand::with_name(constants::UPDATE_CMD)
            .about("Update packages or the system")
            .arg(Arg::with_name(constants::PACKAGES)
                .help("The list of packages to update. Pass \"all\" or nothing to update the \
                       system.")
                .multiple(true)
                .index(1)))
        .subcommand(SubCommand::with_name(constants::SEARCH_CMD)
            .about("Search for packages")
            .arg(Arg::with_name(constants::PACKAGES)
                .help("The list of packages for which to search")
                .multiple(true)
                .index(1)
                .required(true)))
        .get_matches()
}

pub fn get_config_opts(file_path: Option<&str>) -> Table {
    let file_path = file_path.unwrap_or(constants::NIX_CONF_PATH);
    let mut config_file = File::open(&file_path)
        .ok()
        .expect(&format!("Couldn't open {}", &file_path));
    let mut config_contents = String::new();
    config_file.read_to_string(&mut config_contents)
        .ok()
        .expect(&format!("Couldn't read {}", &file_path));
    let options = Parser::new(&config_contents)
        .parse()
        .expect(&format!("Invalid config file at {}", file_path));
    options
}
