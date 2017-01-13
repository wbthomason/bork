#[derive(Deserialize, Debug)]
pub struct SearchResult {
    #[serde(rename="type")]
    result_type: String,
    version: i32,
    resultcount: i32,
    results: Vec<self::SearchInfoResult>
}

#[derive(Deserialize, Debug)]
struct SearchInfoResult {
    #[serde(rename="ID")]
    id: i32,
    #[serde(rename="Name")]
    name: String,
    #[serde(rename="PackageBaseID")]
    package_base_id: i32,
    #[serde(rename="PackageBase")]
    package_base: String,
    #[serde(rename="Version")]
    version: String,
    #[serde(rename="Description")]
    description: String,
    #[serde(rename="URL")]
    url: String,
    #[serde(rename="NumVotes")]
    num_votes: i32,
    #[serde(rename="Popularity")]
    popularity: f32,
    #[serde(rename="OutOfDate")]
    out_of_date: Option<i32>,
    #[serde(rename="Maintainer")]
    maintainer: Option<String>,
    #[serde(rename="FirstSubmitted")]
    first_submitted: i64,
    #[serde(rename="LastModified")]
    last_modified: i64,
    #[serde(rename="URLPath")]
    url_path: String,
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
    #[serde(rename="ID")]
    id: i32,
    #[serde(rename="Name")]
    name: String,
    #[serde(rename="PackageBaseID")]
    package_base_id: i32,
    #[serde(rename="PackageBase")]
    package_base: String,
    #[serde(rename="Version")]
    version: String,
    #[serde(rename="Description")]
    description: String,
    #[serde(rename="URL")]
    url: String,
    #[serde(rename="NumVotes")]
    num_votes: i32,
    #[serde(rename="Popularity")]
    popularity: f32,
    #[serde(rename="OutOfDate")]
    out_of_date: Option<i32>,
    #[serde(rename="Maintainer")]
    maintainer: Option<String>,
    #[serde(rename="FirstSubmitted")]
    first_submitted: i64,
    #[serde(rename="LastModified")]
    last_modified: i64,
    #[serde(rename="URLPath")]
    url_path: String,
    #[serde(rename="Depends")]
    depends: Vec<String>,
    #[serde(rename="MakeDepends")]
    make_depends: Vec<String>,
    #[serde(rename="License")]
    license: Vec<String>,
    #[serde(rename="Keywords")]
    keywords: Vec<String>
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
