use clap::{Parser, Subcommand};
use std::{fs, io::Write};

use crate::{error::BalSaveError, pathing::{saves_path, slot_path}};

#[derive(Parser, Debug)]
#[command(version, about = "Balatro Save File Manager (balsave)", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Save Balatro slot to NAME. Format: balsave save NAME [SLOT] (SLOT optional, defaults to 1)
    Save {
        name: String,
        slot: Option<String>,
        #[arg(short, long, help = "Overwrite without confirmation")]
        force: bool,
    },
    /// Load NAME into Balatro slot. Format: balsave load NAME [SLOT] (SLOT optional, defaults to 1)
    Load {
        name: String,
        slot: Option<String>,
        #[arg(short, long, help = "Overwrite without confirmation")]
        force: bool,
    },
    /// Shows all the files in the save directory. Format: balsave list
    List,
    /// Removes save. Format: balsave remove NAME
    Remove {
        name: String,
        #[arg(short, long, help = "Overwrite without confirmation")]
        force: bool,
    },
}
fn parse_slot(slot: Option<String>) -> Result<u8, BalSaveError> {
    return if let Some(s) = slot {
        let value = s.parse::<u8>()
            .map_err(|_| BalSaveError::Validation("Invalid slot: must be a number".into()))?;

        if ![1, 2, 3].contains(&value) {
            return Err(BalSaveError::Validation("Invalid slot number: must be 1, 2 or 3".into()));
        }

        Ok(value)
    } else {
        Ok(1)
    };
}

/// Returns: **True** - Continue, **False** - Cancel
fn confirm_action(prompt: &str) -> Result<bool, BalSaveError> {
    print!("{}", prompt);
    std::io::stdout().flush()?;
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let confirmation = input.trim().to_lowercase();
    println!();

    Ok(confirmation == "y" || confirmation == "yes")
}

pub fn handle_save(name: String, slot: Option<String>, force: bool) -> Result<(), BalSaveError> {
    let slot = parse_slot(slot)?;

    let source_path = slot_path(slot)?;
    if !source_path.exists() {
        return Err(BalSaveError::Validation(format!(
            "Balatro save file not found for Slot {}. Expected file: {}", 
            slot, 
            source_path.display()
        )));
    }

    let dest_dir = saves_path()?;
    if !dest_dir.exists() {
        fs::create_dir_all(&dest_dir)
            .map_err(|e| BalSaveError::Io(std::io::Error::new(e.kind(), format!("Failed to create destination directory: {}", &dest_dir.display()))))?;
    }

    let dest_path = dest_dir.join(&name);

    if dest_path.exists() && !force {
        if !confirm_action(&format!("WARNING: Save file '{}' already exists. Overwrite? (y/N): ", name)).unwrap_or(false) {
            return Err(BalSaveError::UserCancellation);
        }
    }

    fs::copy(&source_path, &dest_path)?;

    println!("Successfully SAVED Balatro Slot {} as {}", slot, &name);
    Ok(())
}


pub fn handle_load(name: String, slot: Option<String>, force: bool) -> Result<(), BalSaveError> {
    let slot = parse_slot(slot)?;

    let source_path = saves_path()?.join(&name);
    if !source_path.exists() {
        return Err(BalSaveError::Validation(format!(
            "Save file not found at path: {}", 
            source_path.display()
        )));
    }

    let dest_path = slot_path(slot)?;

    if dest_path.exists() && !force {
        if !confirm_action(&format!("WARNING: Loading '{}' will OVERWRITE Balatro Slot {}. Proceed? (y/N): ", name, slot)).unwrap_or(false) {
            return Err(BalSaveError::UserCancellation);
        }
    }

    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| BalSaveError::Io(std::io::Error::new(e.kind(), format!("Failed to create destination directory: {}", parent.display()))))?;
    }
    
    fs::copy(&source_path, &dest_path)?;

    println!("Successfully LOADED {} into Balatro Slot {}", &name, slot);
    Ok(())
}


pub fn handle_list() -> Result<(), BalSaveError> {
    let saves_dir = saves_path()?;

    if !saves_dir.exists() {
        println!("Save directory not found: {}", saves_dir.display());
        return Ok(());
    }

    println!("Saves");
    println!("--------------------------------------");

    let mut found_saves = false;

    for entry in saves_dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                println!("- {}", file_name);
                found_saves = true;
            }
        }
    }

    if !found_saves {
        println!("(No save files found.)");
    }

    println!("--------------------------------------");
    Ok(())
}


pub fn handle_remove(name: String, force: bool) -> Result<(), BalSaveError> {
    let path = saves_path()?.join(&name);

    if !path.exists() {
        return Err(BalSaveError::Validation(format!(
            "Save file to remove not found: {}",
            path.display()
        )));
    }

    if !force {
        if !confirm_action(&format!("WARNING: Permanently delete save file '{}'? (y/N): ", name)).unwrap_or(false) {
            return Err(BalSaveError::UserCancellation);
        }
    }

    fs::remove_file(&path)?;

    println!("Successfully REMOVED save file: {}", &name);
    Ok(())
}