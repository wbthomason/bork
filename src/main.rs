#![feature(phase)]
extern crate serialize;

#[phase(plugin)] extern crate docopt_macros;
extern crate docopt;

extern crate collections;

extern crate http;
extern crate url;

extern crate term;

use docopt::{FlagParser};
use http::client::RequestWriter;
use http::method::Get;
use url::Url;
use std::io::{Command, println};
use std::str;
use serialize::json::{Json, Decoder};
use collections::treemap::TreeMap;
use term::color;

mod data_structures;

// TODO: Refactor into several files; general cleanup.

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

static SearchUrlBase : &'static str = "https://aur.archlinux.org/rpc.php";

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

  for package in packages.iter() {
    let package_name = package.clone();
    spawn( proc() {
      let pacman_results = match Command::new("/bin/pacman").args(&[String::from_str("-Ss"), package_name.clone()]).output() {
        Ok(result)  => match String::from_utf8(result.output) {
          Ok(parsed) => parsed,
          Err(err) => format!("Parsing of output failed: {}", err)
          },
          Err(err)    => format!("pacman failed: {}", err)
        };

        print_pacman_results(&pacman_results, &package_name);

        // TODO: Make this safer
        let search_url = Url::parse(format!("{}?type=search&arg={}", SearchUrlBase, package_name).as_slice()).unwrap();
        let aur_request : RequestWriter = match RequestWriter::new(Get, search_url) {
          Ok(request) =>  request,
          // TODO: Make failure not crash everything
          Err(err)    =>  fail!("Couldn't query AUR: {}", err)
        };

        let mut aur_response = match aur_request.read_response() {
          Ok(response) => response,
          Err((_request, error)) => fail!("Querying AUR failed: {}", error),
        };

        let body_json = match aur_response.read_to_end() {
          Ok(body) => body,
          Err(err) => fail!("Reading response failed: {}", err),
        };
        let body_json = str::from_utf8(body_json.as_slice()).expect("Response was not properly encoded UTF-8");
        // TODO: Make this safer
        let body_json : Json = from_str(body_json).unwrap();
        let aur_results = parse_aur_json(&body_json);
        if aur_results.is_none() {
          println!("Too many AUR results for {}; narrow your query", package_name)
        }
        else {
          print_aur_results(aur_results.unwrap().as_slice(), &package_name);
        } })

    }}

    fn print_aur_results(aur_results : &[data_structures::AurItem], package_name : &String) {
      if aur_results.len() > 0 {
        println!("------------------------------------------");
        println!("\tOutput from AUR for {}:", package_name);
        println!("------------------------------------------\n");
        let mut terminal = term::stdout().unwrap();
        for result in aur_results.iter() {
          terminal.fg(color::GREEN).unwrap();
          (write!(terminal, "[")).unwrap();
          if result.Maintainer.is_some() {
            (write!(terminal, "{}/", result.Maintainer.clone().unwrap())).unwrap();
          }

          (write!(terminal, "{}]:\t", result.Name)).unwrap();
          terminal.fg(color::WHITE).unwrap();
          (writeln!(terminal, "{}", result.Description)).unwrap();
        }

        terminal.reset().unwrap();
      }
      else {
        println!("-------------------------------");
        println!("\tNo results for {} from AUR", package_name);
        println!("-------------------------------");
      }
    }

    fn print_pacman_results(pacman_results : &String, package_name : &String) {
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
    }

    fn parse_aur_json(body_json : &Json)  ->  Option<Vec<data_structures::AurItem>> {
      let id_string : String = String::from_str("ID");
      let name_string : String = String::from_str("Name");
      let packageBaseId_string : String = String::from_str("PackageBaseID");
      let packageBase_string : String = String::from_str("PackageBase");
      let version_string : String = String::from_str("Version");
      let categoryId_string : String = String::from_str("CategoryID");
      let description_string : String = String::from_str("Description");
      let url_string : String = String::from_str("URL");
      let numvotes_string : String = String::from_str("NumVotes");
      let outofdate_string : String = String::from_str("OutOfDate");
      let maintainer_string : String = String::from_str("Maintainer");
      let firstsubmitted_string : String = String::from_str("FirstSubmitted");
      let lastmodified_string : String = String::from_str("LastModified");
      let license_string : String = String::from_str("License");
      let urlPath_string : String = String::from_str("URLPath");

      let json_map : &TreeMap<String, Json> = body_json.as_object().unwrap();
      let result_list : Option<&Vec<Json>> = json_map.find(&String::from_str("results")).unwrap().as_list();
      if result_list.is_none() {
        return None
      }

      let result_list = result_list.unwrap();
      let mut parsed_list : Vec<data_structures::AurItem> = vec![];
      for result in result_list.iter() {
        let result = result.as_object().unwrap();
        // TODO: Make all unwraps safer/eliminate
        // TODO: Only retrieve datums used
        let result_item = data_structures::AurItem {
          ID :  result.find(&id_string).unwrap().as_number().unwrap(),
          Name : String::from_str(result.find(&name_string).unwrap().as_string().unwrap()),
          PackageBaseID : result.find(&packageBaseId_string).unwrap().as_number().unwrap(),
          PackageBase : String::from_str(result.find(&packageBase_string).unwrap().as_string().unwrap()),
          Version : String::from_str(result.find(&version_string).unwrap().as_string().unwrap()),
          CategoryID : result.find(&categoryId_string).unwrap().as_number().unwrap(),
          Description : String::from_str(result.find(&description_string).unwrap().as_string().unwrap()),
          URL : String::from_str(result.find(&url_string).unwrap().as_string().unwrap()),
          NumVotes : result.find(&numvotes_string).unwrap().as_number().unwrap(),
          OutOfDate : result.find(&outofdate_string).unwrap().as_number().unwrap(),
          Maintainer : result.find(&maintainer_string).unwrap().as_string().map(|maintainer| String::from_str(maintainer)),
          FirstSubmitted : result.find(&firstsubmitted_string).unwrap().as_number().unwrap(),
          LastModified : result.find(&lastmodified_string).unwrap().as_number().unwrap(),
          License : String::from_str(result.find(&license_string).unwrap().as_string().unwrap()),
          URLPath : String::from_str(result.find(&urlPath_string).unwrap().as_string().unwrap())
        };

        parsed_list.push(result_item);
      }
      Some(parsed_list)
    }

    fn print_status(operation_type : &str, packages : &Vec<String>) {
      println!("\n{} {} package{}: {}\n",
      operation_type,
      packages.len(),
      if packages.len() == 1 { "" } else { "s" },
      packages );
    }
