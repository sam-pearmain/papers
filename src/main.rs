use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands, 
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialises a .papers file for bibliography tracking
    Init,
    /// Adds entries from a .bib file to the current tracked bibliography
    Add { bibfile: PathBuf },
    /// Removes a specific citation corresponding to the given citekey 
    Remove { citekey: String }, 
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Init => todo!(), 
        Commands::Add { bibfile } => todo!(), 
        Commands::Remove { citekey } => todo!(), 
    }
}
