extern crate rustc_serialize;

// Because the Docopt API requires certain names
#[allow(non_snake_case)]
#[derive(RustcDecodable, Debug)]
pub struct Args {
    pub arg_PACKAGES : Vec<String>,
    pub cmd_install : bool,
    pub flag_install : bool,
    pub arg_IPACKAGES : Vec<String>,
    pub cmd_remove : bool,
    pub flag_remove : bool,
    pub arg_RPACKAGES : Vec<String>,
    pub cmd_update : bool,
    pub flag_update : bool,
    pub arg_UPACKAGES : Vec<String>,
    pub cmd_search : bool,
    pub flag_search : bool,
    pub arg_SPACKAGES : Vec<String>,
    pub flag_help : bool,
}
