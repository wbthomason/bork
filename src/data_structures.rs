extern crate serialize;
use serialize::json;

  #[deriving(Decodable)]
  pub struct AurItem {
    pub ID :  f64,
    pub Name : String,
    pub PackageBaseID : f64,
    pub PackageBase : String,
    pub Version : String,
    pub CategoryID : f64,
    pub Description : String,
    pub URL : String,
    pub NumVotes : f64,
    pub OutOfDate : f64,
    pub Maintainer : Option<String>,
    pub FirstSubmitted : f64,
    pub LastModified : f64,
    pub License : String,
    pub URLPath : String
  }
