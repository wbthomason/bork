extern crate yaml_rust;

mod types;
pub use self::types::Provider;

use yaml_rust::{YamlLoader, Yaml};

pub fn get_valid_providers<'_>() -> Vec<Provider<'_>> {}
