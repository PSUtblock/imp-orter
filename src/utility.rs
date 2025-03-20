use rfd::FileDialog;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Function to get a folder path. Used to get the mod directory
/// #Arguments
/// - none
///
/// Prompts user for path using FileDialog
/// #Returns
/// - String of path
/// - if canceled or an error - returns an empty string
pub fn get_folder_path() -> String {
    let folder_path: Option<PathBuf> = FileDialog::new().pick_folder();
    println!("DEBUG - Selected folder path: {:?}", folder_path);
    folder_path
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Function to get the GZDoom Executable Path Only
/// #Arguments
/// - none
///
/// Prompts user for path using fileDialog
/// #Returns
/// - String to the executable path
/// - if canceled or an error occurs - returns an error message
pub fn select_executable() -> String {
    FileDialog::new()
        .add_filter("Executable", &["exe", "sh", "bin", "command", "app"]) // Supports Windows, macOS, Linux
        .pick_file()
        .map(|path| {
            let path_str = path.to_string_lossy().into_owned();

            // Extract file name for comparison
            if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                if file_name == "GZDoom" || file_name == "GZDoom.exe" || file_name == "GZDoom.app" {
                    println!("DEBUG - Selected executable is {}", path_str);
                    return path_str; // Return path if it's GZDoom
                }
            }

            // If not GZDoom, return a message instead
            "The executable selected is not GZDoom".to_string()
        })
        .unwrap_or_else(|| "No executable selected.".to_string()) // Handle cancel case
}

/// Function to get the DOOM II wad file
/// #Arguments
/// - none
///
/// Prompts user for path using fileDialog
/// #Returns
/// - String to the executable path
/// - if canceled or an error occurs - returns an error message
pub fn get_wad_path() -> String {
    let file = FileDialog::new()
        .add_filter("WAD File", &["wad"]) // Supports Windows, macOS, Linux
        .pick_file();

    println!("Selected file: {:?}", file);

    file.map(|path| {
        let path_str = path.to_string_lossy().into_owned();
        return path_str;
    })
    .unwrap_or_else(|| "No executable selected.".to_string()) // Handle cancel case
}

/// Function to retrieve a list of wad files in the mods folder
/// #Arguments
/// - takes a string reference
///
/// #Returns
/// - HashMap of the file names and their related paths as Strings
/// - empty if no files exist.
pub fn get_wad_files_in_folder(folder_path: &str) -> HashMap<String, String> {
    let mut file_map = HashMap::new();

    if folder_path == "empty" || folder_path.is_empty() {
        return file_map;
    }

    // looping through the directories entries, determining if they're files and if they are wad files
    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if file_name.to_lowercase().ends_with(".wad") {
                            if let Some(file_path) = entry.path().to_str() {
                                file_map.insert(file_name.to_string(), file_path.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    file_map
}
