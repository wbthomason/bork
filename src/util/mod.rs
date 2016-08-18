// A collection of useful utilities for bork

use clap::Values;

use std::collections::HashSet;

// Merge several arg and subcommand results into a single structure
pub fn merge_package_vecs<'a>(package_lists: &[Option<Values<'a>>]) -> HashSet<&'a str> {
    package_lists.iter().fold(
        HashSet::new(),
        |mut acc, &list| {
            let updated_package_set = 
                match list {
                    Some(vals)   => {
                        for val in vals {
                            acc.insert(val);
                        }
                        acc
                    },
                    None         => acc
                };
            updated_package_set
        })
}
