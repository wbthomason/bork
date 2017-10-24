// Primary module; main control flow

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

extern crate config;

extern crate curl;

extern crate tokio_core;
extern crate tokio_curl;

extern crate futures;

extern crate colored;

extern crate chrono;

#[macro_use]
extern crate itertools;

//include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

mod config;
mod constants;
mod installation;
mod removal;
mod search;
mod update;
mod util;
mod aur;

fn main() {
    let matches = clap_app!(bork =>
                            (version: crate_version!())
                            (author: "Wil Thomason <wbthomason@cs.cornell.edu>")
                            (about: "Another AUR wrapper, because we needed one of those")
                            (@arg SYNC: -S --sync "Skip all server checks")
                            (@subcommand run =>
                             (about: "Run provided code for the given assignment problem")
                             (@arg ASSIGNMENT: +required "The assignment containing the problem")
                             (@arg PROBLEM: +required "The problem to run")
                             (@arg TIMEOUT: -t --timeout +takes_value "How long to run for (defaults to 60 seconds)")
                            )
                            (@subcommand build =>
                             (about: "Build the given assignment") (@arg ASSIGNMENT: +required "The assignment to build")
                            )
                            (@subcommand get =>
                             (about: "Download the given assignment")
                             (@arg ASSIGNMENT: +required "The assignment to download")
                            )
                            (@subcommand update =>
                             (about: "Update the simulator tool")
                            )
                            (@subcommand install =>
                             (about: "Install V-REP")
                            )
                            ).get_matches();
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


    // Logging setup
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!());

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
        println!("{}\n{}", core_str, aur_str);
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
