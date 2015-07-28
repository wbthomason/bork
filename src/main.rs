extern crate rustc_serialize;
extern crate docopt;
extern crate hyper;
extern crate term;

mod data_structures;

use docopt::Docopt;

use data_structures::Args;

static USAGE: &'static str = "
Usage:
    borealis install PACKAGES...
    borealis remove PACKAGES...
    borealis update [PACKAGES]...
    borealis search PACKAGES...
    borealis [(-i IPACKAGES...)] [(-r RPACKAGES...)] [(-u [UPACKAGES]...)] [(-s SPACKAGES...)]
    borealis -h

Commands:
-i, --install IPACKAGES     Install packages.
-r, --remove  RPACKAGES     Remove packages.
-u, --update  [UPACKAGES]   Update packages. Pass \"all\" or nothing to update everything.
-s, --search  SPACKAGES     Search for packages.
-h, --help                  Print this help message.
";

fn main() {
  let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
  println!("{:?}", args);
  if args.cmd_install {
    install_packages(args.arg_PACKAGES)
  }

  else if args.cmd_remove {
    remove_packages(args.arg_PACKAGES)
  }

  else if args.cmd_update {
    update_packages(args.arg_PACKAGES)
  }

  else if args.cmd_search {
    find_packages(args.arg_PACKAGES)
  }

  else if args.flag_install || args.flag_remove || args.flag_search || args.flag_update {
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
  if packages.len() == 0 || packages[0].to_string() == "all" {
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
    }

fn print_status(operation_type : &str, packages : &Vec<String>) {
  println!("\n{} {} package{}: {:?}\n",
  operation_type,
  packages.len(),
  if packages.len() == 1 { "" } else { "s" },
  packages );
}
