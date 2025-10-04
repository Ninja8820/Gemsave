use std::{ env, io::{self, ErrorKind}, path::PathBuf };

use crate::error::BalSaveError;

const BALATRO_SAVE_DIR_BASE: &str = "Balatro"; 
const BALATRO_STEAM_APP_ID: &str = "2379780"; 

fn proton_path() -> Result<PathBuf, io::Error> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| io::Error::new(
            ErrorKind::NotFound, 
            "Could not determine user home directory."
        ))?;
    
    let mut proton_path = home_dir.join(".local/share/Steam/steamapps/compatdata");
    
    proton_path.push(BALATRO_STEAM_APP_ID);
    proton_path.push("pfx/drive_c/users/steamuser/AppData/Roaming");
    proton_path.push(BALATRO_SAVE_DIR_BASE);

    Ok(proton_path)
}
/// * `Linux` - ~/.local/share/Steam/steamapps/compatdata/.../pfx/drive_c/users/steamuser/AppData/Roaming/Balatro
/// * `Windows` - C:\Users\...\AppData\Roaming/Balatro
pub fn base_path() -> Result<PathBuf, io::Error> {
    if let Some(base_dir) = dirs::config_dir() {
        let native_path = base_dir.join(BALATRO_SAVE_DIR_BASE);
        if native_path.exists() {
            return Ok(native_path);
        }
    }

    if env::consts::OS == "linux" {
        if let Ok(proton_path) = proton_path() {
            if proton_path.exists() {
                return Ok(proton_path);
            }
        }
    }

    Err(io::Error::new(
        ErrorKind::NotFound, 
        "Balatro save directory not found. Please ensure Balatro has been run at least once."
    ))
}
/// * `Linux` - ~/.local/share/Steam/steamapps/compatdata/.../pfx/drive_c/users/steamuser/AppData/Roaming/Balatro/balsave
/// * `Windows` - C:\Users\...\AppData\Roaming/Balatro/balsave
pub fn saves_path() -> Result<PathBuf, io::Error> {
    let path = base_path()?;
    Ok(path.join("balsave"))
}

/// * `Linux` - ~/.local/share/Steam/steamapps/compatdata/.../pfx/drive_c/users/steamuser/AppData/Roaming/Balatro/balsave/{SLOT}
/// * `Windows` - C:\Users\...\AppData\Roaming/Balatro/{SLOT}
pub fn slot_path(slot: u8) -> Result<PathBuf, BalSaveError> {
    if slot < 1 || slot > 3 {
        return Err(BalSaveError::Validation("Slot must be 1, 2 or 3".into()));
    }
    let base_path = base_path()
        .map_err(|e| BalSaveError::Path(format!("Could not determine base path: {}", e)))?;
    
    let slot_folder = slot.to_string();
    let file_name = "save.jkr"; 
    
    Ok(base_path.join(slot_folder).join(file_name))
}