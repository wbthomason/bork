#[macro_use]
extern crate clap;
extern crate hyper;
extern crate term;

mod constants;

use clap::{App, Arg, SubCommand};

fn main() {
  let args = App::new("borealis")
                .version(&crate_version!()[..])
                .about("\nManages package managers, works universally, dances flamenco.
You can use the subcommands for operations of only one type,
or compose the options and flags to trigger operations of
multiple types.")
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
                        .required(true)))
                .get_matches();
  install_packages(args.values_of(constants::INSTALL));
  remove_packages(args.values_of(constants::REMOVE));
  update_packages(args.is_present(constants::UPDATE), args.values_of(constants::UPDATE));
  find_packages(args.values_of(constants::SEARCH));
}

// Install the listed packages and their dependencies
fn install_packages(maybe_packages : Option<Vec<&str>>) {
  if let Some(packages) = maybe_packages {
    print_status("Installing", &packages);
  }
}

// Remove the listed packages and (if selected) their dependencies
fn remove_packages(maybe_packages : Option<Vec<&str>>) {
  if let Some(packages) = maybe_packages {
    print_status("Removing", &packages);
  }
}

// Update the listed packages, or all packages
fn update_packages(update_present: bool, maybe_packages : Option<Vec<&str>>) {
  if update_present {
    let packages = maybe_packages.unwrap_or(vec!["all"]);
    if packages.len() == 0 || packages[0].to_string() == "all" {
      println!("\tUpdating all packages");
    }
    else {
      print_status("Updating", &packages);
    }
  }
}

// Retrieve details on the listed packages
fn find_packages(maybe_packages : Option<Vec<&str>>) {
  if let Some(packages) = maybe_packages {
    print_status("Searching for", &packages);
  // Retrieve and parse package data asynchronously, using a lock for printing to
  // prevent intermingling of data
    }
}

fn print_status(operation_type : &str, packages : &Vec<&str>) {
  println!("\n{} {} package{}: {:?}\n",
  operation_type,
  packages.len(),
  if packages.len() == 1 { "" } else { "s" },
  packages );
}
