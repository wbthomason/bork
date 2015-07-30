// Config: Manage parsing of command line options and the config file

use clap::{App, Arg, SubCommand, ArgMatches};
use std::fs::File;
use std::io::prelude::*;
use std::process;
use yaml_rust::{YamlLoader, Yaml};

use super::constants;

pub fn args<'_>() -> ArgMatches<'_, '_>  {
    App::new("borealis")
        .version(&crate_version!()[..])
        .about("\nManages package managers, works universally, dances flamenco.
You can use the subcommands for operations of only one type,
or compose the options and flags to trigger operations of
multiple types. Operations are run in the order Install, Remove, Update, Search")
        .author("Wil Thomason <wil.thomason@gmail.com>")
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
                                  Defaults to /etc/borealis/config.yaml on *nix systems,
                                  and C:\\borealis\\config.yaml on Windows"))
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
                .help("The list of packages to update. Pass \"all\" or nothing to update the system.")
                .multiple(true)
                .index(1)
                ))
        .subcommand(SubCommand::with_name(constants::SEARCH_CMD)
            .about("Search for packages")
            .arg(Arg::with_name(constants::PACKAGES)
                .help("The list of packages for which to search")
                .multiple(true)
                .index(1)
                .required(true))).get_matches()
}

pub fn config_opts(conf_path: Option<&str>) -> Vec<Yaml> {
    let file_path = conf_path.unwrap_or(
        if cfg!(target_os = "windows") { constants::WINDOWS_CONF_PATH }
        else { constants::NIX_CONF_PATH }
        );
    let config_file = try!(File::open(&file_path));
    let mut config_contents = String::new();
    try!(config_file.read_to_string(&mut config_contents));
    let options = if let Some(opts) = YamlLoader::load_from_str(config_contents.as_str())
        { opts }
    else {
        println!("Error: Invalid configuration file at {}", file_path);
        process::exit(1)
    };
    println!("{:?}", options);
    options
}
