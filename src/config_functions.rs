use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

/// Function to Check Project Directory for documents/config.txt
///
/// #Arguments
///  - none
///
/// #Returns
/// - PathBuf
///
/// #To-do
/// - Figure out how to not rely on this when packaging for a release.
pub fn get_config_path() -> PathBuf {
    let project_root = std::env::current_dir().expect("Failed to get current directory");
    project_root.join("documents/config.txt")
}

/// Function to read the config.txt files and extract the parameters
///
/// #Arguments
///  - path to file as a string reference
///
/// #Returns
/// - Hashmap of the parameters that were stored
pub fn read_config(_file_path: &str) -> HashMap<String, String> {
    let content = check_config_file(); // Ensure config file exists before reading

    content
        .lines()
        .filter_map(|line| {
            let mut parts = line.splitn(2, '=');
            Some((
                parts.next()?.trim().to_string(),
                parts.next()?.trim().to_string(),
            ))
        })
        .collect()
}

/// Checks the contents of config file and creates a new config.txt if one does not exist
///
/// #Arguments
/// - none
///
/// #Returns
/// - String of the config.txt file
pub fn check_config_file() -> String {
    let config_path = get_config_path();

    // If the config file exists, read and return its content
    if config_path.exists() {
        let mut file = File::open(&config_path).expect("Failed to open config file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read config file");
        return contents;
    }

    // Ensure the parent directory exists
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create documents directory");
    }

    // Default content for config.txt
    let default_config = "\
gzDoom_Path = empty
wad_Path = empty
mods_Directory = empty";

    // Create and write default values to config.txt
    let mut file = File::create(&config_path).expect("Failed to create config file");
    file.write_all(default_config.as_bytes())
        .expect("Failed to write to config file");

    default_config.to_string()
}


/// Updates the gzDoom executable path
///
/// #Arguments
/// - path to config.txt as a string reference
/// - new path to gzDoom as a string reference
///
/// #Returns
/// - nothing
pub fn update_gzdoom_path(config_path: &str, new_path: &str) {
    let config = check_config_file();

    let mut updated_lines: Vec<String> = Vec::new();
    for line in config.lines() {
        if line.starts_with("gzDoom_Path") {
            updated_lines.push(format!("gzDoom_Path = {}", new_path));
        } else {
            updated_lines.push(line.to_string());
        }
    }

    let new_config = updated_lines.join("\n");

    let mut file = File::create(config_path).expect("Failed to open config file for writing");
    file.write_all(new_config.as_bytes())
        .expect("Failed to write updated config");

    println!("gzDoom_Path updated successfully.");
}

/// Updates the Doom II wad path
///
/// #Arguments
/// - path to config.txt as a string reference
/// - new path to Doom II wad as a string reference
///
/// #Returns
/// - nothing
pub fn update_wad_path(config_path: &str, new_path: &str) {
    let config = check_config_file();

    let mut updated_lines: Vec<String> = Vec::new();
    for line in config.lines() {
        if line.starts_with("wad_Path") {
            updated_lines.push(format!("wad_Path = {}", new_path));
        } else {
            updated_lines.push(line.to_string());
        }
    }

    let new_config = updated_lines.join("\n");

    let mut file = File::create(config_path).expect("Failed to open config file for writing");
    file.write_all(new_config.as_bytes())
        .expect("Failed to write updated config");

    println!("Wad file updated successfully.");
}

/// Updates the mod directory path where users store their mod wads.
///
/// #Arguments
/// - path to mod directory as a string reference
/// - new mod directory path as a string reference
///
/// #Returns
/// - nothing
pub fn update_wad_directory(config_path: &str, new_path: &str) {
    let config = check_config_file();

    let mut updated_lines: Vec<String> = Vec::new();
    for line in config.lines() {
        if line.starts_with("mods_Directory") {
            updated_lines.push(format!("mods_Directory = {}", new_path));
        } else {
            updated_lines.push(line.to_string());
        }
    }

    let new_config = updated_lines.join("\n");

    let mut file = File::create(config_path).expect("Failed to open config file for writing");
    file.write_all(new_config.as_bytes())
        .expect("Failed to write updated config");

    println!("Wad Directory updated successfully.");
}
