use std::collections::HashSet;

// Search the AUR and core repositories for packages mentioning the given terms
pub fn search_packages<'a>(package_names: HashSet<&'a str>) -> (&str, &str) {
    ("Foo", "Bar")
}
