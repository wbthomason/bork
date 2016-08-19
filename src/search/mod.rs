use std::collections::HashSet;
use std::process::Command;

// Search the AUR and core repositories for packages mentioning the given terms
pub fn search_packages<'a>(package_names: HashSet<&'a str>) -> (String, &str) {
    let pacman_results_ugly = get_pacman_results(&package_names);
    let aur_results_ugly = get_aur_results(&package_names);
    (pacman_results_ugly, aur_results_ugly)
}

fn get_pacman_results<'a, 'b>(package_names: &HashSet<&'a str>) -> String {
    let mut pacman_search_command = Command::new("pacman");
    pacman_search_command.arg("-Ss");
    for name in package_names {
        pacman_search_command.arg(name);
    }

    let pacman_output = pacman_search_command.output().expect("Couldn't run pacman!").stdout;
    String::from_utf8(pacman_output).unwrap()
}

fn get_aur_results<'a>(package_names: &HashSet<&'a str>) -> &'static str {
    "Bazinga"
}
