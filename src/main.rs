use std::{fs, io::Write};
use clap::{Parser, Subcommand};

mod pathing;
use crate::{error::BalSaveError, pathing::*};

mod error;

#[derive(Parser, Debug)]
#[command(version, about = "Balatro Save File Manager (balsave)", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Save Balatro slot to NAME. Format: balsave save [NAME] [SLOT] (SLOT optional, defaults to 1)
    Save {
        name: String,
        slot: Option<String>,
    },
    /// Load NAME into Balatro slot. Format: balsave load [NAME] [SLOT] (SLOT optional, defaults to 1)
    Load {
        name: String,
        slot: Option<String>,
    },
    /// Shows all the files in the save directory. Format: balsave list
    List,
    /// Removes save. Format: balsave remove [NAME]
    Remove {
        name: String
    },
}

fn main() -> Result<(), BalSaveError> {
    let args = Args::parse();

    match args.command {
        Commands::Save { name, slot } => handle_save(name, slot),
        Commands::Load { name, slot } => handle_load(name, slot),
        Commands::List => handle_list(),
        Commands::Remove { name } => handle_remove(name),
    }
}

fn handle_save(name: String, slot: Option<String>) -> Result<(), BalSaveError> {
    let slot = if let Some(s) = slot {
        s.parse::<u8>()
        .map_err(|_| BalSaveError::Validation("Invalid slot number: must be 1, 2, or 3.".into()))?
    } else {
        1
    };


    let source_path = slot_path(slot)?;
    let dest_path = saves_path()?.join(&name);

    if !source_path.exists() {
        return Err(BalSaveError::Validation(format!(
            "Balatro save file not found for Slot {}. Expected file: {}", 
            slot, 
            source_path.display()
        )));
    }

    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| BalSaveError::Io(std::io::Error::new(e.kind(), format!("Failed to create destination directory: {}", parent.display()))))?;
    }

    if dest_path.exists() {
        if !confirm_action(&format!("⚠️ WARNING: Save file '{}' already exists. Overwrite? (y/N): ", name)).unwrap_or(false) {
            return Err(BalSaveError::UserCancellation);
        }
    }

    fs::copy(&source_path, &dest_path)?;

    println!("\n✅ Successfully SAVED Balatro Slot {} as {}", slot, &name);
    Ok(())
}

fn handle_load(name: String, slot: Option<String>) -> Result<(), BalSaveError> {
    let slot = if let Some(s) = slot {
        s.parse::<u8>()
        .map_err(|_| BalSaveError::Validation("Invalid slot number: must be 1, 2, or 3.".into()))?
    } else {
        1
    };

    let source_path = saves_path()?.join(&name);
    let dest_path = slot_path(slot)?;

    if !source_path.exists() {
        return Err(BalSaveError::Validation(format!(
            "Custom save file not found at path: {}", 
            source_path.display()
        )));
    }

    if !confirm_action(&format!("⚠️ WARNING: Loading '{}' will OVERWRITE Balatro Slot {}. Proceed? (y/N): ", name, slot)).unwrap_or(false) {
        return Err(BalSaveError::UserCancellation);
    }
    
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| BalSaveError::Io(std::io::Error::new(e.kind(), format!("Failed to create destination directory: {}", parent.display()))))?;
    }
    
    fs::copy(&source_path, &dest_path)?;

    println!("\n✅ Successfully LOADED {} into Balatro Slot {}", &name, slot);
    Ok(())
}

fn handle_list() -> Result<(), BalSaveError> {
    let saves_dir = saves_path()?;

    if !saves_dir.exists() {
        println!("📂 Save directory not found: {}", saves_dir.display());
        println!("No custom saves have been created yet.");
        return Ok(());
    }

    println!("\n📂 Saves");
    println!("--------------------------------------");

    let mut found_saves = false;

    for entry in fs::read_dir(saves_dir)? {
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
        println!("(No custom save files found in the directory.)");
    }

    println!("--------------------------------------");
    Ok(())
}
fn handle_remove(name: String) -> Result<(), BalSaveError> {
    let save_path = saves_path()?.join(&name);

    if !save_path.exists() {
        return Err(BalSaveError::Validation(format!(
            "Custom save file to remove not found: {}",
            save_path.display()
        )));
    }

    if !confirm_action(&format!("⚠️ WARNING: Permanently delete save file '{}'? (y/N): ", name)).unwrap_or(false) {
        return Err(BalSaveError::UserCancellation);
    }

    fs::remove_file(&save_path)?;

    println!("\n🗑️ Successfully REMOVED save file: {}", &name);
    Ok(())
}

fn confirm_action(prompt: &str) -> Result<bool, BalSaveError> {
    print!("{}", prompt);
    std::io::stdout().flush()?;
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let confirmation = input.trim().to_lowercase();

    Ok(confirmation == "y" || confirmation == "yes")
}