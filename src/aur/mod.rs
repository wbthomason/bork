use curl::easy::Easy;

use itertools::Itertools;

use futures::{Future, Then};
use futures::future::{err, join_all};

use serde_json;

use tokio_core::reactor::Core;
use tokio_curl::Session;

use std::collections::HashSet;
use std::io::{self, Write};
use std::error::Error;

use constants;
use ::SearchResult;

pub fn search(packages: &HashSet<&str>) -> Vec<String> {
    let mut results = Vec::new();
    let mut rpc = Easy::new();
    let url = constants::AUR_RPC_URL.to_owned() + constants::AUR_RPC_SEARCH_FMT + constants::AUR_RPC_SEARCH_ARG;
    let raw_results = make_requests(&url, &packages);
}

fn make_requests(url: &str, packages: &HashSet<&str>) -> Vec<Result<SearchResult, String>> {
    // TODO: Figure out a better way to do this, i.e. not requiring that the error be passed as a
    // string.
    let make_error = |package, e| format!("Error searching for \"{}\": {}", package, e);
    let mut lp = Core::new().unwrap();
    let session = Session::new(lp.handle());
    let requests = packages.iter().map(move |package| {
        // I think we need this to be owned because of the error messages. Also, it's weird that we
        // get a &&str here - it makes sense, but implies that there might be a cleaner/more
        // idiomatic way to do things
        // TODO: Find a better way to do error messages without requiring creating an owned copy
        let package = String::from(*package);
        let mut search_req = Easy::new();
        let mut search_data = Vec::new();
        match search_req.url(&format!("{}{}", url, package)) {
            Ok(_)   => {
                search_req.write_function(|data| {
                    io::stdout().write_all(data.clone()).unwrap();
                    search_data.extend_from_slice(data);
                    Ok(search_data.len())
                }).unwrap();
                session.perform(search_req).then(|result| {
                    result.map(|_| serde_json::from_slice(&search_data).map_err(|e| make_error(package, e.description())))
                        // I don't really like losing the error like this.
                        // TODO: Unify the error types here in a better way
                }).map_err(|e| make_error(package, e.description())).boxed()
            },
            Err(e)  => err(make_error(package, e.description())).boxed()
        }})
    .collect();
    // TODO: Figure out a way to not make the failure of one future make all the rest fail. This
    // will involve not using join(), but at this time there doesn't seem to be a great alternative
    // in futures-rs
    lp.run(join_all(requests)).unwrap()
}
