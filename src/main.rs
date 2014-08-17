#![feature(phase)]
extern crate serialize;
#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;

use docopt::{FlagParser};
use std::sync::atomic::{AtomicUint, SeqCst};
use std::sync::Arc;
use std::task::deschedule;
use std::io::{Command, TcpStream};

// This expects a string literal, so we can't save the string for reuse without being redundant.
docopt!(Args, "
Usage:
borealis (-i IPACKAGES IPACKAGES... | -r RPACKAGES... | -u [UPACKAGES...] | -s SPACKAGES...)...
borealis -h

Options:
-i, --install   Install the listed packages.
-r, --remove    Remove the listed packages.
-u, --update    Update the listed packages.
-s, --search    Search for the listed packages.
-h, --help      Print this help message.
",
flag_search   : bool,
flag_update   : bool,
flag_remove   : bool,
flag_install  : bool)

fn main() {
  let args: Args = FlagParser::parse().unwrap_or_else(|e| e.exit());
  // Note: Given that docopt seems to not allow for repeated arguments in option
  // descriptions, we've had to opt for this structure of having a flag and paired
  // list of arguments.
  if args.flag_install {
    install_packages(args.arg_IPACKAGES)
  }

  if args.flag_remove {
    remove_packages(args.arg_RPACKAGES)
  }

  if args.flag_update {
    update_packages(args.arg_UPACKAGES)
  }

  if args.flag_search {
    find_packages(args.arg_SPACKAGES)
  }
}

// Print help and usage message

// Install the listed packages and their dependencies
fn install_packages(packages : Vec<String>) {
  print_status("Installing", &packages);
}

// Remove the listed packages and (if selected) their dependencies
fn remove_packages(packages : Vec<String>) {
  print_status("Removing", &packages);
}

// Update the listed packages, or all packages
fn update_packages(packages : Vec<String>) {
  if packages.len() == 0 || packages[0].eq(&String::from_str("all")) {
    println!("\tUpdating all packages");
  }
  else {
    print_status("Updating", &packages);
  }
}

// Retrieve details on the listed packages
fn find_packages(packages : Vec<String>) {
  print_status("Searching for", &packages);
  // Retrieve and parse package data asynchronously, using a lock for printing to
  // prevent intermingling of data
  let lock = Arc::new(AtomicUint::new(1));

  for package in packages.iter() {
    let print_lock = lock.clone();
    let package_name = package.clone();
    spawn( proc() {
      let pacman_results = match Command::new("/bin/pacman").args(&[String::from_str("-Ss"), package_name.clone()]).output() {
          Ok(result)  => match String::from_utf8(result.output) {
                          Ok(parsed) => parsed,
                          Err(err) => format!("Parsing of output failed: {}", err)
                          },
          Err(err)    => format!("pacman failed: {}", err)
      };

      if pacman_results.len() > 0 {
        println!("------------------------------------------");
        println!("\tOutput from pacman for {}:", package_name);
        println!("------------------------------------------\n");
        println!("{}", pacman_results);
      }
      else {
        println!("-------------------------------");
        println!("\tNo results for {} from pacman", package_name);
        println!("-------------------------------");
      }

      } )
  }
}

fn print_status(operation_type : &str, packages : &Vec<String>) {
  println!("\n{} {} package{}: {}\n",
          operation_type,
          packages.len(),
          if packages.len() == 1 { "" } else { "s" },
          packages );
}
