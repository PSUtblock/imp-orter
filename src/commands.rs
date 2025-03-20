use rfd::FileDialog;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use zip::read::ZipArchive;

///Function to build a command to execute the GZDoom Launcher
///
/// #Arguments
/// - path to gzdoom launcher as a string reference
/// - path to the doom II wad as a string reference
///
/// #Returns
/// - Error messages if paths are not defined
/// - Executes the application and begins to run GZDoom
///
/// Imp-orter continues to run while playing
pub fn launch_gzdoom(gzdoom_path: &str, wad_path: &str) -> std::io::Result<()> {
    if !Path::new(gzdoom_path).exists() {
        eprintln!("Error: GZDoom executable not found at '{}'", gzdoom_path);
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "GZDoom not found",
        ));
    }

    // check path existences
    if !Path::new(wad_path).exists() {
        eprintln!("Error: WAD file not found at '{}'", wad_path);
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "WAD file not found",
        ));
    }

    let mut command = if cfg!(target_os = "macos") {
        // macOS: Use "open -a" to launch GZDoom
        let mut cmd = Command::new("open");
        cmd.arg("-a")
            .arg(gzdoom_path)
            .arg("--args")
            .arg("-iwad")
            .arg(wad_path);
        cmd
    } else if cfg!(target_os = "windows") {
        // Windows: execute the .exe file
        let mut cmd = Command::new(gzdoom_path);
        cmd.arg("-iwad").arg(wad_path);
        cmd
    } else {
        // Linux: execute the binary
        let mut cmd = Command::new(gzdoom_path);
        cmd.arg("-iwad").arg(wad_path);
        cmd
    };

    command.spawn()?; // Spawn without waiting
    Ok(())
}

///Function to build a command to execute the GZDoom Launcher with a chosen mod
///
/// #Arguments
/// - path to gzdoom launcher as a string reference
/// - path to the doom II wad as a string reference
/// - path to a selected wad from the mod file directory
///
/// #Returns
/// - Error messages if paths are not defined
/// - Executes the application and begins to run GZDoom
///
/// Imp-orter continues to run while playing.
pub fn launch_gzdoom_with_mod(
    gzdoom_path: &str,
    wad_path: &str,
    mod_path: &str,
) -> std::io::Result<()> {

    // check path existences
    if !Path::new(gzdoom_path).exists() {
        eprintln!("Error: GZDoom executable not found at '{}'", gzdoom_path);
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "GZDoom not found",
        ));
    }

    if !Path::new(wad_path).exists() {
        eprintln!("Error: WAD file not found at '{}'", wad_path);
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "WAD file not found",
        ));
    }

    if !Path::new(mod_path).exists() {
        eprintln!("Error: Mod file not found at '{}'", mod_path);
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Mod file not found",
        ));
    }

    let mut command = if cfg!(target_os = "macos") {
        // macOS: Use "open -a" to launch GZDoom
        let mut cmd = Command::new("open");
        cmd.arg("-a")
            .arg(gzdoom_path)
            .arg("--args")
            .arg("-iwad")
            .arg(wad_path)
            .arg("-file")
            .arg(mod_path);
        cmd
    } else if cfg!(target_os = "windows") {
        // Windows: execute the .exe file
        let mut cmd = Command::new(gzdoom_path);
        cmd.arg("-iwad").arg(wad_path).arg("-file").arg(mod_path);
        cmd
    } else {
        // Linux: execute the binary
        let mut cmd = Command::new(gzdoom_path);
        cmd.arg("-iwad").arg(wad_path).arg("-file").arg(mod_path);
        cmd
    };

    command.spawn()?;
    Ok(())
}

///Function to select a zip file and extract a wad from it and move the file to the mods directory
///
/// #Arguments
/// - path to the mods directory
///
/// #Returns
/// - Error messages if no zip is selected, no wad file exists, and if it already exists in mod directory
/// - extracted .wad file is moved to the mods directory
pub fn extract_and_move_wad(target_dir: &str) -> std::io::Result<()> {
    // File Dialog to find zip file
    let zip_path = FileDialog::new()
        .add_filter("ZIP Files", &["zip"]) // Allow only .zip files
        .pick_file();

    // Check if the user selected a file
    let zip_path = match zip_path {
        Some(path) => path,
        None => {
            println!("No ZIP file selected.");
            return Ok(());
        }
    };

    println!("Selected ZIP file: {:?}", zip_path);

    // Open the ZIP file
    let file = File::open(&zip_path)?;
    let mut archive = ZipArchive::new(BufReader::new(file))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_string();

        if file_name.to_lowercase().ends_with(".wad") {
            let final_path = Path::new(target_dir).join(&file_name);

            //Check if the file already exists
            if final_path.exists() {
                println!("File {} already exists in {}", file_name, target_dir);
                return Ok(()); // Exit without overwriting
            }

            println!("Extracting: {}", file_name);
            let mut outfile = File::create(&final_path)?;
            std::io::copy(&mut file, &mut outfile)?;

            // Set file permissions on Linux/macOS
            #[cfg(target_family = "unix")]
            fs::set_permissions(&final_path, fs::Permissions::from_mode(0o644))?;

            println!("Extracted {} to {}", file_name, final_path.display());
            return Ok(()); // Exit after extracting the first WAD
        }
    }

    println!("No .wad file found in the ZIP.");
    Ok(())
}
