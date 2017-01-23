use std::collections::HashSet;
use std::process::Command;
use std::io::Write;

use colored::*;

use chrono::{TimeZone, NaiveDateTime, Local};

use aur;
use ::SearchResult;

// Search the AUR and core repositories for packages mentioning the given terms
pub fn search_packages<'a>(package_names: HashSet<&'a str>) -> (String, String) {
    let pacman_results = get_pacman_results(&package_names);
    let aur_results = get_aur_results(&package_names);
    (pacman_results, aur_results)
}

fn get_pacman_results<'a, 'b>(package_names: &HashSet<&'a str>) -> String {
    // TODO: Use libalpm instead of directly calling pacman
    // TODO: Format pacman results
    let mut pacman_search_command = Command::new("pacman");
    pacman_search_command.arg("-Ss");
    for name in package_names {
        pacman_search_command.arg(name);
    }

    let pacman_output = pacman_search_command.output().expect("Couldn't run pacman!").stdout;
    String::from_utf8(pacman_output).unwrap()
}

fn get_aur_results<'a>(package_names: &HashSet<&'a str>) -> String {
    let results = aur::search(package_names);
    format_results(results)
}

fn format_results(results: Vec<(String, Result<SearchResult, String>)>) -> String {
    let mut total_length = 0;
    let formatted_results: Vec<String> = results.into_iter().map(|result| {
        match result {
            Ok(package_result)  => {
                let mut total_length = 0;
                let formatted_package_results: Vec<String> = package_result.results.iter().map(|package| {
                    let ref name    = package.name;
                    let ref desc    = package.description;
                    let ref url     = package.url;
                    let ref version = package.version;
                    let maintainer  = package.maintainer.clone().map(|m| m.normal()).unwrap_or("Orphaned".red());
                    let outdated    = package.out_of_date.map(|x| if x == 0 { false } else { true }).unwrap_or(false);
                    let modified    = Local.from_utc_datetime(&NaiveDateTime::from_timestamp(package.last_modified, 0));
                    let mut formatted_result = Vec::new();
                    write!(&mut formatted_result, "{}", "[AUR]\t".bold().on_green());
                    write!(
                        &mut formatted_result, 
                        "{} ({}): {}\n\tMaintainer: {}\tURL: {}\n\tModified: {}",
                        name.bold(),
                        version.blue(),
                        desc,
                        maintainer,
                        url.cyan(),
                        modified.to_string());
                    if outdated {
                        write!(&mut formatted_result," {}", "(Out of date)".red());
                    }

                    write!(&mut formatted_result, "\n");

                    let formatted_result = match String::from_utf8(formatted_result) {
                        Ok(r)   => r,
                        Err(e)  => format!("{}", e)
                    };

                    total_length += formatted_result.len();
                    formatted_result
                }).collect();

                let output = formatted_package_results.join("\n");
                total_length += output.len();
                output
            },
            Err(search_error)  => format!("{}", search_error.bold().white().on_red())
        }
    }).collect();
    formatted_results.join("\n")
}
