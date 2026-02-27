use clap::{Parser, Subcommand};

mod pathing;
mod commands;
use crate::{commands::{handle_list, handle_load, handle_remove, handle_save}, error::BalSaveError};

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
        #[arg(short, long, help = "Overwrite without confirmation")]
        force: bool,
    },
    /// Load NAME into Balatro slot. Format: balsave load [NAME] [SLOT] (SLOT optional, defaults to 1)
    Load {
        name: String,
        slot: Option<String>,
        #[arg(short, long, help = "Overwrite without confirmation")]
        force: bool,
    },
    /// Shows all the files in the save directory. Format: balsave list
    List,
    /// Removes save. Format: balsave remove [NAME]
    Remove {
        name: String,
        #[arg(short, long, help = "Overwrite without confirmation")]
        force: bool,
    },
}

fn main() -> Result<(), BalSaveError> {
    let args = Args::parse();

    match args.command {
        Commands::Save { name, slot, force } => handle_save(name, slot, force),
        Commands::Load { name, slot, force } => handle_load(name, slot, force),
        Commands::List => handle_list(),
        Commands::Remove { name, force } => handle_remove(name, force),
    }
}