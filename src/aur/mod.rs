use curl::easy::Easy;

use itertools::Itertools;

use futures::{Future, Then, BoxFuture};
use futures::future::{err, join_all};

use serde_json;
use serde::Deserialize;

use tokio_core::reactor::Core;
use tokio_curl::Session;

use std::collections::HashSet;
use std::io::{self, Write};
use std::error::Error;
use std::sync::{Mutex, Arc};

use constants;

pub mod types;
use self::types::{SearchResult, InfoResult};

pub fn search(packages: &HashSet<&str>) -> Vec<(String, Result<SearchResult, String>)> {
    let url = constants::AUR_RPC_URL.to_owned() + constants::AUR_RPC_SEARCH_FMT +
        constants::AUR_RPC_SEARCH_ARG;
    make_requests(&url, packages)
}

// TODO: To reuse this for info queries, I need to change the return signature
fn make_requests<T: Deserialize + Send>(
    url: &str,
    packages: &HashSet<&str>,
) -> Vec<(String, Result<T, String>)> {
    let mut lp = Core::new().unwrap();
    let session = Session::new(lp.handle());
    // TODO: Think about a way to not make the failure of one future make all the rest fail. This
    // will involve not using join_all(), but at this time there doesn't seem to be a great
    // alternative in futures-rs
    let package_results = lp.run(join_all(packages.iter().map(move |package_name| {
        // I think we need this to be owned because of the error messages. Also, it's weird that we
        // get a &&str here - it makes sense, but implies that there might be a cleaner/more
        // idiomatic way to do things
        // TODO: Find a better way to do error messages without requiring creating an owned copy
        let package = String::from(*package_name);
        let mut search_req = Easy::new();
        let search_data = Arc::new(Mutex::new(Vec::new()));
        let write_handle = search_data.clone();
        match search_req.url(&format!("{}{}", url, package)) {
            Ok(_)   => {
                search_req.write_function(move |data| {
                    write_handle.lock().unwrap().extend_from_slice(data);
                    Ok(data.len())
                }).unwrap();
                // TODO: This is bad and there is presumably a better way to do it
                let p = package.clone();
                session.perform(search_req).then(move |result| {
                    result.map(|_|
                               serde_json::from_slice(
                                   &search_data.lock().unwrap()
                                )
                               .map_err(|e| format!("{}: {:?}", p, e)))
                        // I don't really like losing the error like this.
                        // TODO: Unify the error types here in a better way
                }).map_err(move |e| format!("{}: {:?}", package, e)).boxed()
            },
            Err(e)  => err(format!("{}: {:?}", package, e)).boxed()
        }}).collect::<Vec<_>>())).unwrap().into_iter();
    packages
        .iter()
        .map(|s| String::from(*s))
        .zip(package_results)
        .collect::<Vec<_>>()
}
