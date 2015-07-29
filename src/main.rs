#[macro_use]
extern crate clap;
extern crate hyper;
extern crate term;

mod constants;
mod config;

use clap::ArgMatches;

fn main() {
    let args = config::args();
    install_packages(args.values_of(constants::INSTALL), args.subcommand_matches(constants::INSTALL_CMD));
    remove_packages(args.values_of(constants::REMOVE), args.subcommand_matches(constants::REMOVE_CMD));
    update_packages(args.values_of(constants::UPDATE), args.subcommand_matches(constants::UPDATE_CMD));
    find_packages(args.values_of(constants::SEARCH), args.subcommand_matches(constants::SEARCH_CMD));
}

// Install the listed packages and their dependencies
fn install_packages(opt_packages: Option<Vec<&str>>, cmd_packages: Option<&ArgMatches>) {
    let packages =
        if let Some(x) = opt_packages {x}
        else if let Some(x) = cmd_packages {x.values_of(constants::PACKAGES).unwrap()}
        else {return};
    print_status("Installing", &packages);
}

// Remove the listed packages and their dependencies
fn remove_packages(opt_packages: Option<Vec<&str>>, cmd_packages: Option<&ArgMatches>) {
    let packages =
        if let Some(x) = opt_packages {x}
        else if let Some(x) = cmd_packages {x.values_of(constants::PACKAGES).unwrap()}
        else {return};
    print_status("Removing", &packages);
}

// Update the listed packages, or all packages
fn update_packages(opt_packages: Option<Vec<&str>>, cmd_packages: Option<&ArgMatches>) {
    let packages =
        if let Some(x) = opt_packages {x}
        else if let Some(x) = cmd_packages {x.values_of(constants::PACKAGES).unwrap_or(vec![])}
        else {return};
    if packages.len() == 0 || packages[0].to_string() == "all" {
        println!("\tUpdating all packages");
    }
    else {
        print_status("Updating", &packages);
    }
}

// Retrieve details on the listed packages
fn find_packages(opt_packages: Option<Vec<&str>>, cmd_packages: Option<&ArgMatches>) {
    let packages =
        if let Some(x) = opt_packages {x}
        else if let Some(x) = cmd_packages {x.values_of(constants::PACKAGES).unwrap()}
        else {return};
    print_status("Searching for", &packages);
}

fn print_status(operation_type : &str, packages : &Vec<&str>) {
    println!("\n{} {} package{}: {:?}\n",
             operation_type,
             packages.len(),
             if packages.len() == 1 { "" } else { "s" },
             packages );
}
