mod error;
mod registry;

use std::{fs::{self, read_dir}, path::PathBuf};
use clap::{Parser, Subcommand};

use crate::{error::{GemError, Result}, registry::Registry};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: String,
        path: PathBuf,

        #[clap(long, short, action)]
        force: bool,
    },
    Remove {
        name: String,
    },
    Save {
        game: String,
        save: String,

        #[clap(long, short, action)]
        force: bool,
    },
    Load {
        game: String,
        save: String,

        #[clap(long, short, action)]
        no_restore: bool,
    },
    Delete {
        game: String,
        save: String,
    },
    List {
        game: Option<String>,
    },
}

pub fn root_dir() -> Result<PathBuf> {
    match std::env::home_dir() {
        Some(h) => return Ok(h.join(".local/share/gemsave/")),
        None => return Err(GemError::HomeNotFound),
    }
}

fn game_dir(name: &str) -> Result<PathBuf> {
    Ok(root_dir()?.join(name))
}

fn save_path(game: &str, save: &str) -> Result<PathBuf> {
    Ok(game_dir(game)?.join(format!("{}.save", save)))
}

fn prompt(text: &str) -> Result<bool> { 
    println!("{text}");
    println!("y/n: ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let confirmation = input.trim().to_lowercase();
    Ok(confirmation == "y" || confirmation == "yes")
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Add { name, path, force } => add(&name, path, force),
        Commands::Remove { name } => remove(&name),
        Commands::Save { game, save, force } => crate::save(&game, &save, force),
        Commands::Load { game, save, no_restore } => load(&game, &save, !no_restore),
        Commands::Delete { game, save } => delete(&game, &save),
        Commands::List { game } => match game {
            Some(g) => save_list(&g),
            None => game_list(),
        },
    }
}


fn add(
    game: &str, 
    target: PathBuf, 
    force: bool,
) -> Result<()> {
    let mut registry = Registry::load()?;

    if registry.contains(game) {
        if !force && !prompt("Game already exists. Overwrite?")? {
            return Ok(())
        }
    }

    registry.insert(game.to_owned(), target)?;
    registry.save()?;
    
    Ok(())
}

fn remove(game: &str) -> Result<()> {
    let mut registry = Registry::load()?;

    if !registry.contains(game) {
        return Err(GemError::GameNotFound);
    }

    registry.remove(game)?;
    registry.save()?;
    
    Ok(())
}

fn save(
    game: &str, 
    save: &str, 
    force: bool
) -> Result<()> {
    let registry = Registry::load()?;
    let src = registry.get(game).ok_or(GemError::GameNotFound)?;
    let dst = save_path(game, save)?;

    if dst.exists() {
        if !force && !prompt("Save already exists. Overwrite?\ny/n: ")? {
            return Ok(())
        }
    }

    fs::copy(src, dst)?;

    Ok(())
}

fn load(
    game: &str, 
    save: &str,
    restore: bool,
) -> Result<()> {
    let registry = Registry::load()?;
    let mut src = game_dir(game)?;
    if !src.exists() {
        return Err(GemError::GameNotFound);
    }
    src.push(format!("{}.save", save));
    if !src.exists() {
        return Err(GemError::SaveNotFound)
    }
    let dst = registry.get(game).ok_or(GemError::GameNotFound)?;

    if dst.exists() && restore {
        crate::save(game, "restore", true)?;
    }

    fs::copy(src, dst)?;

    Ok(())
}

fn delete(game: &str, save: &str) -> Result<()> {
    let mut path = game_dir(game)?;
    if !path.exists() {
        return Err(GemError::GameNotFound);
    }

    path.push(format!("{}.save", save));
    if !path.exists() {
        return Err(GemError::SaveNotFound)
    }

    fs::remove_file(&path)?;

    Ok(())
}

fn game_list() -> Result<()> {
    for entry in read_dir(root_dir()?)? {
        let entry = entry?;

        // Skip Registry
        if entry.file_name() == "games.json" {
            continue;
        }

        println!("{}", entry.file_name().display());
    }

    Ok(())
}

fn save_list(game: &str) -> Result<()> {
    for entry in read_dir(game_dir(game)?)? {
        let entry = entry?;

        println!("{}", entry.file_name().display());
    }

    Ok(())
}