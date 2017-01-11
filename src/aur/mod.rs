use curl::easy::Easy;

use futures::Future;

use tokio_core::reactor::Core;
use tokio_curl::Session;

use std::collections::HashSet;
use std::io::{self, Write};

use constants;
use ::SearchResult;
use serde_json;

pub fn search(packages: &HashSet<&str>) -> Vec<String> {
    let mut results = Vec::new();
    let mut rpc = Easy::new();
    let url = constants::AUR_RPC_URL.to_owned() + constants::AUR_RPC_SEARCH_FMT + constants::AUR_RPC_SEARCH_ARG;
    let raw_results = make_requests(&url, &packages);
}

fn make_requests(url: &str, packages: &HashSet<&str>) -> Vec<Result<SearchResult, String>> {
    let mut lp = Core::new().unwrap();
    let session = Session::new(lp.handle());
    let futures = packages.into_iter().map(|package| {
        let mut search_req = Easy::new();
        let mut results = Vec::new();
        match search_req.url(&format!("{}{}", url, package)) {
            Ok(_)   => {
                search_req.write_function(|data| {
                    io::stdout().write_all(data.clone()).unwrap();
                    results.extend_from_slice(data);
                    Ok(results.len())
                }).unwrap();
                Ok(session.perform(search_req).then(|result| {
                    result.map(|_| serde_json::from_slice(&results))
                }))
            },
            Err(e)  => Err(format!("Error searching for \"{}\": {:?}", package, e))
        }
    });

    Vec::new()
}
