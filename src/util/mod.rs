// A collection of useful utilities for bork
use clap::{Values, ArgMatches};

use std::collections::HashSet;

use constants;

// Merge several arg and subcommand results into a single structure
pub fn merge_package_vecs<'a>(package_lists: &[Option<Values<'a>>]) -> HashSet<&'a str> {
    package_lists.iter().fold(HashSet::new(), |mut acc, list| {
        let updated_package_set = match list {
            &Some(ref vals) => {
                for val in vals.clone() {
                    acc.insert(val);
                }
                acc
            }
            &None => acc,
        };
        updated_package_set
    })
}

pub fn get_subcommand_packages<'a>(
    matches: &'a ArgMatches<'a>,
    subcommand_name: &'static str,
) -> Option<Values<'a>> {
    if let Some(subcommand_packages) = matches.subcommand_matches(subcommand_name) {
        subcommand_packages.values_of(constants::PACKAGES)
    } else {
        None
    }

}
