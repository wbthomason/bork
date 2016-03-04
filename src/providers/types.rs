// Types for providers
//
use constants;

use std::path::Path;

pub struct Provider<'_> {
    lang: constants::LangTypes,
    os: constants::OSTypes,
    path: Option<&'_ Path>,
    install_path: Option<&'_ Path>,
    dependency_path: Option<&'_ Path>,
    name: &'_ str
}
