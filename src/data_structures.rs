extern crate serialize;
use serialize::json;

  #[deriving(Decodable)]
  pub struct AurItem {
    pub Name : String,
    pub Version : String,
    pub Description : String,
    pub Maintainer : Option<String>,
    pub License : String,
  }
