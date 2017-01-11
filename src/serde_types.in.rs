#[derive(Deserialize)]
struct SearchResult {
    #[serde(rename="type")]
    result_type: String,
    version: i32,
    resultcount: i32,
    results: Vec<self::SearchInfoResult>
}

#[derive(Deserialize)]
struct SearchInfoResult {
    ID: i32,
    Name: String,
    PackageBaseID: i32,
    PackageBase: String,
    Version: String,
    Description: String,
    URL: String,
    NumVotes: i32,
    Popularity: f32,
    OutOfDate: Option<i32>,
    Maintainer: Option<String>,
    FirstSubmitted: i32,
    LastModified: i32,
    URLPath: String,
}

#[derive(Deserialize)]
struct InfoResult {
    #[serde(rename="type")]
    result_type: String,
    version: i32,
    resultcount: i32,
    results: Vec<self::InfoResultItem>
}

#[derive(Deserialize)]
struct InfoResultItem {
    ID: i32,
    Name: String,
    PackageBaseID: i32,
    PackageBase: String,
    Version: String,
    Description: String,
    URL: String,
    NumVotes: i32,
    Popularity: f32,
    OutOfDate: Option<i32>,
    Maintainer: Option<String>,
    FirstSubmitted: i32,
    LastModified: i32,
    URLPath: String,
    Depends: Vec<String>,
    MakeDepends: Vec<String>,
    License: Vec<String>,
    Keywords: Vec<String>
}

#[derive(Deserialize)]
struct ErrorResult {
    #[serde(rename="type")]
    result_type: String,
    version: i32,
    resultcount: i32,
    results: Vec<self::InfoResult>,
    error: String
}
