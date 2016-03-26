// The main packages module. Re-exports the types, and provides the package
// interface functions

pub mod types;

pub fn load_packages() -> Vec<i32> {
    vec![0, 1]
}
pub fn save_packages(package_list: Vec<i32>) {}
