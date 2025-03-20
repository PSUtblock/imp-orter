slint::include_modules!();
use slint::{Model, ModelRc, SharedString, VecModel};

mod commands;
mod config_functions;
mod utility;

/// Main function for imp-orter
/// Initializes the Slint UI, loads configuration settings, and sets up event callbacks.
fn main() {

    let description = include_str!("../documents/description.txt");

    println!("DEBUG - Config File Path: {:?}", config_functions::get_config_path());

    // import the Slint ui menu
    let app = MenuApp::new().unwrap();
    app.set_description(description.into());

    let config = config_functions::read_config("documents/config.txt");

    let gzdoom_path = config.get("gzDoom_Path").cloned().unwrap_or_default();
    let wad_path = config.get("wad_Path").cloned().unwrap_or_default();
    let mods_directory = config.get("mods_Directory").cloned().unwrap_or_default();

    app.set_launcherPath(SharedString::from(
        config.get("gzDoom_Path").cloned().unwrap_or_default(),
    ));
    app.set_wadFile(SharedString::from(
        config.get("wad_Path").cloned().unwrap_or_default(),
    ));
    app.set_modDirectory(SharedString::from(
        config.get("mods_Directory").cloned().unwrap_or_default(),
    ));

    println!("DEBUG -- gzDoom_Path: {}", gzdoom_path);
    println!("DEBUG -- wad_Path: {}", wad_path);
    println!("DEBUG -- mods_Directory: {}", mods_directory);

    // retrieve wad file paths in the mods folder
    let mod_map = utility::get_wad_files_in_folder(&mods_directory);

    println!("DEBUG - Mod Mappings: {:?}", mod_map);

    // Converts the Hashmap into a Vector of ModFiles (Modfile is a struct defined in Slint)
    let mod_files: Vec<ModFile> = mod_map
        .into_iter()
        .map(|(name, path)| ModFile {
            name: SharedString::from(name),
            path: SharedString::from(path),
        })
        .collect();

    println!("DEBUG - Vectorized Mod Mappings: {:?}", mod_files);

    let model_rc = ModelRc::new(VecModel::from(mod_files));
    println!("DEBUG - ModelRc content: {:?}", model_rc.row_count());
    app.set_mod_files(model_rc);

    //Callback for getting the GZ Doom Launcher
    app.on_getGzDoomLauncher({
        let app_handle = app.clone_strong();
        move || {
            let exe_path = SharedString::from(utility::select_executable());

            app_handle.set_launcherPath(exe_path.clone()); // Pass a cloned value to UI

            let config_path = std::env::current_dir()
                .unwrap()
                .join("documents/config.txt");
            config_functions::update_gzdoom_path(config_path.to_str().unwrap(), exe_path.as_str());
            // Use as &str
        }
    });

    //Callback for getting the Path to the DOOM 2 WAD
    app.on_getWadPath({
        let app_handle = app.clone_strong();
        move || {
            let file_path = SharedString::from(utility::get_wad_path());

            app_handle.set_wadFile(file_path.clone());

            let config_path = std::env::current_dir()
                .unwrap()
                .join("documents/config.txt");
            config_functions::update_wad_path(config_path.to_str().unwrap(), file_path.as_str());
            // Use as &str
        }
    });

    //Callback for getting the modsDirectory
    // - needs a clone of the mods directory to be move into the function and update the directory listing
    let mod_d = mods_directory.clone();
    app.on_getModDirectory({
        let app_handle = app.clone_strong();
        move || {
            let folder_path = SharedString::from(utility::get_folder_path());

            app_handle.set_modDirectory(folder_path.clone());

            let config_path = std::env::current_dir()
                .unwrap()
                .join("documents/config.txt");
            config_functions::update_wad_directory(
                config_path.to_str().unwrap(),
                folder_path.as_str(),
            );

            let mod_map = utility::get_wad_files_in_folder(&mod_d);
            let mod_files: Vec<ModFile> = mod_map
                .into_iter()
                .map(|(name, path)| ModFile {
                    name: SharedString::from(name),
                    path: SharedString::from(path),
                })
                .collect();


            let model_rc = ModelRc::new(VecModel::from(mod_files));
            app_handle.set_mod_files(model_rc);
        }
    });

    // Callback to start GZDoom with no mods
    app.on_playVanilla({
        let app_handle = app.clone_strong();
        move || {
            println!("Launching Vanilla");
            let launcher = app_handle.get_launcherPath();
            let wad = app_handle.get_wadFile();

            match commands::launch_gzdoom(&launcher, &wad) {
                Ok(_) => println!("GZDoom launched successfully."),
                Err(e) => eprintln!("Failed to launch GZDoom: {}", e),
            }
        }
    });

    // Callback for starting GZDoom with a selected mod
    app.on_playMod({
        let app_handle = app.clone_strong();
        move || {
            println!("Launching Mod");
            let launcher = app_handle.get_launcherPath();
            let wad = app_handle.get_wadFile();
            let mod_file = app_handle.get_selectedWad();

            match commands::launch_gzdoom_with_mod(&launcher, &wad, &mod_file) {
                Ok(_) => println!("GZDoom launched successfully."),
                Err(e) => eprintln!("Failed to launch GZDoom: {}", e),
            }
        }
    });

    // Callback for Importing new WADS from a zip folder
    app.on_importWad(move || {
        println!("Import WAD");

        let value = mods_directory.clone();
        std::thread::spawn(move || {
            // Run in a separate thread to avoid blocking UI
            if let Err(e) = commands::extract_and_move_wad(&value) {
                eprintln!("Failed to extract WAD: {}", e);
            } else {
                println!("WAD extracted successfully.");
            }
        });
    });

    app.run().unwrap();
}
