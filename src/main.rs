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
        (about: "Another AUR wrapper, because we needed one of those.")
        (@arg INSTALL: -S --install +takes_value +multiple "Install packages")
        (@arg SEARCH: -Ss --search +takes_value +multiple "Search for packages")
        (@arg UPGRADE: -Syu --upgrade +takes_value +multiple "Upgrade packages")
        (@arg CONFIG_FILE: --config +takes_value "Specify the config file path")
        (@subcommand install =>
         (about: "Install packages")
         (@arg PACKAGES: +takes_value +required +multiple "The list of packages to install")
        )
        (@subcommand remove =>
         (about: "Remove packages")
         (@arg PACKAGES: +takes_value +required +multiple "The list of packages to remove")
        )
        (@subcommand upgrade =>
         (about: "Upgrade packages")
         (@arg PACKAGES: +takes_value +required +multiple "The list of packages to upgrade")
        )
        (@subcommand search =>
         (about: "Search for packages")
         (@arg PACKAGES: +takes_value +required +multiple "The list of packages to search for")
        )
        ).get_matches();

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
        let packages_for_search = util::merge_package_vecs(
            &[
                args.values_of(constants::SEARCH),
                util::get_subcommand_packages(&args, constants::SEARCH_CMD),
            ],
        );
        info!("Searching for: {:?}", packages_for_search);
        let (core_str, aur_str) = search::search_packages(packages_for_search);
        println!("{}\n{}", core_str, aur_str);
    } else {
        // Remove packages to be removed
        let packages_for_removal = util::merge_package_vecs(
            &[
                args.values_of(constants::REMOVE),
                util::get_subcommand_packages(&args, constants::REMOVE_CMD),
            ],
        );
        info!("Removing: {:?}", packages_for_removal);
        removal::remove_packages(packages_for_removal);

        // Install packages to be installed
        let packages_for_installation =
            util::merge_package_vecs(
                &[
                    args.values_of(constants::INSTALL),
                    util::get_subcommand_packages(&args, constants::INSTALL_CMD),
                ],
            );
        info!("Installing: {:?}", packages_for_installation);
        installation::install_packages(packages_for_installation);

        // Update packages to be updated
        let packages_for_updating = util::merge_package_vecs(
            &[
                args.values_of(constants::UPDATE),
                util::get_subcommand_packages(&args, constants::UPDATE_CMD),
            ],
        );
        info!("Updating: {:?}", packages_for_updating);
        update::update_packages(packages_for_updating);
    }
}
